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

pub fn directions<Label, Distance>(
    input: &str,
    sep: &str,
    label_len: usize,
) -> Vec<(Label, Distance)>
where
    Label: FromStr,
    Distance: FromStr,
{
    let labels = input
        .split(sep)
        .filter_map(|x| str::parse::<Label>(&x[..label_len]).ok());
    let distances = input
        .split(sep)
        .filter_map(|x| str::parse::<Distance>(&x[label_len..]).ok());
    labels.zip(distances).collect()
}
