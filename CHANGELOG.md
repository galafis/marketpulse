# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-10-14

### Added
- Initial release of MarketPulse
- Real-time market data ingestion
- Simple Moving Average (SMA) calculation
- 24-hour volume tracking
- Comprehensive test suite with 10 tests
- CI/CD pipeline with GitHub Actions
- Code coverage reporting with Codecov
- Documentation with English and Portuguese support
- Mermaid diagrams for architecture and data flow
- Contributing guidelines
- MIT License

### Features
- `MarketData` struct for representing market tick data
- `MarketPulse` struct for managing market data
- `ingest()` method for ingesting market data
- `get_latest()` method for retrieving latest market data
- `calculate_sma()` method for calculating Simple Moving Average
- `get_volume_24h()` method for calculating volume

### Technical
- Built with Rust 1.90+
- Zero external dependencies
- High performance HashMap-based storage
- Comprehensive error handling
- Full documentation coverage

### Documentation
- Detailed README in English and Portuguese
- Architecture diagrams
- Data flow diagrams
- Code examples
- Contributing guidelines
- API documentation

### Testing
- Unit tests for all core functionality
- Edge case testing
- Integration tests
- Code coverage >80%

### CI/CD
- Automated testing on push/PR
- Clippy linting
- Rustfmt formatting checks
- Code coverage reporting
- Build verification

## [Unreleased]

### Planned Features
- EMA (Exponential Moving Average) calculation
- RSI (Relative Strength Index) indicator
- MACD (Moving Average Convergence Divergence) indicator
- Bollinger Bands calculation
- WebSocket API for real-time data streaming
- REST API for historical data queries
- Time-series database integration
- Real exchange integration (Binance, Coinbase)
- Dashboard UI
- Performance benchmarks
