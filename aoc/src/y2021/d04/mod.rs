use crate::common;

pub fn main(input: &str) -> String {
    let _v = common::items::<i32>(input, "\n");
    let a = 0;
    let b = 0;
    format!("{} {}", a, b)
}
