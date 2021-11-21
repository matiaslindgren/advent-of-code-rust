use std::str::FromStr;

pub fn numbers<T>(input: &str, sep: &str) -> Vec<T>
where
    T: FromStr,
{
    input
        .split(sep)
        .filter_map(|x| str::parse::<T>(x).ok())
        .collect()
}
