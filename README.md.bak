# 📊 MarketPulse - Real-Time Market Data Analytics Platform

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)

[English](#english) | [Português](#português)

---

## English

### 🚀 Overview

**MarketPulse** is a high-performance real-time market data analytics platform built in Rust. It processes streaming market data, calculates technical indicators, and provides instant insights for trading decisions.

### ✨ Key Features

- **Real-Time Data Ingestion**: Process market data streams with minimal latency
- **Technical Indicators**: SMA, EMA, RSI, MACD, Bollinger Bands
- **Volume Analytics**: 24h volume tracking and analysis
- **Historical Data Storage**: Efficient time-series data management
- **WebSocket API**: Real-time data distribution to clients

### 🛠️ Installation

```bash
git clone https://github.com/gabriellafis/marketpulse.git
cd marketpulse
cargo build --release
```

### 🎯 Quick Start

```bash
cargo run --release
```

Output:
```
📊 MarketPulse - Real-Time Market Data Analytics
===============================================

📈 Latest BTCUSD:
  Price: $50990.00
  Volume: 2.49

📊 SMA(20): $50895.00

💹 24h Volume: 199.50 BTC
```

### 📚 Usage Examples

```rust
use marketpulse::{MarketPulse, MarketData};

fn main() {
    let mut pulse = MarketPulse::new();

    // Ingest market data
    pulse.ingest(MarketData {
        symbol: "BTCUSD".to_string(),
        price: 50000.0,
        volume: 1.5,
        timestamp: 1234567890,
    });

    // Get latest price
    if let Some(latest) = pulse.get_latest("BTCUSD") {
        println!("Latest price: ${}", latest.price);
    }

    // Calculate SMA
    if let Some(sma) = pulse.calculate_sma("BTCUSD", 20) {
        println!("SMA(20): ${}", sma);
    }

    // Get 24h volume
    let volume = pulse.get_volume_24h("BTCUSD");
    println!("24h Volume: {} BTC", volume);
}
```

### 📄 License

MIT License - see [LICENSE](LICENSE) for details.

### 👤 Author

**Gabriel Demetrios Lafis**
- Systems Analyst & Developer
- IT Manager
- Cybersecurity Specialist
- Business Intelligence / Business Analyst
- Data Analyst & Data Scientist

---

## Português

### 🚀 Visão Geral

**MarketPulse** é uma plataforma de analytics de dados de mercado em tempo real de alto desempenho construída em Rust. Processa streams de dados de mercado, calcula indicadores técnicos e fornece insights instantâneos para decisões de trading.

### ✨ Principais Recursos

- **Ingestão de Dados em Tempo Real**: Processa streams de dados de mercado com latência mínima
- **Indicadores Técnicos**: SMA, EMA, RSI, MACD, Bandas de Bollinger
- **Analytics de Volume**: Rastreamento e análise de volume 24h
- **Armazenamento de Dados Históricos**: Gerenciamento eficiente de dados de séries temporais
- **WebSocket API**: Distribuição de dados em tempo real para clientes

### 📄 Licença

Licença MIT - consulte [LICENSE](LICENSE) para detalhes.

### 👤 Autor

**Gabriel Demetrios Lafis**
- Analista e Desenvolvedor de Sistemas
- Gestor de Tecnologia da Informação
- Especialista em Segurança Cibernética
- Business Intelligence / Business Analyst
- Analista e Cientista de Dados
