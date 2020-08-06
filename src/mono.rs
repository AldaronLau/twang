// Copyright (c) 2018-2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, or the ZLib
// license <LICENSE-ZLIB or https://www.zlib.net/zlib_license.html> at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Mono speaker configuration and types.

use crate::{
    chan::{Ch16, Ch32, Ch64, Ch8},
    sample::Sample1,
    Config,
};

/// 1 speaker/channel arrangement (front center)
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Mono;

impl Config for Mono {
    const CHANNEL_COUNT: usize = 1;
}

/// [Mono](struct.Mono.html) [8-bit PCM](../chan/struct.Ch8.html) format.
pub type Mono8 = Sample1<Ch8, Mono>;
/// [Mono](struct.Mono.html) [16-bit PCM](../chan/struct.Ch16.html) format.
pub type Mono16 = Sample1<Ch16, Mono>;
/// [Mono](struct.Mono.html)
/// [32-bit Floating Point](../chan/struct.Ch32.html) format.
pub type Mono32 = Sample1<Ch32, Mono>;
/// [Mono](struct.Mono.html)
/// [64-bit Floating Point](../chan/struct.Ch64.html) format.
pub type Mono64 = Sample1<Ch64, Mono>;
