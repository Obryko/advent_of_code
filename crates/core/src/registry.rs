use crate::RegisteredDay;

pub struct Registry {
    groups: &'static [&'static [RegisteredDay]],
}

impl Registry {
    pub const fn new(groups: &'static [&'static [RegisteredDay]]) -> Self {
        Self { groups }
    }

    pub fn iter(&self) -> impl Iterator<Item = &RegisteredDay> {
        self.groups.iter().flat_map(|group| group.iter())
    }
    pub fn find(&self, year: usize, day: usize) -> Option<&RegisteredDay> {
        self.iter().find(|d| d.year == year && d.day == day)
    }

    pub fn latest_year(&self) -> Option<usize> {
        self.iter().map(|d| d.year).max()
    }

    pub fn latest_day(&self, year: usize) -> Option<usize> {
        self.iter().filter(|d| d.year == year).map(|d| d.day).max()
    }

    pub fn latest(&self) -> Option<&RegisteredDay> {
        self.iter().max_by_key(|d| (d.year, d.day))
    }
}
