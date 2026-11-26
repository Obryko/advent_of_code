use crate::execution::timed;
use crate::{Part, PartResult};

type SolverFactory = fn() -> Box<dyn Day>;

pub trait Day {
    fn parse(&mut self, data: String);
    fn task1(&self) -> String;
    fn task2(&self) -> String;
}

pub struct RegisteredDay {
    pub year: usize,
    pub day: usize,
    solver_factory: SolverFactory,
}

impl RegisteredDay {
    #[must_use]
    pub const fn new<T>(year: usize, day: usize) -> Self
    where
        T: Day + Default + 'static,
    {
        Self {
            year,
            day,
            solver_factory: || Box::new(T::default()),
        }
    }
    pub fn run(&self, input: String, part: Option<Part>) -> Vec<PartResult> {
        let mut solver = (self.solver_factory)();
        solver.parse(input);
        match part {
            Some(part) => vec![timed(part, || part.execute(solver.as_ref()))],
            None => vec![
                timed(Part::One, || Part::One.execute(solver.as_ref())),
                timed(Part::Two, || Part::Two.execute(solver.as_ref())),
            ],
        }
    }
}
