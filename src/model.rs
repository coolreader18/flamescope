/*
 * Modified from speedscope.rs in rbspy:
 * https://github.com/rbspy/rbspy/blob/master/src/ui/speedscope.rs
 *
 * MIT License
 *
 * Copyright (c) 2016 Julia Evans, Kamal Marhubi
 * Portions (continuous integration setup) Copyright (c) 2016 Jorge Aparicio
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

type StrCow = std::borrow::Cow<'static, str>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeedscopeFile {
    #[serde(rename = "$schema")]
    pub schema: &'static str,

    pub profiles: Vec<Profile>,
    pub shared: Shared,

    #[serde(rename = "activeProfileIndex")]
    pub active_profile_index: Option<u64>,

    pub exporter: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Profile {
    #[serde(rename = "sampled")]
    Sampled {
        name: StrCow,
        unit: ValueUnit,
        #[serde(rename = "startValue")]
        start_value: u64,
        #[serde(rename = "endValue")]
        end_value: u64,
        samples: Vec<SampledStack>,
        weights: Vec<u64>,
    },
    #[serde(rename = "evented")]
    Evented {
        name: StrCow,
        unit: ValueUnit,
        #[serde(rename = "startValue")]
        start_value: u64,
        #[serde(rename = "endValue")]
        end_value: u64,
        events: Vec<Event>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub at: u64,
    pub frame: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "O")]
    OpenFrame,
    #[serde(rename = "C")]
    CloseFrame,
}

type SampledStack = Vec<usize>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Shared {
    pub frames: Vec<Frame>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct Frame {
    pub name: StrCow,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub col: Option<u32>,
}

impl Frame {
    #[inline]
    pub fn new(name: StrCow) -> Frame {
        Frame {
            name,
            file: None,
            line: None,
            col: None,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ValueUnit {
    #[serde(rename = "bytes")]
    Bytes,
    #[serde(rename = "microseconds")]
    Microseconds,
    #[serde(rename = "milliseconds")]
    Milliseconds,
    #[serde(rename = "nanoseconds")]
    Nanoseconds,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "seconds")]
    Seconds,
}
