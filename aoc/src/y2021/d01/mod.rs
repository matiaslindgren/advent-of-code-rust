pub fn main(input: &str) -> String {
    let v: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("failed parsing line as i32"))
        .collect();
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[i32]) -> i32 {
    v.windows(2).map(|w| (w[0] < w[1]) as i32).sum()
}

fn find_b(v: &[i32]) -> i32 {
    let sums: Vec<i32> = v.windows(3).map(|w| w.iter().sum()).collect();
    find_a(&sums)
}
