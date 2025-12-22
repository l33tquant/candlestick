/// The `CandleStick` trait provides analytical capabilities to detect key single-candle
/// formations that signal potential market reversals, continuations, or indecision.
///
/// These patterns form the foundation of candlestick chart analysis, a technique
/// developed in 18th century Japan and widely used in modern technical trading.
///
/// Each pattern detection method (`is_*`) returns whether the specific formation criteria
/// are met, along with detailed documentation on trading significance and application.
///
/// Implementers need to provide only the four basic OHLC methods (open, high, low, close),
/// and all pattern recognition capabilities become automatically available.
///
/// For multi-candle pattern identification across a series of candles, use: [`crate::CandleStream`]
pub trait CandleStick {
    /// Hammer body to range ratio for both hammer and inverse hammer pattern.
    /// Can be overridden for custom ratio.
    ///
    /// Default: __30%__
    fn hammer_body_ratio(&self) -> f64 {
        0.3
    }

    /// Hammer upper shadow or wick to range ratio. Also, inversely used as tail to range ratio for inverse hammer pattern.
    /// Can be overridden for custom ratio
    ///
    /// Default: __20%__
    fn hammer_wick_ratio(&self) -> f64 {
        0.2
    }

    /// Hammer lower shadow or tail to range ratio. Also, inversely used as wick to range ratio for inverse hammer pattern.
    /// Can be overridden for custom ratio
    ///
    /// Default: __60%__
    fn hammer_tail_ratio(&self) -> f64 {
        0.6
    }

    /// Spinning top body ratio. Can be overridden for custom ratio
    ///
    /// Default: __20%__
    fn spinning_top_body_ratio(&self) -> f64 {
        0.2
    }

    /// Spinning top shadow ratio. Can be overridden for custom ratio
    ///
    /// Default: __30%__
    fn spinning_top_shadow_ratio(&self) -> f64 {
        0.3
    }

    /// Doji body to range ratio. Can be overridden for custom ratio.
    ///
    /// Default: __10%__
    fn doji_body_ratio(&self) -> f64 {
        0.1
    }

    /// Doji long leg to range ratio. Can be overridden for custom ratio.
    ///
    /// Default: __30%__
    fn doji_long_leg_ratio(&self) -> f64 {
        0.3
    }

    /// Doji tail to range ratio. Can be overridden for custom ratio.
    ///
    /// Default: __30%__
    fn doji_tail_ratio(&self) -> f64 {
        0.3
    }

    /// Doji wick to range ratio. Can be overridden for custom ratio.
    ///
    /// Default: __30%__
    fn doji_wick_ratio(&self) -> f64 {
        0.3
    }

    /// Doji minimum ratio. Can be overridden for custom ratio.
    ///
    /// Default: __5%__
    fn doji_min_ratio(&self) -> f64 {
        0.05
    }

    /// Marubozu minimum ratio. Can be overridden for custom ratio.
    ///
    /// Default: __20%__
    fn marubozu_ratio(&self) -> f64 {
        0.2
    }

    /// Returns the open price
    fn open(&self) -> f64;

    /// Returns the high price
    fn high(&self) -> f64;

    /// Returns the low price
    fn low(&self) -> f64;

    /// Returns the close price
    fn close(&self) -> f64;

    /// Returns the volume
    fn volume(&self) -> f64;

    /// Helper function to return the OHLC tuple
    #[doc(hidden)]
    fn ohlc(&self) -> (f64, f64, f64, f64) {
        (self.open(), self.high(), self.low(), self.close())
    }

    /// Helper function to return the candle length with small epsilon
    #[doc(hidden)]
    fn range(&self) -> f64 {
        (self.high() - self.low()).max(0.001)
    }

    /// Helper function to return the candle wick length of the candle
    #[doc(hidden)]
    fn wick(&self) -> f64 {
        self.high() - self.open().max(self.close())
    }

    /// Helper function to return the candle body as the absolute difference between the open and close prices with small epsilon
    #[doc(hidden)]
    fn body(&self) -> f64 {
        ((self.open() - self.close()).abs()).max(0.0001)
    }

    /// Helper function to return the candle tail length
    #[doc(hidden)]
    fn tail(&self) -> f64 {
        self.open().min(self.close()) - self.low()
    }

    /// Helper function to return the candle's wick to range ratio
    #[doc(hidden)]
    fn wick_range_ratio(&self) -> f64 {
        self.wick() / self.range()
    }

    /// Helper function to return the candle's wick to body ratio
    #[doc(hidden)]
    fn wick_body_ratio(&self) -> f64 {
        self.wick() / self.body()
    }

    /// Helper function to return the candle's body to range ratio
    #[doc(hidden)]
    fn body_range_ratio(&self) -> f64 {
        self.body() / self.range()
    }

