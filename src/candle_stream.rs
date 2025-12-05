use crate::{utils::midpoint, CandleStick};

const SERIES_SIZE: usize = 5;

/// The `CandleStream` provides detection capabilities for powerful multi-candle patterns
///
/// - **Reversal Patterns**: Engulfing, Harami, Morning/Evening Stars, Doji Stars
/// - **Continuation Patterns**: Three White Soldiers, Three Black Crows
/// - **Top/Bottom Formations**: Dark Cloud Cover and other significant reversal signals
///
/// These formations often provide stronger trading signals than single-candle patterns,
/// offering insights into potential trend reversals, continuations, or exhaustion points.
/// Each pattern detection method includes detailed documentation about market context
/// and trading significance.
///
/// # Examples
///
/// ```
/// use candlestick_rs::{CandleStick, CandleStream};
///
/// // Create a new stream and add candles
/// let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
/// let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
///
/// let mut stream = CandleStream::new();
/// stream.push(&candle1).push(&candle2);
///
/// // Check for patterns
/// if stream.is_bullish_engulfing() {
///     println!("Bullish engulfing pattern detected!");
/// }
/// ```

#[derive(Debug)]
pub struct CandleStream<'s, T> {
    series: [Option<&'s T>; SERIES_SIZE],
    idx: usize,
}

impl<'s, T> CandleStream<'s, T> {
    /// Returns a new candle series
    pub fn new() -> Self {
        Self::default()
    }

    // Returns the index of the nth last candle
    fn nth_index(&self, n: usize) -> Option<usize> {
        if n > SERIES_SIZE {
            return None;
        }

        Some((self.idx + SERIES_SIZE - n) % SERIES_SIZE)
    }

    // Returns the candle at the given index
    fn at(&self, idx: usize) -> Option<&T> {
        match idx < SERIES_SIZE {
            true => self.series[idx],
            false => None,
        }
    }

    // Fetches reference to the current candle
    fn get(&self) -> Option<&T> {
        self.at(self.nth_index(1)?)
    }

    // Returns the previous candle
    fn prev(&self, n: usize) -> Option<&T> {
        self.at(self.nth_index(n + 1)?)
    }

    /// Pushes a candle to the series
    pub fn push(&mut self, candle: &'s T) -> &mut Self {
        self.series[self.idx % SERIES_SIZE] = Some(candle);
        self.idx = (self.idx + 1) % SERIES_SIZE;
        self
    }
}

