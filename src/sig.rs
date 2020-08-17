// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Digital audio signal.

use fon::{
    chan::{Ch64, Channel},
    mono::Mono,
    sample::Sample1,
};
use std::f64::consts::PI;

/// A signed digital audio signal that can be routed through processing
/// components.  This differs from `Mono64` in that the values are not clamped
/// between -1 and 1.
#[derive(Copy, Clone, Debug)]
pub struct Signal(f64);

impl Signal {
    /// Sine wave generator component - takes a sawtooth (`Fc`) wave.
    #[inline]
    pub fn sine(self) -> Self {
        Self((self.0 * PI).cos())
    }

    /// Triangle wave generator component - takes a sawtooth (`Fc`) wave.
    #[inline]
    pub fn triangle(self) -> Self {
        Self(self.0.abs() * 2.0 - 1.0)
    }

    /// Pulse wave generator component - takes a sawtooth (`Fc`) wave.
    /// - `half_duty`: ½ Duty cycle - range: 0~1 (1.0 for square wave)
    #[inline]
    pub fn pulse(self, half_duty: f64) -> Self {
        let phase_shifted = self.shift(half_duty);
        Self((self.0 - phase_shifted.0).signum())
    }

    /// Shift signal.  Takes a signal and adds an amount to it, wrapping to -1
    /// if it goes over 1, and to 1 if it goes under -1.
    #[inline]
    pub fn shift(self, amount: f64) -> Self {
        match (self.0 + amount) % 2.0 {
            x if x < -1.0 => Self(x + 2.0),
            x if x > 1.0 => Self(x - 2.0),
            x => Self(x),
        }
    }

    /// Amplify signal.
    #[inline]
    pub fn amp(self, volume: f64) -> Self {
        Self(self.0 * volume)
    }

    /// Invert (negate) signal.
    #[inline]
    pub fn invert(self) -> Self {
        Self(-self.0)
    }

    /// Absolute value of signal.
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// The minimum of two signals.
    #[inline]
    pub fn min<I: Into<Self>>(self, limit: I) -> Self {
        Self(self.0.min(limit.into().0))
    }

    /// The maximum of two signals.
    #[inline]
    pub fn max<I: Into<Self>>(self, limit: I) -> Self {
        Self(self.0.min(limit.into().0))
    }

    /// Raise a signal to a power.  This can be used to get the `x` root of a
    /// signal as well with `1 / x`.
    #[inline]
    pub fn pow(self, exp: f64) -> Self {
        Self(self.0.powf(exp))
    }

    /// Amplify a signal with soft clipping.
    #[inline]
    pub fn clip_soft(self, volume: f64) -> Self {
        Self(
            (2.0 / (1.0 + (self.0 * -volume).exp()) - 1.0)
                / (2.0 / (1.0 + (-volume).exp()) - 1.0),
        )
    }

    /// Clamp a signal -1 to 1 (hard clipping)
    #[inline]
    pub fn clamp(self) -> Self {
        self.min(1.0).max(-1.0)
    }

    /// Convert signal into Mono channel.
    #[inline]
    pub fn to_mono<Ch>(self) -> Sample1<Ch, Mono>
    where
        Ch: From<Ch64> + Channel,
    {
        let ch: Ch = Ch64::new(self.0.min(1.0).max(-1.0)).into();
        Sample1::new(ch)
    }
}

impl From<f64> for Signal {
    fn from(signal: f64) -> Signal {
        Signal(signal)
    }
}

impl From<Signal> for f64 {
    fn from(signal: Signal) -> f64 {
        signal.0
    }
}