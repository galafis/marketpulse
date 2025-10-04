use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn main() {
    println!("📊 MarketPulse - Real-Time Market Data Analytics");
    println!("===============================================\n");

    let mut pulse = MarketPulse::new();

    // Simulate market data ingestion
    for i in 0..100 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0 + (i as f64 * 10.0),
            volume: 1.5 + (i as f64 * 0.01),
            timestamp,
        });
    }

    // Display analytics
    if let Some(latest) = pulse.get_latest("BTCUSD") {
        println!("📈 Latest BTCUSD:");
        println!("  Price: ${:.2}", latest.price);
        println!("  Volume: {:.2}", latest.volume);
    }

    if let Some(sma_20) = pulse.calculate_sma("BTCUSD", 20) {
        println!("\n📊 SMA(20): ${:.2}", sma_20);
    }

    let volume_24h = pulse.get_volume_24h("BTCUSD");
    println!("\n💹 24h Volume: {:.2} BTC", volume_24h);
}