impl<T: CandleStick> CandleStream<'_, T> {
    /// Identifies a Bullish Doji Star pattern, a potential reversal signal in downtrends.
    ///
    /// This two-candle pattern occurs when a bearish candle is followed by a Doji that gaps below
    /// the prior candle's low. The Doji represents market indecision after a dominant downtrend.
    ///
    /// **Trading Significance**:
    /// - Signals potential exhaustion of selling pressure
    /// - Often precedes bullish price movements when confirmed
    /// - Traders typically wait for a third bullish candle before entering long positions
    /// - Most effective when appearing at support levels or after extended downtrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (52.0, 52.5, 48.0, 48.5, 0.0);      
    /// let curr = (47.0, 47.5, 46.8, 47.0, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_doji_star());
    /// ```
    pub fn is_bullish_doji_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .is_some_and(|(c, p)| p.is_bearish() && c.is_doji() && c.high() < p.low())
    }

    /// Identifies a Bearish Doji Star pattern, a potential reversal signal in uptrends.
    ///
    /// This two-candle pattern occurs when a bullish candle is followed by a Doji that gaps above
    /// the prior candle's high. The Doji represents market indecision after a dominant uptrend.
    ///
    /// **Trading Significance**:
    /// - Signals potential exhaustion of buying pressure
    /// - Often precedes bearish price movements when confirmed
    /// - Traders typically wait for a third bearish candle before entering short positions
    /// - Most effective when appearing at resistance levels or after extended uptrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (48.0, 52.5, 47.8, 52.0, 0.0);
    /// let curr = (52.6, 53.2, 52.6, 52.6, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_doji_star());
    /// ```
    pub fn is_bearish_doji_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .is_some_and(|(c, p)| p.is_bullish() && c.is_doji() && c.low() > p.high())
    }

    ///
    /// Identifies a Bullish Engulfing pattern, a strong reversal signal at the end of downtrends.
    ///
    /// This two-candle pattern occurs when a bearish candle is completely engulfed by a larger bullish candle
    /// (open lower than prior close, close higher than prior open). It shows buyers overwhelmingly defeating sellers.
    ///
    /// **Trading Significance**:
    /// - Indicates strong shift from selling to buying pressure
    /// - More reliable than single-candle patterns due to the decisive price action
    /// - Often used as an immediate entry signal, especially when volume increases
    /// - Higher reliability when occurring at support zones or after extended downtrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (101.0, 102.0, 99.5, 100.5, 0.0); // bearish: open > close
    /// let curr = (99.0, 103.0, 98.5, 102.5, 0.0);  // bullish: open < close, engulfs prev body
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_engulfing());
    /// ```
    pub fn is_bullish_engulfing(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bearish() && c.is_bullish() && c.open() < p.close() && c.close() > p.open()
        })
    }

    /// Identifies a Bearish Engulfing pattern, a strong reversal signal at the end of uptrends.
    ///
    /// This two-candle pattern occurs when a bullish candle is completely engulfed by a larger bearish candle
    /// (open higher than prior close, close lower than prior open). It shows sellers overwhelmingly defeating buyers.
    ///
    /// **Trading Significance**:
    /// - Indicates strong shift from buying to selling pressure
    /// - More reliable than single-candle patterns due to the decisive price action
    /// - Often used as an immediate exit signal for longs or entry for shorts
    /// - Higher reliability when occurring at resistance zones or after extended uptrends
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (99.0, 100.5, 98.5, 100.0, 0.0);  // bullish: open < close
    /// let curr = (101.5, 102.0, 97.0, 98.5, 0.0);  // bearish: open > close, engulfs prev body
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_engulfing());
    /// ```
    pub fn is_bearish_engulfing(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bullish() && c.is_bearish() && c.open() > p.close() && c.close() < p.open()
        })
    }

    /// Identifies a Bullish Harami pattern, indicating potential reversal or continuation in downtrends.
    ///
    /// This two-candle pattern occurs when a small bullish candle is contained within the trading range of a
    /// preceding larger bearish candle. The Japanese word "harami" means pregnant, describing the visual appearance.
    ///
    /// **Trading Significance**:
    /// - Signals indecision after a bearish move and possible loss of downward momentum
    /// - Less powerful than engulfing patterns but still a notable reversal signal
    /// - Traders typically wait for additional confirmation before entering long positions
    /// - Part of contingent trading strategies where position size increases after confirmation
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (129.0, 130.0, 124.0, 125.0, 0.0);
    /// let curr = (125.2, 127.0, 124.8, 126.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bullish_harami());
    /// ```
    pub fn is_bullish_harami(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bearish() && c.is_bullish() && c.open() > p.close() && c.close() < p.open()
        })
    }

    /// Identifies a Bearish Harami pattern, indicating potential reversal or continuation in uptrends.
    ///
    /// This two-candle pattern occurs when a small bearish candle is contained within the trading range of a
    /// preceding larger bullish candle. The Japanese word "harami" means pregnant, describing the visual appearance.
    ///
    /// **Trading Significance**:
    /// - Signals indecision after a bullish move and possible loss of upward momentum
    /// - Less powerful than engulfing patterns but still a notable reversal warning
    /// - Often used to protect profits on long positions or tighten stop losses
    /// - Sometimes precedes a period of consolidation rather than immediate reversal
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (124.0, 129.0, 122.0, 127.0, 0.0);
    /// let curr = (126.9, 129.7, 125.0, 124.8, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_bearish_harami());
    /// ```
    pub fn is_bearish_harami(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            p.is_bullish() && c.is_bearish() && c.open() < p.close() && c.close() > p.open()
        })
    }

    /// Identifies a Dark Cloud Cover pattern, a bearish reversal signal in uptrends.
    ///
    /// This two-candle pattern occurs when a bearish candle opens above the prior bullish candle's close
    /// but closes below the midpoint of the prior candle's body. It shows rejection of higher prices.
    ///
    /// **Trading Significance**:
    /// - Signals strong selling pressure after an uptrend
    /// - More significant when the bearish candle closes deep into the prior bullish candle
    /// - Often used by traders to exit long positions or initiate short positions
    /// - Particularly effective when appearing at historical resistance levels
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev = (100.0, 105.0, 99.5, 104.5, 0.0);
    /// let curr = (105.5, 106.0, 102.0, 101.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev).push(&curr).is_dark_cloud_cover());
    /// ```
    pub fn is_dark_cloud_cover(&self) -> bool {
        self.get().zip(self.prev(1)).is_some_and(|(c, p)| {
            c.is_bearish()
                && p.is_bullish()
                && c.open() > p.close()
                && c.close() < midpoint(p.open(), p.close())
        })
    }

    /// Identifies an Evening Star pattern, a bearish reversal formation at market tops.
    ///
    /// This three-candle pattern consists of:
    /// 1. A strong bullish candle extending the uptrend
    /// 2. A small-bodied candle showing indecision (star), often with a gap
    /// 3. A bearish candle closing well into the first candle's body
    ///
    /// **Trading Significance**:
    /// - Represents a complete shift from bullish to bearish sentiment
    /// - Considered one of the most reliable bearish reversal patterns
    /// - Traders often exit longs or enter shorts when the third candle confirms
    /// - Effectiveness increases with the size of the third bearish candle
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (100.0, 106.0, 99.5, 105.5, 0.0);
    /// let prev1 = (106.2, 107.0, 105.8, 106.5, 0.0);
    /// let curr = (105.5, 106.0, 102.0, 101.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_evening_star());
    /// ```
    pub fn is_evening_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && (p1.is_doji() || p1.open() < p1.close())
                    && c.is_bearish()
                    && c.close() < midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies an Evening Star Doji variant, a strong bearish reversal pattern at market tops.
    ///
    /// This three-candle pattern is similar to the Evening Star, but the middle candle is specifically
    /// a Doji (open ≈ close), emphasizing the perfect equilibrium between buyers and sellers before
    /// bears take control.
    ///
    /// **Trading Significance**:
    /// - Considered stronger than the standard Evening Star due to the Doji's stronger indecision signal
    /// - Often precedes significant price declines when confirmed by the third candle
    /// - Used by traders as a high-probability signal to exit long positions
    /// - Particularly powerful when occurring after an extended uptrend with high momentum
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 =  (100.0, 106.0, 99.5, 105.5, 0.0);
    /// let prev1 =  (106.1, 107.0, 105.8, 106.1, 0.0);
    /// let curr = (105.0, 105.2, 99.8, 101.0, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_evening_star_doji());
    /// ```
    pub fn is_evening_star_doji(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_doji() & c.is_bearish()
                    && c.close() < midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies a Morning Star pattern, a bullish reversal formation at market bottoms.
    ///
    /// This three-candle pattern consists of:
    /// 1. A strong bearish candle extending the downtrend
    /// 2. A small-bodied candle showing indecision (star), often with a gap
    /// 3. A bullish candle closing well into the first candle's body
    ///
    /// **Trading Significance**:
    /// - Represents a complete shift from bearish to bullish sentiment
    /// - Considered one of the most reliable bullish reversal patterns
    /// - Traders often enter long positions when the third candle confirms
    /// - Effectiveness increases with the size of the third bullish candle and supporting volume
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (52.0, 52.5, 48.0, 48.5, 0.0);
    /// let prev1 = (48.2, 48.9, 47.5, 48.3, 0.0);
    /// let curr = (48.7, 51.5, 48.5, 51.2, 0.0);   
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_morning_star());
    /// ```
    pub fn is_morning_star(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && (p1.is_doji() || p1.open() < p1.close())
                    && c.is_bullish()
                    && c.close() > midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies a Morning Star Doji variant, a strong bullish reversal pattern at market bottoms.
    ///
    /// This three-candle pattern is similar to the Morning Star, but the middle candle is specifically
    /// a Doji (open ≈ close), emphasizing the perfect equilibrium between buyers and sellers before
    /// bulls take control.
    ///
    /// **Trading Significance**:
    /// - Considered stronger than the standard Morning Star due to the Doji's stronger indecision signal
    /// - Often precedes significant price rallies when confirmed by the third candle
    /// - Used by traders as a high-probability entry point for long positions
    /// - Particularly powerful when occurring at support levels with increasing volume
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (52.0, 52.5, 48.0, 48.5, 0.0);
    /// let prev1 = (48.3, 48.9, 47.5, 48.4, 0.0);
    /// let curr =  (48.7, 51.5, 48.5, 51.2, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_morning_star_doji());
    /// ```
    pub fn is_morning_star_doji(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_doji()
                    && c.is_bullish()
                    && c.close() > midpoint(p2.open(), p2.close())
            })
    }

    /// Identifies Three White Soldiers, a powerful bullish reversal or continuation pattern.
    ///
    /// This three-candle pattern consists of consecutive bullish candles, each opening within the previous
    /// candle's body and closing higher, creating a stair-step appearance. Each candle shows progressively
    /// stronger buying pressure overtaking sellers.
    ///
    /// **Trading Significance**:
    /// - Indicates sustained buying pressure and strong bullish momentum
    /// - Shows buyers controlling the market over multiple time periods
    /// - Traders use it to confirm bullish trend reversals or continuations
    /// - Most reliable when candles have minimal upper shadows (little selling pressure at highs)
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (48.0, 50.5, 47.8, 50.2, 0.0);
    /// let prev1 = (50.3, 52.7, 50.1, 52.4, 0.0);
    /// let curr =  (52.5, 54.8, 52.3, 54.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_white_soldiers());
    /// ```
    pub fn is_three_white_soldiers(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_bullish()
                    && p1.open() > p2.close()
                    && p1.close() > p2.close()
                    && c.is_bullish()
                    && c.open() > p1.close()
                    && c.close() > p1.close()
            })
    }

    /// Identifies Three Black Crows, a powerful bearish reversal or continuation pattern.
    ///
    /// This three-candle pattern consists of consecutive bearish candles, each opening within the previous
    /// candle's body and closing lower, creating a downward stair-step appearance. Each candle shows progressively
    /// stronger selling pressure overtaking buyers.
    ///
    /// **Trading Significance**:
    /// - Indicates sustained selling pressure and strong bearish momentum
    /// - Shows sellers controlling the market over multiple time periods
    /// - Traders use it to confirm bearish trend reversals or continuations
    /// - Most reliable when candles have minimal lower shadows (little buying pressure at lows)
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (54.0, 54.5, 51.8, 52.2, 0.0);
    /// let prev1 = (52.0, 52.3, 49.7, 50.4, 0.0);
    /// let curr =  (50.2, 50.5, 47.9, 48.3, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_black_crows());
    /// ```
    pub fn is_three_black_crows(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_bearish()
                    && p1.open() < p2.close()
                    && p1.close() < p2.close()
                    && c.is_bearish()
                    && c.open() < p1.close()
                    && c.close() < p1.close()
            })
    }

    /// Identifies the Three Inside Up pattern, a bullish reversal.
    ///
    /// This three-candle pattern typically appears in a downtrend and signals a potential
    /// shift from bearish to bullish momentum. This pattern can be seen as a confirmation or
    /// continuation of a bullish harami.
    ///
    /// **Trading Significance**:
    /// - Suggests that prior selling pressure is weakening
    /// - Indicates buyers are starting to regain control
    /// - Often used as an early sign of a bullish reversal after a down move
    /// - Considerably stronger when combined with support levels, volume confirmation,
    ///   or higher-timeframe confluence
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    ///
    /// let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
    /// let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
    /// let curr  = (52.9, 55.0, 52.7, 54.5, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_inside_up());
    /// ```
    pub fn is_three_inside_up(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bearish()
                    && p1.is_bullish()
                    && p1.open() > p2.close()
                    && p1.close() < p2.open()
                    && c.is_bullish()
                    && c.close() > p1.close()
                    && !c.is_doji()
            })
    }

    /// Identifies the Three Inside Down pattern, a bearish reversal.
    ///
    /// This three-candle pattern typically appears in an uptrend and signals a potential
    /// shift from bullish to bearish momentum. This pattern can be seen as a confirmation or
    /// continuation of a bearish harami.
    ///
    /// **Trading Significance**:
    /// - Suggests that prior buying pressure is weakening
    /// - Indicates sellers are starting to regain control
    /// - Often used as an early sign of a bearish reversal after an up move
    /// - Considerably stronger when combined with resistance levels, volume confirmation,
    ///   or higher-timeframe confluence
    ///
    /// # Example
    /// ```
    /// use candlestick_rs::CandleStream;
    /// let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
    /// let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
    /// let curr  = (48.8, 49.0, 47.5, 47.9, 0.0);
    /// let mut series = CandleStream::new();
    /// assert!(series.push(&prev2).push(&prev1).push(&curr).is_three_inside_down());
    /// ```
    pub fn is_three_inside_down(&self) -> bool {
        self.get()
            .zip(self.prev(1))
            .zip(self.prev(2))
            .is_some_and(|((c, p1), p2)| {
                p2.is_bullish()
                    && p1.is_bearish()
                    && p1.open() < p2.close()
                    && p1.close() > p2.open()
                    && c.is_bearish()
                    && c.close() < p1.close()
                    && !c.is_doji()
            })
    }
}

