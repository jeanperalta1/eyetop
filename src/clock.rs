// Time keeping

use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Instant;

use chrono::{Local, TimeZone};

pub type Timestamp = i64;

pub const SECOND: i64 = 1_000;
pub const MINUTE: i64 = 60 * SECOND;
pub const HOUR: i64 = 60 * MINUTE;
pub const DAY: i64 = 24 * HOUR;

// Real world clock
pub fn wall_clock_now() -> Timestamp {
    chrono::Utc::now().timestamp_millis()
}

// Shareable source of "now"
#[derive(Clone)]
pub struct Clock {
    kind: Arc<ClockKind>,
}

enum ClockKind {
    // Read Time
    Real,
    // TIme that runs a bit faster than real time (you dont say)
    SpedUp {
        started: Instant,
        started_at: Timestamp,
        speed: f64,
    },
    // Time that only move when Clock::set is called
    Manual(AtomicI64),
}
