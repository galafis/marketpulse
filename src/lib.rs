use std::collections::HashMap;

/// Represents a single market data point with price, volume, and timestamp information.
#[derive(Debug, Clone)]
pub struct MarketData {
    /// The trading symbol (e.g., "BTCUSD", "ETHUSD")
    pub symbol: String,
    /// The price at the time of the tick
    pub price: f64,
    /// The volume traded
    pub volume: f64,
    /// Unix timestamp in seconds
    pub timestamp: u64,
}

/// Main structure for managing market data and performing analytics.
#[derive(Debug, Default)]
pub struct MarketPulse {
    data: HashMap<String, Vec<MarketData>>,
}

impl MarketPulse {
    /// Creates a new MarketPulse instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ingests a new market data point.
    ///
    /// # Arguments
    ///
    /// * `data` - The market data to ingest
    pub fn ingest(&mut self, data: MarketData) {
        self.data.entry(data.symbol.clone()).or_default().push(data);
    }

    /// Gets the latest market data for a given symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading symbol to query
    ///
    /// # Returns
    ///
    /// The most recent market data point, or None if no data exists
    pub fn get_latest(&self, symbol: &str) -> Option<&MarketData> {
        self.data.get(symbol).and_then(|v| v.last())
    }

    /// Calculates the Simple Moving Average (SMA) for a given symbol and period.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading symbol to analyze
    /// * `period` - The number of periods for the SMA calculation
    ///
    /// # Returns
    ///
    /// The SMA value, or None if insufficient data is available
    pub fn calculate_sma(&self, symbol: &str, period: usize) -> Option<f64> {
        let prices = self.data.get(symbol)?;
        if prices.len() < period {
            return None;
        }
        let sum: f64 = prices.iter().rev().take(period).map(|d| d.price).sum();
        Some(sum / period as f64)
    }

    /// Gets the total volume for a symbol.
    ///
    /// Note: Currently returns all-time volume. In production, this should
    /// be filtered to the last 24 hours based on timestamps.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The trading symbol to query
    ///
    /// # Returns
    ///
    /// The total volume, or 0.0 if no data exists
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

    #[test]
    fn test_multiple_symbols() {
        let mut pulse = MarketPulse::new();
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0,
            volume: 1.0,
            timestamp: 1,
        });
        pulse.ingest(MarketData {
            symbol: "ETHUSD".to_string(),
            price: 3000.0,
            volume: 2.0,
            timestamp: 1,
        });

        let btc_latest = pulse.get_latest("BTCUSD").unwrap();
        let eth_latest = pulse.get_latest("ETHUSD").unwrap();

        assert_eq!(btc_latest.price, 50000.0);
        assert_eq!(eth_latest.price, 3000.0);
    }

    #[test]
    fn test_sma_with_exact_period() {
        let mut pulse = MarketPulse::new();
        for i in 0..5 {
            pulse.ingest(MarketData {
                symbol: "BTCUSD".to_string(),
                price: 100.0 + i as f64,
                volume: 1.0,
                timestamp: i as u64,
            });
        }
        // SMA for last 5 prices: (104+103+102+101+100)/5 = 102
        let sma = pulse.calculate_sma("BTCUSD", 5).unwrap();
        assert_eq!(sma, 102.0);
    }

    #[test]
    fn test_default_trait() {
        let pulse = MarketPulse::default();
        assert!(pulse.data.is_empty());
    }

    #[test]
    fn test_get_latest_empty() {
        let pulse = MarketPulse::new();
        assert!(pulse.get_latest("BTCUSD").is_none());
    }

    #[test]
    fn test_calculate_sma_empty() {
        let pulse = MarketPulse::new();
        assert!(pulse.calculate_sma("BTCUSD", 5).is_none());
    }
}
