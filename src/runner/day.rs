use crate::runner::inputs::get_day_input;
use std::error::Error;

pub type DayCtor = fn() -> Box<dyn Day>;

pub enum Part {
    One,
    Two,
}

pub trait Day {
    fn parse(&mut self, data: String);
    fn task1(&self) -> String;
    fn task2(&self) -> String;
}

pub struct RegisteredDay {
    year: usize,
    day: usize,
    solver_ctor: DayCtor,
}

impl RegisteredDay {
    pub fn new(year: usize, day: usize, solver_ctor: DayCtor) -> Self {
        Self {
            year,
            day,
            solver_ctor,
        }
    }
    pub fn run(&self, part: Option<Part>) -> Result<(), Box<dyn Error>> {
        let mut solver = (self.solver_ctor)();
        self.load(solver.as_mut())?;
        match part {
            Some(Part::One) => self.timed_task1(solver.as_ref()),
            Some(Part::Two) => self.timed_task2(solver.as_ref()),
            None => {
                self.timed_task1(solver.as_ref());
                self.timed_task2(solver.as_ref());
            }
        };
        Ok(())
    }

    fn timed_task1(&self, solver: &dyn Day) {
        timed(1, || solver.task1());
    }
    fn timed_task2(&self, solver: &dyn Day) {
        timed(2, || solver.task2());
    }

    fn load(&self, solver: &mut dyn Day) -> Result<(), Box<dyn Error>> {
        let data = get_day_input(self.year, self.day)?;
        println!(
            "----- Parsing data for a Day {} Year {} -----",
            self.day, self.year
        );
        solver.parse(data);
        Ok(())
    }
}

fn timed(n: u8, task: impl FnOnce() -> String) {
    let start = std::time::Instant::now();
    println!("Task {n}: {} ({:?})", task(), start.elapsed());
}
