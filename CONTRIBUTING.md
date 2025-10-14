# Contributing to MarketPulse

Thank you for your interest in contributing to MarketPulse! This document provides guidelines for contributing to the project.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/marketpulse.git
   cd marketpulse
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## 🔧 Development Setup

### Prerequisites
- Rust 1.90 or higher
- Cargo (comes with Rust)

### Build and Test
```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with coverage
cargo tarpaulin

# Check code quality
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## 📝 Code Guidelines

### Code Style
- Follow the Rust style guidelines
- Use `cargo fmt` to format your code
- Ensure `cargo clippy` passes without warnings
- Write meaningful commit messages

### Testing
- Add tests for all new functionality
- Ensure all tests pass before submitting a PR
- Aim for high test coverage (>80%)

### Documentation
- Add doc comments for public APIs
- Update README.md if needed
- Include examples in doc comments

## 🔄 Pull Request Process

1. **Update documentation** if you're changing functionality
2. **Add tests** for new features
3. **Ensure CI passes** - all tests and checks must pass
4. **Update CHANGELOG.md** with your changes
5. **Request review** from maintainers

## 📋 Commit Message Format

Use conventional commit format:
```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Example:
```
feat(indicators): add RSI calculation

Implement Relative Strength Index (RSI) technical indicator
with configurable period.

Closes #123
```

## 🐛 Reporting Bugs

When reporting bugs, please include:
- Operating system and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected behavior
- Actual behavior
- Any error messages

## 💡 Feature Requests

We welcome feature requests! Please:
- Check if the feature already exists or is planned
- Clearly describe the use case
- Explain how it would benefit users
- Consider implementation complexity

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🤝 Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other contributors

## ❓ Questions?

If you have questions, feel free to:
- Open an issue for discussion
- Contact the maintainers

Thank you for contributing to MarketPulse! 🎉
