use std::sync::Arc;
use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use colored::{Colorize, Color};
use inquire::{Confirm, Text, Select, CustomType};

use crate::state::AppState;
use crate::engine::{dispatch, IzParser};
use crate::models::{
    order::{TradeSource, OrderAction},
    AccountScope, Account, ApiClient, AccountType, OrderRequest
};

/// iZ-Console: Hệ điều hành dòng lệnh cho Trading (v2.5 Hermes Edition)
pub struct IzConsole {
    state: Arc<AppState>,
    current_account_id: Option<i64>,
}

impl IzConsole {
    pub fn new(state: Arc<AppState>) -> Self {
        Self {
            state,
            current_account_id: None,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut rl = DefaultEditor::new()?;
        
        self.print_banner();
        self.show_dashboard().await?;

        loop {
            let prompt = match self.current_account_id {
                Some(id) => format!("{} {} ", "iz".dimmed(), format!("[{}] >", id).cyan().bold()),
                None => format!("{} ", "iz >".green().bold()),
            };

            let readline = rl.readline(&prompt);
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() { continue; }
                    rl.add_history_entry(line)?;

                    if !self.handle_input(line).await? {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("{} {:?}", "Error:".red(), err);
                    break;
                }
            }
        }
        Ok(())
    }

    fn print_banner(&self) {
        println!("{}", r#"
   _ ________               __   
  (_)__  / /__________ ____/ /__ 
 / /  / / __/ ___/ __ `/ __  / _ \
/ /  / / /_/ /  / /_/ / /_/ /  __/
/_/  /_/\__/_/   \__,_/\__,_/\___/ v2.5
"#.cyan().bold());
        println!("{} App Scope ({}) | Account Scope ({})", "Structure:".dimmed(), "iz >".green(), "iz[id] >".cyan());
        println!("Type {} for commands, {} to exit.", "help".yellow(), "exit".red());
        println!();
    }

    async fn show_dashboard(&self) -> Result<()> {
        let status = self.state.get_system_status().await;
        
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Metric", "Value", "Status"]);

        let uptime = AppState::format_uptime(status.uptime_secs);
        let pnl_color = if status.total_daily_pnl >= 0.0 { Color::Green } else { Color::Red };
        
        table.add_row(vec!["Uptime", &uptime, "ONLINE"]);
        table.add_row(vec![
            "Accounts", 
            &format!("{} Total / {} Connected", status.total_accounts, status.connected_accounts),
            if status.active_accounts > 0 { "AUTO ON" } else { "AUTO OFF" }
        ]);
        table.add_row(vec![
            "Finance",
            &format!("Balance: ${:.2} | Equity: ${:.2}", status.total_real_balance, status.total_real_equity),
            &format!("PNL: ${:+.2}", status.total_daily_pnl).color(pnl_color).to_string()
        ]);

        println!("\n{}\n{}", "[SYSTEM DASHBOARD]".bold(), table);
        Ok(())
    }

    async fn handle_input(&mut self, line: &str) -> Result<bool> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();

        // ── 1. Navigation & Global Helpers ─────────────────────────────
        match cmd.as_str() {
            "exit" | "quit" => return Ok(false),
            "help" | "h" | "?" => {
                self.show_help();
                return Ok(true);
            }
            "back" | ".." | "app" | "top" => {
                self.current_account_id = None;
                println!("[INFO] Returned to Global Scope");
                return Ok(true);
            }
            "clear" | "cls" => {
                print!("{}[2J{}[1;1H", 27 as char, 27 as char);
                self.print_banner();
                return Ok(true);
            }
            "r" | "refresh" | "dash" => {
                self.show_dashboard().await?;
                return Ok(true);
            }
            _ => {}
        }

        // ── 2. Scope Switching ────────────────────────────────────────
        if let Ok(id) = cmd.parse::<i64>() {
            let accounts = self.state.accounts.read().await;
            if accounts.contains_key(&id) {
                self.current_account_id = Some(id);
                println!("[OK] Switched to Account: {}", id.to_string().cyan().bold());
            } else {
                println!("[ERROR] Account #{} not found.", id);
            }
            return Ok(true);
        }

        // ── 3. Interactive Prompts for Management ─────────────────────
        if cmd == "acc" && parts.len() > 1 && parts[1] == "add" {
            self.interactive_add_account().await?;
            return Ok(true);
        }

        if cmd == "sys" && parts.len() > 1 && parts[1] == "nuclear" {
            self.interactive_nuclear_wipe().await?;
            return Ok(true);
        }

        // ── 4. Standard Command Parser Fallback ────────────────────────
        if let Some(mut req) = crate::engine::IzParser::parse(line, crate::models::TradeSource::Cli) {
            if let Some(id) = self.current_account_id {
                req.account_scope = crate::models::AccountScope::Single;
                req.account_ids = vec![id];
            }
            let res = crate::engine::dispatch(self.state.clone(), req).await?;
            self.print_dispatch_result(res).await;
            return Ok(true);
        }

        // ── 5. AI Agent Fallback (Natural Language) ───────────────────
        {
            let mut agent = self.state.ai_agent.lock().await;
            if !agent.is_loaded() {
                println!("[INFO] Unknown command: {}. Type 'help' for commands, or '/agent on' to enable AI.", line.yellow());
                return Ok(true);
            }

            println!("AI is thinking...");
            let res = agent.process_natural_language(line, self.state.clone()).await;

            match res {
                Ok(Some(req)) => {
                    println!("[AGENT] AI suggested action: {:?}", req.action);
                    let dispatch_res = crate::engine::dispatch(self.state.clone(), req).await?;
                    self.print_dispatch_result(dispatch_res).await;
                }
                Ok(None) => {
                    println!("[AGENT] I'm not sure how to help with that.");
                }
                Err(e) => {
                    println!("[ERROR] AI Agent error: {}", e);
                }
            }
        }

        Ok(true)
    }

    async fn interactive_add_account(&self) -> Result<()> {
        println!("\n[ADD] {}", "Add New cTrader Account".bold());
        
        let id = CustomType::<i64>::new("Account ID:").prompt()?;
        let name = Text::new("Account Name:").prompt()?;
        let token = Text::new("Access Token:").prompt()?;
        let is_real = Select::new("Account Type:", vec!["demo", "real"]).prompt()? == "real";

        let param = format!("{}:{}:{}:{}", id, name, token, if is_real { "real" } else { "demo" });
        let req = OrderRequest::new(TradeSource::Cli, "console".to_string(), OrderAction::AddAccount);
        let mut req = req;
        req.target_id = param;

        let res = dispatch(self.state.clone(), req).await?;
        println!("{}", res.messages.join("\n").green());
        Ok(())
    }

    async fn interactive_nuclear_wipe(&self) -> Result<()> {
        println!("\n[DANGER] {}", "WARNING: NUCLEAR WIPE DETECTED".red().bold());
        let confirm1 = Confirm::new("Are you absolutely sure you want to delete ALL data?").with_default(false).prompt()?;
        
        if confirm1 {
            let confirm2 = Text::new("Type 'CONFIRM' to proceed:").prompt()?;
            if confirm2 == "CONFIRM" {
                let req = OrderRequest::new(TradeSource::Cli, "console".to_string(), OrderAction::SystemNuclearWipe);
                let res = dispatch(self.state.clone(), req).await?;
                println!("{}", res.messages.join("\n").on_red().white().bold());
            } else {
                println!("[CANCEL] Wipe cancelled.");
            }
        } else {
            println!("[CANCEL] Wipe cancelled.");
        }
        Ok(())
    }

    async fn print_dispatch_result(&self, res: crate::engine::DispatchResult) {
        let status_label = if res.error_count == 0 { "[OK]".green() } else { "[!]".yellow() };
        println!("\n{} {}", status_label, "Execution Result".bold());
        println!("----------------------------------------");
        for msg in res.messages {
            println!("  - {}", msg);
        }
        if !res.positions.is_empty() {
            let mut table = Table::new();
            table.set_header(vec!["Acc", "Side", "Symbol", "Vol", "P&L"]);
            for p in res.positions {
                table.add_row(vec![
                    p.account_id.to_string(),
                    p.side,
                    p.symbol,
                    p.volume.to_string(),
                    format!("{:+.2}", p.pnl)
                ]);
            }
            println!("{}", table);
        }
        println!("----------------------------------------");
    }

    fn show_help(&self) {
        println!("\n{}", "[COMMAND REFERENCE] iZ-Console v2.5:".bold().cyan());
        
        let mut table = Table::new();
        table.load_preset(UTF8_FULL).set_header(vec!["Command", "Description", "Scope"]);
        
        table.add_row(vec!["list | l", "List all trading accounts", "Global"]);
        table.add_row(vec!["<ID>", "Enter Account Scope (e.g. 101)", "Global"]);
        table.add_row(vec!["status | r", "Show grouped report / dashboard", "Contextual"]);
        table.add_row(vec!["rb", "Bot Performance Report", "Global"]);
        table.add_row(vec!["acc add", "Interactive account creation", "Global/Admin"]);
        table.add_row(vec!["acc del <id>", "Remove account from DB", "Global/Admin"]);
        table.add_row(vec!["pos | p", "Show grouped open positions", "Contextual"]);
        table.add_row(vec!["/a[N] | /d[N]", "Enable/Disable Bot N", "Shorthand"]);
        table.add_row(vec!["/c[N]", "Disable Bot N & Close Orders", "Shorthand"]);
        table.add_row(vec!["on / off", "Toggle Account Autotrade", "Contextual"]);
        table.add_row(vec!["close", "EMERGENCY CLOSE ALL", "Contextual"]);
        table.add_row(vec!["#BUY <SBL> <VOL>", "Execute direct order", "Contextual"]);
        table.add_row(vec!["sys cleanup", "Clean logs and temp files", "Admin"]);
        table.add_row(vec!["sys nuclear", "WIPE EVERYTHING (CLI only)", "Admin"]);
        table.add_row(vec!["back | top", "Return to Global Scope", "Account"]);

        println!("{}", table);
    }
}
