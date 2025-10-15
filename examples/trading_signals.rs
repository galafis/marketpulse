use marketpulse::{MarketData, MarketPulse};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("🔔 Simple Trading Signal Example\n");
    println!("Strategy: Golden Cross (SMA crossover)\n");

    let mut pulse = MarketPulse::new();

    // Simulate price trend
    println!("Simulating price trend...");
    for i in 0..50 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create an uptrend pattern
        let base_price = 50000.0;
        let trend = i as f64 * 20.0;
        let noise = (i as f64 * 0.5).sin() * 100.0;
        let price = base_price + trend + noise;

        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price,
            volume: 1.0 + (i as f64 * 0.02),
            timestamp,
        });

        // Check for signals every 10 data points
        if i >= 20 && i % 10 == 0 {
            let sma_10 = pulse.calculate_sma("BTCUSD", 10);
            let sma_20 = pulse.calculate_sma("BTCUSD", 20);

            if let (Some(short_sma), Some(long_sma)) = (sma_10, sma_20) {
                let latest = pulse.get_latest("BTCUSD").unwrap();

                println!("\n📊 Data Point {}:", i);
                println!("  Current Price: ${:.2}", latest.price);
                println!("  SMA(10): ${:.2}", short_sma);
                println!("  SMA(20): ${:.2}", long_sma);

                // Simple signal logic
                if short_sma > long_sma {
                    let diff = (short_sma - long_sma) / long_sma * 100.0;
                    println!(
                        "  🟢 BULLISH SIGNAL - Short SMA is {:.2}% above Long SMA",
                        diff
                    );
                } else {
                    let diff = (long_sma - short_sma) / long_sma * 100.0;
                    println!(
                        "  🔴 BEARISH SIGNAL - Short SMA is {:.2}% below Long SMA",
                        diff
                    );
                }
            }
        }
    }

    println!("\n✓ Analysis complete");
}
