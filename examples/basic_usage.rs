use marketpulse::{MarketData, MarketPulse};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("📈 Basic MarketPulse Usage Example\n");

    let mut pulse = MarketPulse::new();

    // Ingest some sample market data
    println!("Ingesting market data...");
    for i in 0..20 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0 + (i as f64 * 50.0),
            volume: 1.0 + (i as f64 * 0.05),
            timestamp,
        });
    }
    println!("✓ Ingested 20 data points\n");

    // Get latest price
    if let Some(latest) = pulse.get_latest("BTCUSD") {
        println!("📊 Latest Market Data:");
        println!("  Symbol: {}", latest.symbol);
        println!("  Price: ${:.2}", latest.price);
        println!("  Volume: {:.2}", latest.volume);
        println!("  Timestamp: {}\n", latest.timestamp);
    }

    // Calculate SMA with different periods
    println!("📈 Simple Moving Averages:");
    for period in [5, 10, 15, 20] {
        if let Some(sma) = pulse.calculate_sma("BTCUSD", period) {
            println!("  SMA({}): ${:.2}", period, sma);
        }
    }

    // Get total volume
    let volume = pulse.get_volume_24h("BTCUSD");
    println!("\n💹 Total Volume: {:.2} BTC", volume);
}
