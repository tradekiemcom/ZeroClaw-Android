#[cfg(test)]
mod tests {
    use iztrade::engine::IzParser;
    use iztrade::models::order::{TradeSource, OrderAction};

    #[test]
    fn test_parser_hash_buy() {
        let req = IzParser::parse("#BUY XAUUSD 0.1", TradeSource::Cli).unwrap();
        assert_eq!(req.action, OrderAction::Open);
        assert_eq!(req.symbol.unwrap(), "XAUUSD");
        assert_eq!(req.volume.unwrap(), 0.1);
    }

    #[test]
    fn test_parser_slash_closeall() {
        let req = IzParser::parse("/closeall", TradeSource::Telegram).unwrap();
        assert_eq!(req.action, OrderAction::CloseAll);
    }
}
