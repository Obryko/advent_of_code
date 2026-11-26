#[macro_export]
macro_rules! year {
    ($year:literal, $($n:literal => $module:ident),* $(,)?) => {
        $(mod $module;)*

        pub static DAYS: &[$crate::RegisteredDay] = &[
            $(
                $crate::RegisteredDay::new::<$module::Solver>($year, $n)
            ),*
        ];
    };
}