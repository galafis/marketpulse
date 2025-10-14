use marketpulse::{MarketData, MarketPulse};
use std::time::{SystemTime, UNIX_EPOCH};

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
