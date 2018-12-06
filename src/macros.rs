macro_rules! hashset {
    ($($val: expr),*) => {{
         let values = vec![$($val),*];
         let map: ::std::collections::HashSet<_> = values.into_iter().collect();
         map
    }}
}
