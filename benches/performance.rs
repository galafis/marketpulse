use marketpulse::{MarketData, MarketPulse};
use std::time::Instant;

fn main() {
    println!("🔬 MarketPulse Performance Benchmarks\n");

    // Benchmark 1: Data ingestion
    benchmark_ingestion();

    // Benchmark 2: SMA calculation
    benchmark_sma();

    // Benchmark 3: Latest data retrieval
    benchmark_latest();

    // Benchmark 4: Volume calculation
    benchmark_volume();
}

fn benchmark_ingestion() {
    println!("📥 Benchmarking data ingestion...");
    let mut pulse = MarketPulse::new();

    let start = Instant::now();
    for i in 0..100_000 {
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0 + (i as f64),
            volume: 1.0,
            timestamp: i as u64,
        });
    }
    let duration = start.elapsed();

    println!("  ✓ Ingested 100,000 ticks in {:?}", duration);
    println!(
        "  ✓ Rate: {:.0} ticks/sec\n",
        100_000.0 / duration.as_secs_f64()
    );
}

fn benchmark_sma() {
    println!("📊 Benchmarking SMA calculation...");
    let mut pulse = MarketPulse::new();

    // Prepare data
    for i in 0..10_000 {
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0 + (i as f64),
            volume: 1.0,
            timestamp: i as u64,
        });
    }

    let start = Instant::now();
    for _ in 0..1_000 {
        pulse.calculate_sma("BTCUSD", 20);
    }
    let duration = start.elapsed();

    println!("  ✓ Calculated 1,000 SMA(20) in {:?}", duration);
    println!("  ✓ Average: {:?} per calculation\n", duration / 1_000);
}

fn benchmark_latest() {
    println!("🔍 Benchmarking latest data retrieval...");
    let mut pulse = MarketPulse::new();

    for i in 0..10_000 {
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0 + (i as f64),
            volume: 1.0,
            timestamp: i as u64,
        });
    }

    let start = Instant::now();
    for _ in 0..1_000_000 {
        pulse.get_latest("BTCUSD");
    }
    let duration = start.elapsed();

    println!("  ✓ Retrieved latest 1,000,000 times in {:?}", duration);
    println!("  ✓ Average: {:?} per retrieval\n", duration / 1_000_000);
}

fn benchmark_volume() {
    println!("💹 Benchmarking volume calculation...");
    let mut pulse = MarketPulse::new();

    for i in 0..10_000 {
        pulse.ingest(MarketData {
            symbol: "BTCUSD".to_string(),
            price: 50000.0,
            volume: 1.0 + (i as f64 * 0.01),
            timestamp: i as u64,
        });
    }

    let start = Instant::now();
    for _ in 0..10_000 {
        pulse.get_volume_24h("BTCUSD");
    }
    let duration = start.elapsed();

    println!("  ✓ Calculated volume 10,000 times in {:?}", duration);
    println!("  ✓ Average: {:?} per calculation\n", duration / 10_000);
}
