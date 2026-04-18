use std::sync::Arc;
use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::state::AppState;
use crate::engine::{dispatch, IzParser};
use crate::models::{
    order::{TradeSource, OrderAction},
    AccountScope, Account, ApiClient, AccountType
};

/// iZ-Console: Hệ điều hành dòng lệnh cho Trading (v2.1)
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
        
        println!("🚀 iZ-Console v2.1.0 Started");
        println!("Cấu trúc ngữ cảnh: App Scope (iz >) | Account Scope (iz[acc_id] >)");
        println!("Gõ 'help' để xem lệnh, 'exit' để thoát.");

        loop {
            let prompt = match self.current_account_id {
                Some(id) => format!("iz[{}] > ", id),
                None => "iz > ".to_string(),
            };

            let readline = rl.readline(&prompt);
            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() { continue; }
                    rl.add_history_entry(line)?;

                    if !self.handle_command(line).await? {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn handle_command(&mut self, line: &str) -> Result<bool> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0].to_lowercase();

        // ── 1. Global Navigation Commands ─────────────────────────────
        match cmd.as_str() {
            "exit" | "quit" => return Ok(false),
            "help" | "h" | "?" => {
                self.show_help();
                return Ok(true);
            }
            "back" | ".." | "app" | "top" | "topapp" | "top app" => {
                if self.current_account_id.is_some() {
                    self.current_account_id = None;
                    println!("🏠 Returned to Global Scope (iz >)");
                }
                return Ok(true);
            }
            "clear" | "cls" => {
                print!("{}[2J{}[1;1H", 27 as char, 27 as char);
                return Ok(true);
            }
            _ => {}
        }

        // ── 2. Context-Aware Dispatch ─────────────────────────────────
        if self.current_account_id.is_none() {
            self.handle_global_command(&parts).await?;
        } else {
            self.handle_account_command(&parts).await?;
        }

        Ok(true)
    }

    // ── Global Scope Handlers ────────────────────────────────────────────────

    async fn handle_global_command(&mut self, parts: &[&str]) -> Result<()> {
        let cmd = parts[0].to_lowercase();
        match cmd.as_str() {
            // Switch to Account Scope
            id if id.parse::<i64>().is_ok() => {
                let acc_id = id.parse::<i64>().unwrap();
                let accounts = self.state.accounts.read().await;
                if accounts.contains_key(&acc_id) {
                    self.current_account_id = Some(acc_id);
                    println!("🎯 Switched to Account: {}", acc_id);
                } else {
                    println!("❌ Account #{} not found.", acc_id);
                }
            }

            "l" | "list" => self.list_accounts().await?,
            "r" | "report" => self.show_global_report().await?,
            
            // Global Account Actions
            "a" | "d" | "c" => {
                self.handle_short_action(&cmd).await?;
            }

            // Account Management
            "acc" => self.handle_acc_management(parts).await?,
            
            // API Key Management
            "api" => self.handle_api_management(parts).await?,

            line if line.starts_with('#') => {
                self.execute_hash_command(line).await?;
            }

            "sys" => self.handle_sys_management(parts).await?,

            _ => println!("❓ Unknown command: {}. Type 'help' for help.", cmd),
        }
        Ok(())
    }

    // ── Account Scope Handlers ────────────────────────────────────────────────

    async fn handle_account_command(&mut self, parts: &[&str]) -> Result<()> {
        let cmd = parts[0].to_lowercase();
        let acc_id = self.current_account_id.unwrap();

        match cmd.as_str() {
            "a" | "d" | "c" => {
                self.handle_short_action(&cmd).await?;
            }
            "p" | "pos" => self.show_positions().await?,
            "b" | "bots" => self.show_bots().await?,
            "r" | "report" | "rpt" => self.show_account_report(acc_id).await?,
            
            // Order Execution (#BUY...)
            line if line.starts_with('#') => {
                self.execute_hash_command(line).await?;
            }

            _ => println!("❓ Unknown account command: {}. Type 'back' to exit scope.", cmd),
        }
        Ok(())
    }

    // ── Sub-module Handlers ──────────────────────────────────────────────────

    async fn handle_acc_management(&self, parts: &[&str]) -> Result<()> {
        if parts.len() < 2 {
            println!("Usage: acc add|del|status <args>");
            return Ok(());
        }

        match parts[1] {
            "list" | "l" => self.list_accounts().await?,
            "add" => {
                if parts.len() < 5 {
                    println!("Usage: acc add <id> <name> <token> [demo|real]");
                } else {
                    let id = parts[2].parse::<i64>().unwrap_or(0);
                    let name = parts[3].to_string();
                    let token = parts[4].to_string();
                    let atype = if parts.get(5) == Some(&"real") { AccountType::Real } else { AccountType::Demo };
                    
                    let mut acc = Account::new(id, name, id, atype);
                    acc.access_token = Some(token);
                    self.state.add_account(acc).await?;
                    println!("✅ Added Account #{}", id);
                }
            }
            "del" | "rm" => {
                if let Some(id_str) = parts.get(2) {
                    if let Ok(id) = id_str.parse::<i64>() {
                        if self.state.delete_account(id).await? {
                            println!("✅ Deleted Account #{}", id);
                        } else {
                            println!("❌ Account not found.");
                        }
                    }
                }
            }
            "on" | "off" => {
                if let Some(id_str) = parts.get(2) {
                    if let Ok(id) = id_str.parse::<i64>() {
                        let enabled = parts[1] == "on";
                        self.state.update_account_autotrade(id, enabled).await?;
                        println!("✅ Account #{} Autotrade: {}", id, if enabled { "ENABLED" } else { "DISABLED" });
                    }
                }
            }
            "status" | "st" | "r" => {
                if let Some(id_str) = parts.get(2) {
                    if let Ok(id) = id_str.parse::<i64>() {
                        self.show_account_report(id).await?;
                    }
                } else if let Some(id) = self.current_account_id {
                    self.show_account_report(id).await?;
                } else {
                    println!("Usage: acc status <id>");
                }
            }
            _ => println!("❓ Unknown acc subcommand. Try: list, add, del, on, off, status"),
        }
        Ok(())
    }

    async fn handle_api_management(&self, parts: &[&str]) -> Result<()> {
        if parts.len() < 2 {
            println!("Usage: api list|add|del|on|off <args>");
            return Ok(());
        }

        match parts[1] {
            "list" => {
                let clients = self.state.list_api_clients().await;
                println!("\n🔑 API CLIENTS");
                for c in clients {
                    println!("{}", c.format_list_item());
                }
            }
            "add" => {
                if parts.len() < 3 {
                    println!("Usage: api add <name> [source]");
                } else {
                    let name = parts[2].to_string();
                    let source = parts.get(3).unwrap_or(&"API").to_string();
                    let client = ApiClient::new(name, source);
                    let key = client.api_key.clone();
                    self.state.add_api_client(client).await?;
                    println!("✅ Created API Client. Key: {}", key);
                }
            }
            "on" | "off" => {
                if let Some(id) = parts.get(2) {
                    self.state.set_client_enabled(id, parts[1] == "on").await?;
                    println!("✅ Key {} updated.", id);
                }
            }
            "del" => {
                if let Some(id) = parts.get(2) {
                    self.state.delete_api_client(id).await?;
                    println!("✅ Key {} deleted.", id);
                }
            }
            _ => println!("Unknown api subcommand."),
        }
        Ok(())
    }

    async fn handle_sys_management(&self, parts: &[&str]) -> Result<()> {
        if parts.len() < 2 {
            println!("Usage: sys cleanup|nuclear|logs");
            return Ok(());
        }

        match parts[1] {
            "cleanup" => {
                println!("🧹 Initializing System Cleanup...");
                let status = std::process::Command::new("bash")
                    .arg("../scripts/99-uninstall-cleanup.sh")
                    .status()?;
                if status.success() { println!("✅ Cleanup completed."); }
            }
            "nuclear" => {
                println!("☢️ WARNING: INITIALIZING NUCLEAR CLEANUP (DB & ENV REMOVAL)...");
                let status = std::process::Command::new("bash")
                    .arg("../scripts/99-uninstall-cleanup.sh")
                    .arg("--force")
                    .status()?;
                if status.success() { println!("✅ System wiped clean."); }
            }
            "logs" => {
                let _ = std::process::Command::new("tail")
                    .args(["-n", "50", "iztrade.log"])
                    .status();
            }
            _ => println!("Unknown sys subcommand."),
        }
        Ok(())
    }

    // ── Reporting & View Tools ──────────────────────────────────────────────

    fn show_help(&self) {
        println!("\n💠 iZ-Console v2.1.0 Command Reference:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  🌐 GLOBAL COMMANDS (App Scope)");
        println!("    l | list            - List summary of all accounts");
        println!("    <ID>                - Enter Account Scope (e.g. 101)");
        println!("    a / d               - Global Enable/Disable ALL Autotrade");
        println!("    c                   - EMERGENCY CLOSE ALL + Global Disable");
        println!("    r                   - Show system-wide health report");
        println!("    #BUY #SELL...       - Order execution across all accounts");
        println!("");
        println!("  📁 MODULE: ACCOUNT MANAGEMENT (acc)");
        println!("    acc list            - List all accounts");
        println!("    acc add <id> <name> <token> [demo|real]");
        println!("    acc del <id>        - Delete account from DB");
        println!("    acc on/off <id>     - Toggle autotrade for one account");
        println!("    acc status <id>     - Detailed report for one account");
        println!("");
        println!("  🔑 MODULE: API KEY MANAGEMENT (api)");
        println!("    api list            - Show all external API keys");
        println!("    api add <name>      - Generate a new API key");
        println!("    api on/off <key>    - Enable/Disable an API key");
        println!("    api del <key>       - Remove an API key");
        println!("");
        println!("  🎯 ACCOUNT SCOPE (iz[acc_id] >)");
        println!("    p | pos             - Show open positions for this account");
        println!("    b | bots            - Manage trading bots for this account");
        println!("    r | report          - Detailed account report/PNL");
        println!("    a | d               - Enable/Disable autotrade for THIS acc");
        println!("    c                   - EMERGENCY CLOSE THIS account + Disable");
        println!("    #BUY #SELL...       - Execution restricted to THIS account");
        println!("    top | back | app    - Return to Global Scope");
        println!("");
        println!("  ⚙️ SYSTEM UTILITIES (sys)");
        println!("    sys cleanup         - Clean logs & temp files (Safe)");
        println!("    sys nuclear         - WIPE EVERYTHING (Delete DB & .env)");
        println!("    sys logs            - Tail latest system logs");
        println!("");
        println!("  ⌨️ NAVIGATION");
        println!("    clear | cls         - Clear terminal screen");
        println!("    exit | quit         - Shut down iZ-Console");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }

    async fn list_accounts(&self) -> Result<()> {
        let summary = self.state.format_accounts_summary().await;
        println!("{}", summary);
        Ok(())
    }

    async fn show_global_report(&self) -> Result<()> {
        let status = self.state.get_system_status().await;
        println!("\n📊 SYSTEM REPORT (GLOBAL)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Uptime: {}", AppState::format_uptime(status.uptime_secs));
        println!("Acc: {} targets | Connected: {} | Autotrade: {}", status.total_accounts, status.connected_accounts, status.active_accounts);
        println!("Real Balance: ${:.2} | Equity: ${:.2}", status.total_real_balance, status.total_real_equity);
        println!("Daily P&L: ${:+.2} | Float: {:+.2}", status.total_daily_pnl, status.total_float_profit);
        println!("Bots: {}/{} running | Positions: {}", status.active_bots, status.total_bots, status.open_positions);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        Ok(())
    }

    async fn show_account_report(&self, acc_id: i64) -> Result<()> {
        let accounts = self.state.accounts.read().await;
        let Some(acc) = accounts.get(&acc_id) else {
            println!("❌ Account #{} not found.", acc_id);
            return Ok(());
        };

        println!("\n💼 ACCOUNT #{} REPORT: {}", acc_id, acc.name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("Type: {:?} | Connected: {} | Auto: {}", acc.account_type, acc.connected, acc.autotrade);
        println!("Balance: ${:.2} | Equity: ${:.2}", acc.balance, acc.equity);
        println!("Daily P&L: ${:+.2} | Float: {:+.2}", acc.daily_pnl, acc.float_profit);
        println!("Limits: Target ${:.2} | MaxLoss ${:.2}", acc.daily_target_profit, acc.daily_max_loss);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        Ok(())
    }

    async fn show_positions(&self) -> Result<()> {
        let positions = self.state.get_open_positions().await;
        if positions.is_empty() {
            println!("📭 No open positions.");
            return Ok(());
        }
        
        println!("\n📦 OPEN POSITIONS");
        for p in positions {
            if let Some(target_id) = self.current_account_id {
                if p.account_id != target_id { continue; }
            }
            println!("Acc:{} | {} {} {} @ {} | P&L: {:+.2}", p.account_id, p.side, p.symbol, p.volume, p.open_price, p.pnl);
        }
        Ok(())
    }

    async fn show_bots(&self) -> Result<()> {
        let summary = self.state.format_bots_summary().await;
        println!("{}", summary);
        Ok(())
    }

    async fn handle_short_action(&self, cmd: &str) -> Result<()> {
        use crate::models::OrderRequest;
        let mut req = match cmd {
            "a" => OrderRequest::new(TradeSource::Api, "manual".to_string(), OrderAction::EnableAutotrade),
            "d" => OrderRequest::new(TradeSource::Api, "manual".to_string(), OrderAction::DisableAutotrade),
            "c" => OrderRequest::new(TradeSource::Api, "manual".to_string(), OrderAction::CloseAll),
            _ => return Ok(()),
        };

        if let Some(id) = self.current_account_id {
            req.account_scope = AccountScope::Single;
            req.account_ids = vec![id];
        }

        let res = dispatch(self.state.clone(), req).await?;
        println!("{}", res.to_telegram_msg());
        Ok(())
    }

    async fn execute_hash_command(&self, line: &str) -> Result<()> {
        if let Some(mut req) = IzParser::parse(line, TradeSource::Api) {
            if let Some(id) = self.current_account_id {
                req.account_scope = AccountScope::Single;
                req.account_ids = vec![id];
            } else {
                req.account_scope = AccountScope::All;
            }

            let res = dispatch(self.state.clone(), req).await?;
            println!("{}", res.to_telegram_msg());
        } else {
            println!("❌ Invalid # command format.");
        }
        Ok(())
    }
}