    /// Helper function to return the candle's tail to range ratio
    #[doc(hidden)]
    fn tail_range_ratio(&self) -> f64 {
        self.tail() / self.range()
    }

    /// Helper function to return the candle's tail to body ratio
    #[doc(hidden)]
    fn tail_body_ratio(&self) -> f64 {
        self.tail() / self.body()
    }

    /// Identifies a Bullish Candlestick, a foundational pattern in price action analysis.
    ///
    /// This basic pattern forms when the closing price is higher than the opening price,
    /// creating a filled (often green/white) candle body. The length of the body indicates
    /// the strength of buying pressure during the period.
    ///
    /// **Trading Significance**:
    /// - Signals buying pressure and bullish sentiment in the market
    /// - Longer bullish bodies indicate stronger buying conviction
    /// - Series of bullish candles confirm uptrends, especially with higher highs and lows
    /// - Often used as confirmation for other technical signals in trend-following strategies
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 110.0, 99.0, 109.0, 0.0);
    /// assert!(candle.is_bullish());
    /// ```
    fn is_bullish(&self) -> bool {
        self.open() < self.close()
    }

    /// Identifies a Bearish Candlestick, a foundational pattern in price action analysis.
    ///
    /// This basic pattern forms when the closing price is lower than the opening price,
    /// creating a filled (often red/black) candle body. The length of the body indicates
    /// the strength of selling pressure during the period.
    ///
    /// **Trading Significance**:
    /// - Signals selling pressure and bearish sentiment in the market
    /// - Longer bearish bodies indicate stronger selling conviction
    /// - Series of bearish candles confirm downtrends, especially with lower highs and lows
    /// - Often used as confirmation for other technical signals in trend-following strategies
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (110.0, 111.0, 99.0, 100.0, 0.0);
    /// assert!(candle.is_bearish());
    /// ```
    fn is_bearish(&self) -> bool {
        self.open() > self.close()
    }

    /// Identifies a Marubozu pattern, one of the strongest single-candle signals.
    ///
    /// This pattern forms when a candle has virtually no upper or lower shadows (wicks),
    /// with the body extending across nearly the entire range. The term "marubozu" means
    /// "bald head" or "shaved head" in Japanese, referring to the absence of shadows.
    ///
    /// **Trading Significance**:
    /// - Represents complete dominance of either buyers (bullish marubozu) or sellers (bearish marubozu)
    /// - Signals exceptional conviction in the market direction
    /// - Often precedes continuation in the same direction, especially early in trends
    /// - When appearing against the prevailing trend, can signal potential exhaustion and reversal
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 110.0, 99.0, 109.0, 0.0);
    /// assert!(candle.is_marubozu());
    /// ```
    fn is_marubozu(&self) -> bool {
        self.wick_body_ratio() < self.marubozu_ratio()
            && self.tail_body_ratio() < self.marubozu_ratio()
    }

    /// Identifies a Bullish Marubozu, a powerful signal of buyer dominance.
    ///
    /// This pattern forms when a bullish candle (close > open) has virtually no shadows,
    /// with the open at or near the low and the close at or near the high. It shows
    /// buyers controlled the price action throughout the entire period with no significant
    /// selling pressure.
    ///
    /// **Trading Significance**:
    /// - Indicates exceptional buying pressure and momentum
    /// - Often signals the beginning or acceleration of an uptrend
    /// - Traders frequently use it as a strong entry signal for long positions
    /// - When appearing after a consolidation or pullback, suggests the resumption of a bullish trend
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 110.0, 99.0, 109.0, 0.0);
    /// assert!(candle.is_bullish_marubozu());
    /// ```
    fn is_bullish_marubozu(&self) -> bool {
        self.is_bullish() && self.is_marubozu()
    }

    /// Identifies a Bearish Marubozu, a powerful signal of seller dominance.
    ///
    /// This pattern forms when a bearish candle (close < open) has virtually no shadows,
    /// with the open at or near the high and the close at or near the low. It shows
    /// sellers controlled the price action throughout the entire period with no significant
    /// buying pressure.
    ///
    /// **Trading Significance**:
    /// - Indicates exceptional selling pressure and downward momentum
    /// - Often signals the beginning or acceleration of a downtrend
    /// - Traders frequently use it as a strong exit signal for long positions or entry for shorts
    /// - When appearing after an uptrend, can signal a potential trend reversal
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (110.0, 111.0, 99.0, 100.0, 0.0);
    /// assert!(candle.is_bearish_marubozu());
    /// ```
    fn is_bearish_marubozu(&self) -> bool {
        self.is_bearish() && self.is_marubozu()
    }

    /// Identifies a Hammer pattern, a significant bullish reversal signal.
    ///
    /// This single-candle pattern is characterized by a small body at the upper portion of the
    /// trading range and a long lower shadow (tail) at least twice the length of the body.
    /// It resembles a hammer with a handle at the top.
    ///
    /// **Trading Significance**:
    /// - Signals potential bullish reversal when appearing at the bottom of a downtrend
    /// - Indicates rejection of lower prices as buyers stepped in after initial selling
    /// - More reliable when followed by confirmation (a bullish candle or increased volume)
    /// - Body color is less important than the overall shape, though bullish hammers (close > open) are slightly more significant
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 101.0, 95.0, 100.8, 0.0);
    /// assert!(candle.is_hammer());
    /// ```
    fn is_hammer(&self) -> bool {
        self.body_range_ratio() < self.hammer_body_ratio()
            && self.wick_range_ratio() < self.hammer_wick_ratio()
            && self.tail_range_ratio() > self.hammer_tail_ratio()
    }

    /// Identifies an Inverted Hammer pattern, a potential bullish reversal signal.
    ///
    /// This single-candle pattern features a small body at the lower portion of the
    /// trading range and a long upper shadow (wick) at least twice the length of the body.
    /// It resembles an upside-down hammer.
    ///
    /// **Trading Significance**:
    /// - Signals potential bullish reversal when appearing at the bottom of a downtrend
    /// - Indicates attempted upside movement, though sellers pushed prices back down
    /// - Generally requires stronger confirmation than a standard hammer
    /// - Often precedes the end of a downtrend, especially when followed by bullish price action
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 104.0, 99.8, 100.5, 0.0);
    /// assert!(candle.is_inverted_hammer());
    /// ```
    fn is_inverted_hammer(&self) -> bool {
        self.body_range_ratio() < self.hammer_body_ratio()
            && self.wick_range_ratio() > self.hammer_tail_ratio()
            && self.tail_range_ratio() < self.hammer_wick_ratio()
    }

    /// Identifies a Hanging Man pattern, an important bearish reversal signal.
    ///
    /// This pattern has the same shape as a hammer (small body at the top with a long lower shadow),
    /// but appears during an uptrend. The long lower shadow indicates selling pressure that emerged
    /// but was overcome by buyers—a warning sign after an advance.
    ///
    /// **Trading Significance**:
    /// - Signals potential bearish reversal when appearing after an uptrend
    /// - Suggests market vulnerability as sellers tested lower prices
    /// - Most effective at resistance levels or after extended price advances
    /// - Traders typically wait for next-day confirmation (a bearish candle or gap down)
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (592.0, 593.75, 587.0, 593.0, 0.0);
    /// assert!(candle.is_hanging_man());
    /// ```
    fn is_hanging_man(&self) -> bool {
        self.is_hammer()
    }

    /// Identifies a Shooting Star pattern, a significant bearish reversal signal.
    ///
    /// This pattern has the same shape as an inverted hammer (small body at the bottom with a long upper shadow),
    /// but appears during an uptrend. The long upper shadow indicates rejection of higher prices,
    /// as buyers attempted to push prices up but ultimately failed.
    ///
    /// **Trading Significance**:
    /// - Signals potential bearish reversal when appearing after an uptrend
    /// - Indicates strong rejection of higher prices and exhaustion of buying pressure
    /// - More significant when the upper shadow is at least twice the length of the body
    /// - Often used by traders to exit long positions or initiate shorts, especially when confirmed
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 106.0, 99.7, 100.8, 0.0);
    /// assert!(candle.is_shooting_star());
    /// ```
    fn is_shooting_star(&self) -> bool {
        self.is_inverted_hammer()
    }

    /// Identifies a Spinning Top pattern, a signal of market indecision and equilibrium.
    ///
    /// This single-candle pattern features a small body centered within the trading range
    /// with relatively long upper and lower shadows of similar length. The small body shows
    /// little net movement from open to close despite the larger trading range.
    ///
    /// **Trading Significance**:
    /// - Indicates indecision and balance between buyers and sellers
    /// - Often appears during consolidation phases or at potential reversal points
    /// - Suggests weakening of the current trend when appearing after a strong directional move
    /// - By itself provides limited directional bias; traders use it as an alert for potential change
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 105.0, 95.0, 100.5, 0.0);
    /// assert!(candle.is_spinning_top());
    /// ```
    fn is_spinning_top(&self) -> bool {
        self.body_range_ratio() < self.spinning_top_body_ratio()
            && self.wick_range_ratio() > self.spinning_top_shadow_ratio()
            && self.tail_range_ratio() > self.spinning_top_shadow_ratio()
    }

    /// Identifies a Doji pattern, a powerful signal of market equilibrium and indecision.
    ///
    /// This pattern forms when the opening and closing prices are virtually the same,
    /// creating an extremely small or non-existent body with shadows extending above and below.
    /// The Japanese word "doji" means "mistake" or "blunder," referring to the rare equality of open and close.
    ///
    /// **Trading Significance**:
    /// - Represents perfect equilibrium between buyers and sellers
    /// - Signals potential trend reversal, especially after extended price moves
    /// - Indicates exhaustion of the prevailing trend and possible consolidation
    /// - Traders use it as an alert to reduce position size or tighten stops
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 105.0, 95.0, 100.0, 0.0);
    /// assert!(candle.is_doji());
    /// ```
    fn is_doji(&self) -> bool {
        self.body_range_ratio() < self.doji_body_ratio()
    }

    /// Identifies a Long-Legged Doji, a volatility-based signal of strong market indecision.
    ///
    /// This distinctive doji variant has unusually long upper and lower shadows compared to its minimal body.
    /// It shows significant price movement in both directions during the period, but ultimately
    /// closing near the opening price.
    ///
    /// **Trading Significance**:
    /// - Indicates extreme volatility and fierce battle between buyers and sellers
    /// - Frequently signals an impending trend change when appearing in extended trends
    /// - Represents exhaustion of the prevailing trend as neither side maintains control
    /// - Particularly significant when occurring at support/resistance levels or after strong directional moves
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 110.0, 90.0, 100.2, 0.0);
    /// assert!(candle.is_long_legged_doji());
    /// ```
    fn is_long_legged_doji(&self) -> bool {
        self.is_doji()
            && self.tail_range_ratio() > self.doji_long_leg_ratio()
            && self.wick_range_ratio() > self.doji_long_leg_ratio()
    }

    /// Identifies a Dragonfly Doji, a specialized pattern suggesting potential bullish reversal.
    ///
    /// This doji variant has its open and close at or near the high of the period,
    /// with virtually no upper shadow but a long lower shadow, creating a "T" shape.
    /// It shows sellers pushing prices down during the period, but buyers regained control by the close.
    ///
    /// **Trading Significance**:
    /// - Strong bullish reversal signal when appearing at the bottom of downtrends
    /// - Indicates rejection of lower prices and return to the opening level
    /// - More reliable when formed at key support levels or round numbers
    /// - When appearing at market tops, can act as a warning signal of waning momentum
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 100.5, 90.0, 100.1, 0.0);
    /// assert!(candle.is_dragonfly_doji());
    /// ```
    fn is_dragonfly_doji(&self) -> bool {
        self.is_doji()
            && self.tail_range_ratio() > self.doji_tail_ratio()
            && self.wick_range_ratio() < self.doji_min_ratio()
    }

    /// Identifies a Gravestone Doji, a specialized pattern suggesting potential bearish reversal.
    ///
    /// This doji variant has its open and close at or near the low of the period,
    /// with virtually no lower shadow but a long upper shadow, creating an inverted "T" shape.
    /// It shows buyers pushing prices up during the period, but sellers regained control by the close.
    ///
    /// **Trading Significance**:
    /// - Strong bearish reversal signal when appearing at the top of uptrends
    /// - Indicates rejection of higher prices and return to the opening level
    /// - Particularly ominous when formed at key resistance levels or after extended rallies
    /// - Named for its resemblance to a gravestone, suggesting the "death" of the current uptrend
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStick;
    /// let candle = (100.0, 110.0, 99.5, 100.1, 0.0);
    /// assert!(candle.is_gravestone_doji());
    /// ```
    fn is_gravestone_doji(&self) -> bool {
        self.is_doji()
            && self.wick_range_ratio() > self.doji_wick_ratio()
            && self.tail_range_ratio() < self.doji_min_ratio()
    }

    /// Summarizes the price action for the candle
    fn typical_price(&self) -> f64 {
        (self.high() + self.low() + self.close()) / 3.0
    }

    /// Flow of money into or out
    fn raw_money_flow(&self) -> f64 {
        self.typical_price() * self.volume()
    }
}

impl CandleStick for (f64, f64, f64, f64, f64) {
    fn open(&self) -> f64 {
        self.0
    }

    /// Returns the high price
    fn high(&self) -> f64 {
        self.1
    }

    /// Returns the low price
    fn low(&self) -> f64 {
        self.2
    }

    /// Returns the close price
    fn close(&self) -> f64 {
        self.3
    }

    /// Returns the volume
    fn volume(&self) -> f64 {
        self.4
    }
}

impl CandleStick for &(f64, f64, f64, f64, f64) {
    fn open(&self) -> f64 {
        self.0
    }

    /// Returns the high price
    fn high(&self) -> f64 {
        self.1
    }

    /// Returns the low price
    fn low(&self) -> f64 {
        self.2
    }

    /// Returns the close price
    fn close(&self) -> f64 {
        self.3
    }

    /// Returns the volume
    fn volume(&self) -> f64 {
        self.4
    }
}
