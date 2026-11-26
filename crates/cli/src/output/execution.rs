use aoc_core::PartResult;

pub fn print_day(year: usize, day: usize, results: &[PartResult]) {
    println!("\n🎄 Advent of Code {year} · Day {day:02}");
    println!("────────────────────────────────────────");

    for result in results {
        println!(
            "  Part {} │ {:<20} │ {:>10.3?}",
            result.part, result.answer, result.duration,
        );
    }

    println!("────────────────────────────────────────\n");
}
