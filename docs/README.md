# MarketPulse Documentation

This directory contains architecture diagrams and documentation for the MarketPulse project.

## 📁 Contents

### Architecture Diagrams

#### `architecture.mmd` / `images/architecture.png`
System architecture diagram showing:
- Data sources (exchanges)
- Ingestion layer
- MarketPulse core engine
- Storage layer (time-series DB, cache)
- Analytics components
- Output layer (API, WebSocket, Dashboard)

#### `data_flow.mmd` / `images/data_flow.png`
Data flow sequence diagram illustrating:
- Real-time data streaming from exchanges
- Data validation and storage
- Analytics processing (SMA, EMA, RSI, MACD, Bollinger Bands)
- Client request/response flows
- Volume calculations

## 🔨 Building Diagrams

The Mermaid diagrams can be rendered using:

1. **GitHub** - Displays automatically in README.md
2. **Mermaid Live Editor** - https://mermaid.live/
3. **VS Code** - Using Mermaid extension
4. **Command line** - Using mermaid-cli:
   ```bash
   npm install -g @mermaid-js/mermaid-cli
   mmdc -i docs/architecture.mmd -o docs/images/architecture.png
   mmdc -i docs/data_flow.mmd -o docs/images/data_flow.png
   ```

## 📚 Additional Documentation

- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [CHANGELOG.md](../CHANGELOG.md) - Version history
- [README.md](../README.md) - Main project documentation

## 🎯 Future Documentation

Planned documentation additions:
- API Reference Guide
- Performance Benchmarks
- Deployment Guide
- Configuration Reference
- Troubleshooting Guide