impl<T> Default for CandleStream<'_, T> {
    fn default() -> Self {
        Self {
            series: [const { None }; SERIES_SIZE],
            idx: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_index() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let candle4 = (111.5, 115.0, 110.0, 114.0, 0.0);
        let candle5 = (114.5, 118.0, 113.0, 117.0, 0.0);
        let candle6 = (117.5, 120.0, 116.0, 119.0, 0.0);

        let mut stream = CandleStream::new();

        assert_eq!(stream.nth_index(6), None);

        stream.push(&candle1).push(&candle2);

        assert_eq!(stream.nth_index(0), Some(2));
        assert_eq!(stream.nth_index(1), Some(1));
        assert_eq!(stream.nth_index(2), Some(0));

        stream.push(&candle3).push(&candle4).push(&candle5);

        assert_eq!(stream.nth_index(0), Some(0));
        assert_eq!(stream.nth_index(1), Some(4));
        assert_eq!(stream.nth_index(2), Some(3));
        assert_eq!(stream.nth_index(3), Some(2));
        assert_eq!(stream.nth_index(4), Some(1));
        assert_eq!(stream.nth_index(5), Some(0));

        stream.push(&candle6);

        assert_eq!(stream.nth_index(0), Some(1));
        assert_eq!(stream.nth_index(1), Some(0));
        assert_eq!(stream.nth_index(2), Some(4));
        assert_eq!(stream.nth_index(3), Some(3));
        assert_eq!(stream.nth_index(4), Some(2));
        assert_eq!(stream.nth_index(5), Some(1));
    }

    #[test]
    fn test_at() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);
        let candle4 = (111.5, 115.0, 110.0, 114.0, 0.0);
        let candle5 = (114.5, 118.0, 113.0, 117.0, 0.0);
        let candle6 = (117.5, 120.0, 116.0, 119.0, 0.0);

        let mut stream = CandleStream::new();
        stream.push(&candle1).push(&candle2);

        assert_eq!(stream.at(0), Some(&candle1));
        assert_eq!(stream.at(1), Some(&candle2));
        assert_eq!(stream.at(2), None);

        stream.push(&candle3).push(&candle4).push(&candle5);

        assert_eq!(stream.at(0), Some(&candle1));
        assert_eq!(stream.at(1), Some(&candle2));
        assert_eq!(stream.at(2), Some(&candle3));
        assert_eq!(stream.at(3), Some(&candle4));
        assert_eq!(stream.at(4), Some(&candle5));

        stream.push(&candle6);

        assert_eq!(stream.at(0), Some(&candle6));
        assert_eq!(stream.at(1), Some(&candle2));
        assert_eq!(stream.at(2), Some(&candle3));
        assert_eq!(stream.at(3), Some(&candle4));
        assert_eq!(stream.at(4), Some(&candle5));
    }

    #[test]
    fn test_get() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);

        let mut stream = CandleStream::new();
        assert_eq!(stream.get(), None);

        stream.push(&candle1);
        assert_eq!(stream.get(), Some(&candle1));

        stream.push(&candle2);
        assert_eq!(stream.get(), Some(&candle2));

        stream.push(&candle3).push(&candle1).push(&candle2);
        assert_eq!(stream.get(), Some(&candle2));

        stream.push(&candle3);
        assert_eq!(stream.get(), Some(&candle3));
    }

    #[test]
    fn test_prev() {
        let candle1 = (100.0, 105.0, 99.0, 104.0, 0.0);
        let candle2 = (104.5, 110.0, 104.0, 109.0, 0.0);
        let candle3 = (109.5, 112.0, 108.0, 111.0, 0.0);

        let mut stream = CandleStream::new();
        assert_eq!(stream.prev(1), None);

        stream.push(&candle1);
        assert_eq!(stream.prev(1), None);

        stream.push(&candle2);
        assert_eq!(stream.prev(1), Some(&candle1));

        stream.push(&candle3);
        assert_eq!(stream.prev(1), Some(&candle2));
        assert_eq!(stream.prev(2), Some(&candle1));
    }

    #[test]
    fn test_is_three_inside_up() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_three_inside_up_if_curr_engulfs_prev1() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr_engulf_prev1 = (52.0, 55.0, 51.9, 53.5, 0.0);

        let mut series = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_engulf_prev1)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_curr_is_doji() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let doji = (53.4, 55.0, 52.7, 53.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&doji)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev2_not_bearish() {
        let not_bearish_prev2 = (52.0, 54.5, 51.8, 54.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0); // valid curr

        let mut series = CandleStream::new();

        assert!(!series
            .push(&not_bearish_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev2_is_doji() {
        let doji_prev2 = (53.0, 54.5, 51.8, 53.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&doji_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_not_bullish() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let not_bullish_prev1 = (52.8, 53.0, 52.0, 52.2, 0.0); // open > close
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&not_bullish_prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_opens_below_prev2_close() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1_open_below_prev2 = (51.9, 53.0, 51.8, 52.5, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_open_below_prev2)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_closes_above_prev2_open() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1_close_above_prev2 = (52.2, 55.0, 52.0, 54.5, 0.0);
        let curr = (54.6, 56.0, 52.7, 55.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_close_above_prev2)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_engulfs_prev2() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1_engulf_prev2 = (51.5, 55.0, 51.0, 54.5, 0.0);
        let curr = (54.6, 56.0, 53.5, 55.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_engulf_prev2)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_prev1_is_doji() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let doji_prev1 = (52.8, 53.0, 52.0, 52.8, 0.0);
        let curr = (52.9, 55.0, 52.7, 54.5, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&doji_prev1)
            .push(&curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_curr_is_inside_prev1() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let curr_inside_prev1 = (52.3, 53.1, 52.1, 52.7, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_inside_prev1)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_if_curr_not_bullish() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);
        let not_bullish_curr = (55.0, 55.5, 52.7, 53.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&not_bullish_curr)
            .is_three_inside_up());
    }

    #[test]
    fn test_is_not_three_inside_up_with_insufficient_candles() {
        let prev2 = (54.0, 54.5, 51.8, 52.0, 0.0);
        let prev1 = (52.2, 53.0, 52.0, 52.8, 0.0);

        let mut series = CandleStream::new();

        assert!(!series.push(&prev2).is_three_inside_up());
        assert!(!series.push(&prev1).is_three_inside_up());
    }

    #[test]
    fn test_is_three_inside_down() {
        let prev2: (f64, f64, f64, f64, f64) = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1: (f64, f64, f64, f64, f64) = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr: (f64, f64, f64, f64, f64) = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series: CandleStream<'_, (f64, f64, f64, f64, f64)> = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_three_inside_down_if_curr_engulfs_prev1() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr_engulf_prev1 = (49.8, 50.0, 47.5, 48.8, 0.0); // open > prev1.open, close < prev1.close

        let mut series = CandleStream::new();

        assert!(series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_engulf_prev1)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_curr_is_doji() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let doji = (48.5, 50.0, 47.5, 48.5, 0.0); // open == close

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&doji)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev2_not_bullish() {
        let not_bullish_prev2 = (50.0, 50.5, 47.8, 48.0, 0.0); // bearish instead of bullish
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0); // valid curr

        let mut series = CandleStream::new();

        assert!(!series
            .push(&not_bullish_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev2_is_doji() {
        let doji_prev2 = (49.0, 50.5, 47.8, 49.0, 0.0); // open == close
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&doji_prev2)
            .push(&prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_not_bearish() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let not_bearish_prev1 = (48.5, 49.5, 48.0, 49.2, 0.0); // open < close
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&not_bearish_prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_opens_above_prev2_close() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1_open_above_prev2 = (50.2, 50.5, 48.5, 49.5, 0.0);
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_open_above_prev2)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_closes_below_prev2_open() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1_close_below_prev2 = (49.5, 49.8, 47.5, 47.9, 0.0); // close < 48.0
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_close_below_prev2)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_engulfs_prev2() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0); // body [48.0, 50.0]
        let prev1_engulf_prev2 = (50.5, 51.0, 47.0, 47.5, 0.0); // open > 50.0, close < 48.0
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1_engulf_prev2)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_prev1_is_doji() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let doji_prev1 = (49.0, 49.5, 48.5, 49.0, 0.0); // open == close
        let curr = (48.8, 49.0, 47.5, 47.9, 0.0);

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&doji_prev1)
            .push(&curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_curr_is_inside_prev1() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0); // body [49.0, 49.5]
        let curr_inside_prev1 = (49.4, 49.6, 48.8, 49.1, 0.0); // close 49.1 > 49.0

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&curr_inside_prev1)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_if_curr_not_bearish() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);
        let not_bearish_curr = (47.8, 48.5, 47.5, 48.6, 0.0); // bullish

        let mut series = CandleStream::new();

        assert!(!series
            .push(&prev2)
            .push(&prev1)
            .push(&not_bearish_curr)
            .is_three_inside_down());
    }

    #[test]
    fn test_is_not_three_inside_down_with_insufficient_candles() {
        let prev2 = (48.0, 50.5, 47.8, 50.0, 0.0);
        let prev1 = (49.5, 49.8, 48.5, 49.0, 0.0);

        let mut series = CandleStream::new();

        assert!(!series.push(&prev2).is_three_inside_down());
        assert!(!series.push(&prev1).is_three_inside_down());
    }
}
