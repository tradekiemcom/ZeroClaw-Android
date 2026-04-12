use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceQuote {
    pub symbol: String,
    pub bid: f64,
    pub ask: f64,
    pub spread: f64,        // ask - bid (in points)
    pub mid: f64,           // (bid + ask) / 2
    pub timestamp: DateTime<Utc>,
    pub source: String,     // "mock" | "ctrader" | "webhook"
}

impl PriceQuote {
    pub fn new(symbol: &str, bid: f64, ask: f64) -> Self {
        let spread = ((ask - bid) * 10000.0).round() / 10000.0;
        let mid = (bid + ask) / 2.0;
        Self {
            symbol: symbol.to_uppercase(),
            bid,
            ask,
            spread,
            mid,
            timestamp: Utc::now(),
            source: "mock".to_string(),
        }
    }

    pub fn from_ctrader(symbol: &str, bid: f64, ask: f64) -> Self {
        let mut q = Self::new(symbol, bid, ask);
        q.source = "ctrader".to_string();
        q
    }

    pub fn age_secs(&self) -> i64 {
        (Utc::now() - self.timestamp).num_seconds()
    }

    pub fn is_stale(&self) -> bool {
        self.age_secs() > 60  // Coi là stale nếu > 60 giây chưa update
    }
}

/// Danh sách các symbol mặc định và giá mock seed
pub fn default_mock_prices() -> Vec<PriceQuote> {
    vec![
        PriceQuote::new("XAUUSD",  3299.50, 3300.50),   // Gold
        PriceQuote::new("XAGUSD",   32.450,   32.500),  // Silver
        PriceQuote::new("BTCUSD", 84850.0,  84950.0),   // Bitcoin
        PriceQuote::new("ETHUSD",  1610.0,   1612.0),   // Ethereum
        PriceQuote::new("EURUSD",   1.0848,   1.0850),  // Euro
        PriceQuote::new("GBPUSD",   1.2648,   1.2650),  // Pound
        PriceQuote::new("USDJPY", 149.450,  149.470),   // Yen
        PriceQuote::new("USDCAD",   1.3748,   1.3750),  // CAD
        PriceQuote::new("AUDUSD",   0.6348,   0.6350),  // AUD
        PriceQuote::new("USDCHF",   0.8948,   0.8950),  // CHF
        PriceQuote::new("NZDUSD",   0.5748,   0.5750),  // NZD
        PriceQuote::new("US30",   39450.0,  39460.0),   // Dow Jones
        PriceQuote::new("US500",   5248.0,    5250.0),  // S&P 500
        PriceQuote::new("NAS100", 18248.0,  18252.0),   // Nasdaq
        PriceQuote::new("OIL",      71.48,    71.52),   // WTI Oil
    ]
}
