use crate::{Part, PartResult};
use std::time::Instant;

pub fn timed(part: Part, task: impl FnOnce() -> String) -> PartResult {
    let start = Instant::now();
    let answer = task();
    let duration = start.elapsed();
    PartResult {
        part,
        answer,
        duration,
    }
}
