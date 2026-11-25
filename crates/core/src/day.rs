
type SolverFactory = fn() -> Box<dyn Day>;

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
    solver_factory: SolverFactory,
}

impl RegisteredDay {
    #[must_use]
    pub fn new<T>(year: usize, day: usize) -> Self
    where
        T: Day + Default + 'static,
    {
        Self {
            year,
            day,
            solver_factory: || Box::new(T::default()),
        }
    }
    pub fn run(&self, input: String, part: Option<Part>) {
        let mut solver = (self.solver_factory)();
        solver.parse(input);
        match part {
            Some(Part::One) => self.timed_task1(solver.as_ref()),
            Some(Part::Two) => self.timed_task2(solver.as_ref()),
            None => {
                self.timed_task1(solver.as_ref());
                self.timed_task2(solver.as_ref());
            }
        }
    }

    fn timed_task1(&self, solver: &dyn Day) {
        timed(1, || solver.task1());
    }
    fn timed_task2(&self, solver: &dyn Day) {
        timed(2, || solver.task2());
    }
}

fn timed(n: u8, task: impl FnOnce() -> String) {
    let start = std::time::Instant::now();
    println!("Task {n}: {} ({:?})", task(), start.elapsed());
}
