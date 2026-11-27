use aoc_core::PartResult;

use super::error::print_day_error;
use crate::report::execution::ExecutionReport;

const SEPARATOR: &str = "────────────────────────────────────────────────────────────";

pub fn print_report(report: &ExecutionReport) {
    if report.days.len() == 1 {
        let day = &report.days[0];

        match &day.outcome {
            Ok(results) => {
                print_day(day.year, day.day, results);
            }
            Err(error) => {
                print_day_error(day.year, day.day, error);
            }
        }

        return;
    }

    print_multiple_days(report);
    print_summary(report);
}

pub fn print_day(year: usize, day: usize, results: &[PartResult]) {
    println!("\n🎄 Advent of Code {year} · Day {day:02}");
    println!("{SEPARATOR}");

    for result in results {
        println!(
            "  Part {} │ {:<24} │ {:>12.3?}",
            result.part, result.answer, result.duration,
        );
    }

    println!("{SEPARATOR}\n");
}

fn print_multiple_days(report: &ExecutionReport) {
    println!("\n🎄 Advent of Code — Execution report");
    println!("{SEPARATOR}");

    println!(
        "  {:<6} {:<5} {:<5} {:<24} {:>12}",
        "Year", "Day", "Part", "Answer", "Duration",
    );

    println!("{SEPARATOR}");

    for day in &report.days {
        match &day.outcome {
            Ok(results) => {
                for result in results {
                    println!(
                        "  {:<6} {:02}    {:<5} {:<24} {:>12.3?}",
                        day.year, day.day, result.part, result.answer, result.duration,
                    );
                }
            }

            Err(error) => {
                print_day_error(day.year, day.day, error);
            }
        }
    }

    println!("{SEPARATOR}");
}

pub fn print_summary(report: &ExecutionReport) {
    println!("\nExecution summary");
    println!("{SEPARATOR}");

    println!("  Total days  : {}", report.total_days());
    println!("  Successful  : {}", report.successful_days());
    println!("  Failed      : {}", report.failed_days());
    println!("  Solver time : {:.3?}", report.total_duration());

    println!("{SEPARATOR}\n");
}
