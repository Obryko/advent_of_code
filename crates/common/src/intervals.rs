use std::cmp::{max, min};
use std::error::Error;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
pub enum IntervalError<T = i64> {
    InvalidBounds { start: T, end: T },
}
impl<T: Display> Display for IntervalError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntervalError::InvalidBounds { start, end } => {
                write!(f, "Invalid bounds: start={}, end={}", start, end)
            }
        }
    }
}
impl<T: Display + Debug> Error for IntervalError<T> {}
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Interval<T = i64> {
    start: T,
    end: T,
}

impl<T: Ord + Copy> Interval<T> {
    pub fn new(start: T, end: T) -> Result<Self, IntervalError<T>> {
        if start > end {
            return Err(IntervalError::InvalidBounds { start, end });
        }

        Ok(Self::new_unchecked(start, end))
    }

    fn new_unchecked(start: T, end: T) -> Self {
        Self { start, end }
    }
    pub fn start(&self) -> T {
        self.start
    }

    pub fn end(&self) -> T {
        self.end
    }
    pub fn contains(&self, value: T) -> bool {
        self.start <= value && value <= self.end
    }

    pub fn contains_interval(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            Some(Self::new_unchecked(
                max(self.start, other.start),
                min(self.end, other.end),
            ))
        } else {
            None
        }
    }

    pub fn merge(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            Some(Self::new_unchecked(
                min(self.start, other.start),
                max(self.end, other.end),
            ))
        } else {
            None
        }
    }
}

impl<T> Interval<T>
where
    T: Ord + Copy + Add<Output = T>,
{
    pub fn shift(&self, value: T) -> Self {
        Self::new_unchecked(self.start + value, self.end + value)
    }
}

impl<T> Interval<T>
where
    T: Copy + Sub<Output = T> + Add<Output = T> + From<u8>,
{
    pub fn len(&self) -> T {
        self.end - self.start + T::from(1)
    }
}

impl<T: Ord + Copy> TryFrom<(T, T)> for Interval<T> {
    type Error = IntervalError<T>;

    fn try_from(value: (T, T)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl<T> Sub<Interval<T>> for Interval<T>
where
    T: Ord + Copy + Sub<Output = T> + Add<Output = T> + From<u8>,
{
    type Output = Vec<Interval<T>>;

    fn sub(self, rhs: Interval<T>) -> Self::Output {
        let Some(overlap) = self.intersection(&rhs) else {
            return vec![self];
        };

        let mut result = Vec::new();

        if self.start < overlap.start {
            result.push(Self::new_unchecked(self.start, overlap.start - T::from(1)));
        }

        if overlap.end < self.end {
            result.push(Self::new_unchecked(overlap.end + T::from(1), self.end));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_interval() {
        let interval = Interval::new(1, 3);
        assert!(interval.is_ok());
        let interval = interval.unwrap();
        assert_eq!(interval.start, 1);
        assert_eq!(interval.end, 3);
    }
    #[test]
    fn should_not_create_interval() {
        let interval = Interval::new(3, 1);
        assert!(interval.is_err());
        let interval_error = interval.unwrap_err();
        assert_eq!(
            interval_error,
            IntervalError::InvalidBounds { start: 3, end: 1 }
        );
    }

    #[test]
    fn should_get_start_of_interval() {
        let interval = Interval::new_unchecked(1, 3);
        assert_eq!(interval.start(), 1);
    }

    #[test]
    fn should_get_end_of_interval() {
        let interval = Interval::new_unchecked(1, 3);
        assert_eq!(interval.end(), 3);
    }

    #[test]
    fn should_check_if_interval_contains_value() {
        let interval = Interval::new_unchecked(1, 3);
        assert!(interval.contains(2));
        assert!(!interval.contains(4));
    }

    #[test]
    fn should_check_if_interval_contains_interval() {
        let interval = Interval::new(1, 3).unwrap();
        let other = Interval::new_unchecked(2, 3);
        assert!(interval.contains_interval(&other));
        assert!(interval.overlaps(&other));
        let other = Interval::new_unchecked(4, 5);
        assert!(!interval.contains_interval(&other));
    }
    #[test]
    fn should_check_if_interval_overlaps() {
        let interval = Interval::new_unchecked(1, 3);
        let other = Interval::new_unchecked(2, 4);
        assert!(!interval.contains_interval(&other));
        assert!(interval.overlaps(&other));
        let other = Interval::new_unchecked(4, 5);
        assert!(!interval.overlaps(&other));
    }

    #[test]
    fn should_get_intersection_of_intervals() {
        let interval = Interval::new_unchecked(1, 3);
        let other = Interval::new_unchecked(2, 4);
        let intersection = interval.intersection(&other).unwrap();
        assert_eq!(intersection.start, 2);
        assert_eq!(intersection.end, 3);
    }
    #[test]
    fn should_get_length_of_interval() {
        let interval = Interval::new_unchecked(1, 3);
        assert_eq!(interval.len(), 3);
    }

    #[test]
    fn should_merge_intervals() {
        let interval = Interval::new_unchecked(1, 3);
        let other = Interval::new_unchecked(2, 4);
        let merged = interval.merge(&other).unwrap();
        assert_eq!(merged.start, 1);
        assert_eq!(merged.end, 4);
    }

    #[test]
    fn should_shift_interval() {
        let interval = Interval::new_unchecked(1, 3);
        let shifted = interval.shift(2);
        assert_eq!(shifted.start, 3);
        assert_eq!(shifted.end, 5);
    }

    #[test]
    fn should_contain_boundaries() {
        let interval = Interval::new_unchecked(1, 3);

        assert!(interval.contains(1));
        assert!(interval.contains(3));
    }

    #[test]
    fn should_not_merge_non_overlapping_intervals() {
        let interval = Interval::new_unchecked(1, 3);
        let other = Interval::new_unchecked(4, 5);

        assert_eq!(interval.merge(&other), None);
    }

    #[test]
    fn should_subtract_interval_from_middle() {
        let interval = Interval::new(10, 20).unwrap();
        let other = Interval::new(15, 17).unwrap();

        assert_eq!(
            interval - other,
            vec![
                Interval::new(10, 14).unwrap(),
                Interval::new(18, 20).unwrap(),
            ]
        );
    }

    #[test]
    fn should_subtract_interval_from_left() {
        let interval = Interval::new(10, 20).unwrap();
        let other = Interval::new(5, 12).unwrap();

        assert_eq!(interval - other, vec![Interval::new(13, 20).unwrap()]);
    }

    #[test]
    fn should_subtract_interval_from_right() {
        let interval = Interval::new(10, 20).unwrap();
        let other = Interval::new(18, 25).unwrap();

        assert_eq!(interval - other, vec![Interval::new(10, 17).unwrap()]);
    }

    #[test]
    fn should_subtract_whole_interval() {
        let interval = Interval::new(10, 20).unwrap();
        let other = Interval::new(5, 25).unwrap();

        assert!((interval - other).is_empty());
    }

    #[test]
    fn should_keep_interval_when_there_is_no_overlap() {
        let interval = Interval::new(10, 20).unwrap();
        let other = Interval::new(30, 40).unwrap();

        assert_eq!(interval - other, vec![interval]);
    }
}
