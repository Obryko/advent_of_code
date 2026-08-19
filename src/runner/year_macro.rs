#[macro_export]
macro_rules! year {
    ($year:literal, $($n:literal => $module:ident::$ty:ident),* $(,)?) => {
        $(mod $module;)*
        pub fn get_year_days() -> ::std::collections::BTreeMap<usize, $crate::RegisteredDay> {
            #[allow(unused_mut)]
            let mut days: ::std::collections::BTreeMap<usize, $crate::RegisteredDay> = ::std::collections::BTreeMap::new();
            $(days.insert(
                $n,
                $crate::RegisteredDay::new($year, $n, || Box::new($module::$ty::default()) as Box<dyn $crate::Day>),
            );)*
            days
        }
    };
}
