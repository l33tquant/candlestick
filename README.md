<div align="center">

# 🕯️ Candlestick

### Candlestick Pattern Recognition for Rust

![Tests](https://github.com/l33tquant/candlestick/actions/workflows/ci.yml/badge.svg?branch=main)
[![Crates.io](https://img.shields.io/crates/v/candlestick-rs.svg)](https://crates.io/crates/candlestick-rs)
[![Documentation](https://docs.rs/candlestick-rs/badge.svg)](https://docs.rs/candlestick-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

A `no_std` and zero-dependency Rust library for identifying Japanese candlestick patterns in financial markets. Perfect for algorithmic trading, backtesting engines, and technical analysis tools.

## ✨ Features

- **Comprehensive Pattern Coverage**: Detects 30+ traditional Japanese candlestick patterns
- **Zero Dependencies**: Fully `no_std` compatible with no external dependency
- **Flexible Architecture**:
  - Single-candle pattern detection via `CandleStick` trait
  - Multi-candle pattern analysis via `CandleStream`
- **Performance-Oriented**: Zero allocation and efficient algorithms
- **Simple Integration**: Works with any data structure through straightforward trait implementation

## 📋 Supported Patterns

### Single Candle Patterns

- Basic Formations: Bullish/Bearish, Marubozu
- Reversal Signals: Hammer, Inverted Hammer, Hanging Man, Shooting Star
- Indecision Indicators: Spinning Top, Doji and variants (Long-Legged, Dragonfly, Gravestone)

### Multi-Candle Patterns

- Reversals: Engulfing, Harami, Morning/Evening Star, Three Inside Up/Down
- Continuations: Three White Soldiers, Three Black Crows
- Complex Formations: Dark Cloud Cover, Doji Star patterns

## 🚀 Getting Started

```bash
cargo add candlestick-rs
```

## 💻 Usage Examples

### Basic Pattern Detection

```rust
use candlestick_rs::CandleStick;

// Simple tuple representation: (open, high, low, close, volume)
let candle = (100.0, 105.0, 99.0, 101.0, 0.0);

// Check for patterns
if candle.is_hammer() {
    println!("Potential bullish reversal detected!");
} else if candle.is_doji() {
    println!("Market indecision detected!");
}
```

### Multi-Candle Pattern Analysis

```rust
use candlestick_rs::{CandleStick, CandleStream};

// Create candles (open, high, low, close, volume)
let candle1 = (100.0, 105.0, 99.0, 101.0, 0.0);  // Day 1
let candle2 = (102.0, 110.0, 101.5, 109.5, 0.0); // Day 2

// Create a stream and add candles
let mut stream = CandleStream::new();
stream.push(&candle1).push(&candle2);

// Check for patterns
if stream.is_bullish_engulfing() {
    println!("Strong buy signal detected!");
}
```

### Custom Data Structures

```rust
use candlestick_rs::CandleStick;

struct MyCandle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64
    // ... other fields (timestamp, etc.)
}

impl CandleStick for MyCandle {
    fn open(&self) -> f64 { self.open }
    fn high(&self) -> f64 { self.high }
    fn low(&self) -> f64 { self.low }
    fn close(&self) -> f64 { self.close }
    fn volume(&self) -> f64 { self.volume }
}

// Now you can use all pattern detection methods!
let my_candle = MyCandle { open: 100.0, high: 105.0, low: 99.0, close: 103.0, volume: 1.0 };
if my_candle.is_bullish_marubozu() {
    println!("Strong bullish conviction detected!");
}
```

## 📈 For Traders

This library follows traditional Japanese candlestick pattern definitions and provides detailed context on each pattern's trading significance. Pattern detection is based on mathematically sound ratios that can be customized when needed.

All pattern detection methods include comprehensive documentation explaining:
- Pattern formation characteristics
- Market psychology behind each pattern
- Typical trading signals and significance
- Context where patterns are most reliable

## 📚 Documentation

For complete API documentation, examples, and pattern explanations, visit:
[https://docs.rs/candlestick-rs](https://docs.rs/candlestick-rs)

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

<div align="center">

### Disclaimer

*This software is for informational purposes only. It is not intended as trading or investment advice.*

</div>
