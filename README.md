# Viztra

A desktop financial charting application built with Tauri and TradingView's lightweight-charts. Visualize historical market data from local Parquet files with professional candlestick charts.

## What is Viztra?

Viztra is a desktop application that lets you:
- 📊 View interactive candlestick charts powered by TradingView's lightweight-charts
- 📈 Analyze historical market data from local Parquet files
- 🔍 Explore multiple symbols and timeframes
- 💾 Work with data stored locally on your machine

## Quick Start

### Prerequisites

- **Node.js**: v25.1.0+
- **pnpm**: 10.26.2+
- **Rust**: 1.92.0+
- **System dependencies** (Linux):
  - `webkit2gtk`
  - `libayatana-appindicator`
  - `libappindicator`

### Setup

```bash
# Install dependencies
pnpm install

# Run the application
pnpm tauri dev
```

That's it! The app will open and automatically load EURUSD 1M data if available in your cache directory.

## Data Setup

Viztra reads Parquet files from your OS cache directory:
- **Linux**: `~/.cache/viztra/historical-asset-prices/{SYMBOL}/{TIMEFRAME}.parquet`
- **macOS**: `~/Library/Caches/viztra/historical-asset-prices/{SYMBOL}/{TIMEFRAME}.parquet`
- **Windows**: `%LOCALAPPDATA%\viztra\historical-asset-prices\{SYMBOL}\{TIMEFRAME}.parquet`

Use the "Pull Assets" button in the app to download data, or place Parquet files manually in the cache directory.

## Tech Stack

- **Framework**: Tauri (Rust + Web)
- **Frontend**: React 19 + TypeScript
- **Charting**: TradingView lightweight-charts v5
- **Data**: Parquet files (via Polars)
- **Styling**: CSS Modules

## Roadmap

- [x] Basic candlestick chart rendering
- [ ] Symbol and timeframe selection
- [ ] Chart drawing tools
- [ ] Chart indicators
- [ ] Chart layouts settings
- [ ] Data export capabilities

## Development

```bash
# Development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## License

MIT License - See [LICENSE](LICENSE) file for details.

You are free to use, modify, and distribute this software for your own analysis and projects.

## Third-Party Licenses

This project is built on top of [TradingView's Lightweight Charts™](https://www.tradingview.com/lightweight-charts/), which is licensed under the Apache License 2.0. See [NOTICES](NOTICES) file for full attribution and license details.

Lightweight Charts™ is a trademark of TradingView, Inc.
