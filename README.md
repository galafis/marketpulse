# 📊 MarketPulse - Real-Time Market Data Analytics Platform



[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/galafis/marketpulse/workflows/CI/badge.svg)](https://github.com/galafis/marketpulse/actions)
[![codecov](https://codecov.io/gh/galafis/marketpulse/branch/master/graph/badge.svg)](https://codecov.io/gh/galafis/marketpulse)
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


### 🏗️ Architecture

![Architecture Diagram](docs/images/architecture.png)

The platform implements a real-time data processing pipeline:

1. **Data Sources**: WebSocket feeds from exchanges (Binance, Coinbase, etc.)
2. **Ingestion Layer**: High-throughput data ingestion with buffering
3. **Processing Engine**: Technical indicator calculations (SMA, EMA, RSI, MACD)
4. **Storage Layer**: Time-series database for historical data
5. **API Layer**: REST and WebSocket APIs for client access
6. **Analytics**: Volume analysis, price trends, and market insights

### 📊 Data Flow

![Data Flow Diagram](docs/images/data_flow.png)

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

#### Basic Example

```rust
use marketpulse::{MarketPulse, MarketData};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut pulse = MarketPulse::new();

    // Simulate market data ingestion
    for i in 0..10 {
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
        println!("Latest BTCUSD:");
        println!("  Price: ${:.2}", latest.price);
        println!("  Volume: {:.2}", latest.volume);
    }

    if let Some(sma_20) = pulse.calculate_sma("BTCUSD", 5) {
        println!("SMA(5): ${:.2}", sma_20);
    }

    let volume_24h = pulse.get_volume_24h("BTCUSD");
    println!("24h Volume: {:.2} BTC", volume_24h);
}
```

#### More Examples

See the `examples/` directory for more detailed examples:

```bash
# Basic usage
cargo run --example basic_usage

# Multiple trading symbols
cargo run --example multiple_symbols

# Trading signals with SMA crossover
cargo run --example trading_signals
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

### 🏗️ Arquitetura

![Diagrama de Arquitetura](docs/images/architecture.png)

A plataforma implementa um pipeline de processamento de dados em tempo real:

1. **Fontes de Dados**: Feeds WebSocket de exchanges (Binance, Coinbase, etc.)
2. **Camada de Ingestão**: Ingestão de dados de alto throughput com buffering
3. **Motor de Processamento**: Cálculos de indicadores técnicos (SMA, EMA, RSI, MACD)
4. **Camada de Armazenamento**: Banco de dados de séries temporais para dados históricos
5. **Camada de API**: APIs REST e WebSocket para acesso de clientes
6. **Analytics**: Análise de volume, tendências de preço e insights de mercado

### 📊 Fluxo de Dados

![Diagrama de Fluxo de Dados](docs/images/data_flow.png)

### 🛠️ Instalação

```bash
git clone https://github.com/galafis/marketpulse.git
cd marketpulse
cargo build --release
```

### 🎯 Início Rápido

```bash
cargo run --release
```

Saída:
```
📊 MarketPulse - Real-Time Market Data Analytics
===============================================

📈 Latest BTCUSD:
  Price: $50990.00
  Volume: 2.49

📊 SMA(20): $50895.00

💹 24h Volume: 199.50 BTC
```

### 📚 Exemplos de Uso

#### Exemplo Básico

```rust
use marketpulse::{MarketPulse, MarketData};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut pulse = MarketPulse::new();

    // Simular ingestão de dados de mercado
    for i in 0..10 {
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

    // Exibir analytics
    if let Some(latest) = pulse.get_latest("BTCUSD") {
        println!("Último BTCUSD:");
        println!("  Preço: ${:.2}", latest.price);
        println!("  Volume: {:.2}", latest.volume);
    }

    if let Some(sma_20) = pulse.calculate_sma("BTCUSD", 5) {
        println!("SMA(5): ${:.2}", sma_20);
    }

    let volume_24h = pulse.get_volume_24h("BTCUSD");
    println!("Volume 24h: {:.2} BTC", volume_24h);
}
```

#### Mais Exemplos

Veja o diretório `examples/` para exemplos mais detalhados:

```bash
# Uso básico
cargo run --example basic_usage

# Múltiplos símbolos de trading
cargo run --example multiple_symbols

# Sinais de trading com cruzamento de SMA
cargo run --example trading_signals
```

### 🧪 Executar Testes

```bash
# Executar todos os testes
cargo test

# Executar testes com saída detalhada
cargo test -- --nocapture

# Executar clippy para análise de código
cargo clippy -- -D warnings
```

### 📄 Licença

Licença MIT - consulte [LICENSE](LICENSE) para detalhes.

### 👤 Autor

**Gabriel Demetrios Lafis**
- Analista e Desenvolvedor de Sistemas
- Gestor de Tecnologia da Informação
- Especialista em Segurança Cibernética
- Business Intelligence / Business Analyst
- Analista e Cientista de Dados
