use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct MarketPulse {
    data: HashMap<String, Vec<MarketData>>,
}

impl MarketPulse {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn ingest(&mut self, data: MarketData) {
        self.data
            .entry(data.symbol.clone())
            .or_insert_with(Vec::new)
            .push(data);
    }

    pub fn get_latest(&self, symbol: &str) -> Option<&MarketData> {
        self.data.get(symbol).and_then(|v| v.last())
    }

    pub fn calculate_sma(&self, symbol: &str, period: usize) -> Option<f64> {
        let prices = self.data.get(symbol)?;
        if prices.len() < period {
            return None;
        }
        let sum: f64 = prices.iter().rev().take(period).map(|d| d.price).sum();
        Some(sum / period as f64)
    }

    pub fn get_volume_24h(&self, symbol: &str) -> f64 {
        self.data
            .get(symbol)
            .map(|v| v.iter().map(|d| d.volume).sum())
            .unwrap_or(0.0)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_market_pulse() {
        let pulse = MarketPulse::new();
        assert!(pulse.data.is_empty());
    }

    #[test]
    fn test_ingest_market_data() {
        let mut pulse = MarketPulse::new();
        let data = MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0,
            volume: 1.0,
            timestamp: 1,
        };
        pulse.ingest(data.clone());
        assert_eq!(pulse.data.len(), 1);
        assert_eq!(pulse.data["BTCUSD"].len(), 1);
        assert_eq!(pulse.data["BTCUSD"][0].price, 50000.0);
    }

    #[test]
    fn test_get_latest() {
        let mut pulse = MarketPulse::new();
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0,
            volume: 1.0,
            timestamp: 1,
        });
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 51000.0,
            volume: 2.0,
            timestamp: 2,
        });
        let latest = pulse.get_latest("BTCUSD").unwrap();
        assert_eq!(latest.price, 51000.0);
    }

    #[test]
    fn test_calculate_sma() {
        let mut pulse = MarketPulse::new();
        for i in 0..10 {
            pulse.ingest(MarketData {
                symbol: "BTCUSD".to_string(),
                price: 100.0 + i as f64,
                volume: 1.0,
                timestamp: i as u64,
            });
        }
        // SMA for last 5 prices: (109+108+107+106+105)/5 = 107
        let sma = pulse.calculate_sma("BTCUSD", 5).unwrap();
        assert_eq!(sma, 107.0);

        // Not enough data for SMA
        assert!(pulse.calculate_sma("BTCUSD", 15).is_none());
    }

    #[test]
    fn test_get_volume_24h() {
        let mut pulse = MarketPulse::new();
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0,
            volume: 1.0,
            timestamp: 1,
        });
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 51000.0,
            volume: 2.0,
            timestamp: 2,
        });
        let volume = pulse.get_volume_24h("BTCUSD");
        assert_eq!(volume, 3.0);

        // Test with non-existent symbol
        let volume_non_existent = pulse.get_volume_24h("ETHUSD");
        assert_eq!(volume_non_existent, 0.0);
    }
}

