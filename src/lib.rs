//! Export [flame] data to [speedscope]'s profile format.
//!
//! Base profile's are derived from top-level spans, so it's recommended to ensure that you have
//! an appropriately named span that encompasses the entire run of the program.
//!
//! [flame]: https://github.com/TyOverby/flame
//! [speedscope]: https://www.speedscope.app/

#[macro_use]
extern crate serde;

pub mod model;

use flame::Span;
use indexmap::IndexSet;
use model::*;
use std::io::Write;

const JSON_SCHEMA_URL: &str = "https://www.speedscope.app/file-format-schema.json";

/// Convert flame spans to the speedscope profile format.
pub fn spans_to_speedscope(spans: Vec<Span>) -> SpeedscopeFile {
    let mut frames = IndexSet::new();
    let profiles = spans
        .into_iter()
        .map(|span| Profile::Evented {
            name: span.name.clone(),
            unit: ValueUnit::Nanoseconds,
            start_value: span.start_ns,
            end_value: span.end_ns,
            events: {
                let mut events = Vec::new();
                span_extend_events(&mut frames, &mut events, span);
                events
            },
        })
        .collect();
    SpeedscopeFile {
        // always the same
        schema: JSON_SCHEMA_URL,
        active_profile_index: None,
        exporter: None,
        name: None,
        profiles,
        shared: Shared {
            frames: frames.into_iter().collect(),
        },
    }
}

fn span_extend_events(frames: &mut IndexSet<Frame>, events: &mut Vec<Event>, span: Span) {
    let (frame, _) = frames.insert_full(Frame::new(span.name));
    events.push(Event {
        event_type: EventType::OpenFrame,
        at: span.start_ns,
        frame,
    });
    for child in span.children {
        span_extend_events(frames, events, child);
    }
    events.push(Event {
        event_type: EventType::CloseFrame,
        at: span.end_ns,
        frame,
    });
}

#[inline]
/// Dump flame spans to a writer, e.g. a file.
/// ```no_run
/// # use std::fs::File;
/// flamescope::dump(&mut File::create("flamescope.json").unwrap()).unwrap();
/// ```
pub fn dump(writer: impl Write) -> serde_json::Result<()> {
    write_spans(writer, flame::spans())
}

#[inline]
/// Dump flame spans to a writer, e.g. a file, specifying the spans you want to write.
/// ```no_run
/// # use std::fs::File;
/// flamescope::write_spans(
///     &mut File::create("flamescope.json").unwrap(),
///     // if you wanted only every other span, for some reason.
///     flame::spans()
///         .into_iter()
///         .enumerate()
///         .filter(|(i, _)| i % 2 == 0)
///         .map(|(_, span)| span)
///         .collect(),
/// ).unwrap();
/// ```
pub fn write_spans(writer: impl Write, spans: Vec<Span>) -> serde_json::Result<()> {
    let speedscope = spans_to_speedscope(spans);
    serde_json::to_writer(writer, &speedscope)
}
