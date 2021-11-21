use aoc;

pub fn get(input: &str, year: u32, day: u32) -> String {
    // TODO macro
    match (year, day) {
        (2020, 1) => aoc::y2020::d01::main(input),
        _ => panic!("year {} day {} not implemented", year, day),
    }
}
