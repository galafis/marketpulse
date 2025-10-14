use std::time::{SystemTime, UNIX_EPOCH};
use marketpulse::{MarketData, MarketPulse};

fn main() {
    println!("📊 Multiple Symbols Trading Example\n");

    let mut pulse = MarketPulse::new();

    // Simulate trading data for multiple cryptocurrencies
    let symbols = vec![
        ("BTCUSD", 50000.0),
        ("ETHUSD", 3000.0),
        ("ADAUSD", 0.5),
        ("SOLUSD", 100.0),
    ];

    println!("Ingesting market data for {} symbols...", symbols.len());
    
    for i in 0..30 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        for (symbol, base_price) in &symbols {
            // Simulate price fluctuation
            let price_change = (i as f64 * 0.01 * base_price).sin() * 0.02 * base_price;
            let price = base_price + price_change;
            let volume = 1.0 + (i as f64 * 0.1);
            
            pulse.ingest(MarketData {
                symbol: symbol.to_string(),
                price,
                volume,
                timestamp,
            });
        }
    }
    println!("✓ Ingested 30 data points for each symbol\n");

    // Display analytics for each symbol
    println!("📈 Market Summary:\n");
    for (symbol, _) in &symbols {
        println!("{}:", symbol);
        
        if let Some(latest) = pulse.get_latest(symbol) {
            println!("  Latest Price: ${:.2}", latest.price);
        }
        
        if let Some(sma_10) = pulse.calculate_sma(symbol, 10) {
            println!("  SMA(10): ${:.2}", sma_10);
        }
        
        if let Some(sma_20) = pulse.calculate_sma(symbol, 20) {
            println!("  SMA(20): ${:.2}", sma_20);
        }
        
        let volume = pulse.get_volume_24h(symbol);
        println!("  Total Volume: {:.2}", volume);
        println!();
    }
}
