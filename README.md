# Viztra

A powerful desktop application designed to empower trading financial analysis through professional charting tools, comprehensive analytics, and detailed performance reports. Built with Tauri and TradingView's lightweight-charts, Viztra provides traders and analysts with the tools they need to visualize market data, analyze trading performance, and gain insights from their trading activity.

## What is Viztra?

Viztra is a comprehensive trading analysis platform that combines:

- 📊 **Professional Charting**: Interactive candlestick charts powered by TradingView's lightweight-charts for visualizing historical market data
- 📈 **Performance Analytics**: Advanced insights and analytics tools for analyzing trading performance, profits, and deal entries
- 📋 **Performance Reports**: Detailed reports including profit by symbol, trade entries analysis, balance tracking, and more
- 💾 **Local Data Management**: Work with your trading data stored locally in Parquet format for privacy and performance
- 🔍 **Flexible Data Import**: Import and validate deal/trade data from Parquet files with automatic schema validation

## Key Features

### Charting & Visualization

- Professional candlestick charts with TradingView's lightweight-charts v5
- Historical market data visualization from local Parquet files
- Support for multiple symbols and timeframes
- Interactive chart navigation and zooming
- Dark theme optimized for trading environments

### Performance Analytics & Insights

Viztra includes a powerful insights system that provides various analytical reports:

- **Profit by Symbol**: Calculate total profit, volume, and trade count grouped by trading symbol
- **Trade Entries**: Analyze all trade entries (entry == 1) for detailed trade-by-trade analysis
- **Balance Entries**: Track balance entries (type == 2 AND entry == 0) to monitor account balance changes
- **Total Balance**: Calculate the sum of profit from balance entries
- **Trade Entries with Balance**: Combined analysis of trade entries and balance entries
- **All Deal Entries**: Complete view of all deal entries with no filtering

These insights can be executed individually or in batch for comprehensive performance analysis.

### Data Management

- **Historical Asset Data**: Download and manage historical price data (OHLC) from GitHub repositories
- **Deal/Trade Data**: Import, validate, and analyze trading deal data from Parquet files
- **Automatic Validation**: Schema validation ensures data integrity during import
- **Local Storage**: All data stored locally in your OS cache directory for privacy and performance

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

# Run the application in development mode
pnpm tauri dev
```

That's it! The app will open and you can start analyzing your trading data.

## Data Setup

### Historical Asset Prices

Viztra reads historical price data (OHLC) from Parquet files in your OS cache directory:

- **Linux**: `~/.cache/viztra/historical-asset-prices/{SYMBOL}/{TIMEFRAME}.parquet`
- **macOS**: `~/Library/Caches/viztra/historical-asset-prices/{SYMBOL}/{TIMEFRAME}.parquet`
- **Windows**: `%LOCALAPPDATA%\viztra\historical-asset-prices\{SYMBOL}\{TIMEFRAME}.parquet`

Use the "Pull Assets" button in the app to download data from the configured repository, or place Parquet files manually in the cache directory.

### Trading Deal Data

Import your trading deal data from Parquet files. The application expects deal files with the following schema:

- `ticket`, `order`, `time`, `time_msc`, `type`, `entry`, `magic`
- `position_id`, `reason`, `volume`, `price`, `commission`, `swap`
- `profit`, `fee`, `symbol`, `comment`, `external_id`

The application will automatically validate the schema during import to ensure data integrity.

## Tech Stack

- **Framework**: Tauri 2 (Rust backend + Web frontend)
- **Frontend**: React 19 + TypeScript
- **Charting**: TradingView lightweight-charts v5
- **Data Processing**: Polars (for efficient Parquet file handling)
- **Backend**: Rust with async/await support (Tokio)
- **Data Format**: Parquet files for efficient storage and querying
- **Styling**: CSS Modules
- **Schema Validation**: JSON Schema with schemars

## Architecture

Viztra follows a modular architecture:

- **Features**: Organized by domain (assets, deals, insights, charting)
- **Insights System**: Extensible plugin-like architecture for adding new analytics
- **Shared Utilities**: Common functionality for error handling, validation, and cache management
- **Type-Safe APIs**: Full TypeScript integration with Rust backend via Tauri commands

## Development

```bash
# Development mode (with hot reload)
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Project Structure

- `src/` - Frontend React application
- `src-tauri/` - Rust backend application
- `src-tauri/src/features/` - Feature modules (assets, deals, insights)
- `src-tauri/src/shared/` - Shared utilities and error handling

## License

MIT License - See [LICENSE](LICENSE) file for details.

You are free to use, modify, and distribute this software for your own analysis and projects.

## Third-Party Licenses

This project is built on top of [TradingView's Lightweight Charts™](https://www.tradingview.com/lightweight-charts/), which is licensed under the Apache License 2.0. See [NOTICES](NOTICES) file for full attribution and license details.

Lightweight Charts™ is a trademark of TradingView, Inc.
