use marketpulse::{MarketData, MarketPulse};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_full_trading_workflow() {
    let mut pulse = MarketPulse::new();

    // Simulate a full trading day
    let symbols = ["BTCUSD", "ETHUSD"];
    let base_prices = [50000.0, 3000.0];

    // Ingest 100 ticks for each symbol
    for i in 0..100 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for (idx, symbol) in symbols.iter().enumerate() {
            pulse.ingest(MarketData {
                symbol: symbol.to_string(),
                price: base_prices[idx] + (i as f64 * 10.0),
                volume: 1.0 + (i as f64 * 0.01),
                timestamp,
            });
        }
    }

    // Verify data for BTCUSD
    let btc_latest = pulse.get_latest("BTCUSD").unwrap();
    assert_eq!(btc_latest.price, 50990.0);

    let btc_sma_10 = pulse.calculate_sma("BTCUSD", 10).unwrap();
    assert!(btc_sma_10 > 50900.0 && btc_sma_10 < 51000.0);

    let btc_volume = pulse.get_volume_24h("BTCUSD");
    assert!(btc_volume > 0.0);

    // Verify data for ETHUSD
    let eth_latest = pulse.get_latest("ETHUSD").unwrap();
    assert_eq!(eth_latest.price, 3990.0);

    let eth_sma_20 = pulse.calculate_sma("ETHUSD", 20).unwrap();
    assert!(eth_sma_20 > 3800.0 && eth_sma_20 < 4000.0);
}

#[test]
fn test_sma_accuracy() {
    let mut pulse = MarketPulse::new();

    // Create predictable data
    let prices = [100.0, 101.0, 102.0, 103.0, 104.0];
    for (i, price) in prices.iter().enumerate() {
        pulse.ingest(MarketData {
            symbol: "TEST".to_string(),
            price: *price,
            volume: 1.0,
            timestamp: i as u64,
        });
    }

    // SMA(5) = (100+101+102+103+104)/5 = 102
    let sma = pulse.calculate_sma("TEST", 5).unwrap();
    assert_eq!(sma, 102.0);

    // SMA(3) = (102+103+104)/3 = 103
    let sma_3 = pulse.calculate_sma("TEST", 3).unwrap();
    assert_eq!(sma_3, 103.0);
}

#[test]
fn test_concurrent_symbol_operations() {
    let mut pulse = MarketPulse::new();

    // Ingest data for multiple symbols simultaneously
    for i in 0..50 {
        pulse.ingest(MarketData {
            symbol: "BTC".to_string(),
            price: 50000.0 + i as f64,
            volume: 1.0,
            timestamp: i as u64,
        });

        pulse.ingest(MarketData {
            symbol: "ETH".to_string(),
            price: 3000.0 + i as f64,
            volume: 2.0,
            timestamp: i as u64,
        });
    }

    // Verify independent calculations
    let btc_sma = pulse.calculate_sma("BTC", 10).unwrap();
    let eth_sma = pulse.calculate_sma("ETH", 10).unwrap();

    assert!(btc_sma > 50000.0);
    assert!(eth_sma > 3000.0);
    assert_ne!(btc_sma, eth_sma);
}

#[test]
fn test_volume_aggregation() {
    let mut pulse = MarketPulse::new();

    let volumes = [1.5, 2.3, 3.7, 4.2, 5.0];
    for (i, volume) in volumes.iter().enumerate() {
        pulse.ingest(MarketData {
            symbol: "BTC".to_string(),
            price: 50000.0,
            volume: *volume,
            timestamp: i as u64,
        });
    }

    let total_volume = pulse.get_volume_24h("BTC");
    let expected: f64 = volumes.iter().sum();
    assert_eq!(total_volume, expected);
}

#[test]
fn test_edge_cases() {
    let mut pulse = MarketPulse::new();

    // Test with single data point
    pulse.ingest(MarketData {
        symbol: "BTC".to_string(),
        price: 50000.0,
        volume: 1.0,
        timestamp: 1,
    });

    // SMA with period 1 should work
    assert_eq!(pulse.calculate_sma("BTC", 1).unwrap(), 50000.0);

    // Test with very large period
    assert!(pulse.calculate_sma("BTC", 1000).is_none());

    // Test with non-existent symbol
    assert!(pulse.get_latest("XYZ").is_none());
    assert!(pulse.calculate_sma("XYZ", 5).is_none());
    assert_eq!(pulse.get_volume_24h("XYZ"), 0.0);
}
