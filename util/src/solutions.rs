pub fn solve(input: &str, year: u32, day: u32) -> String {
    // TODO macro
    match (year, day) {
        (2019, 1) => aoc::y2019::d01::main(input),
        (2019, 2) => aoc::y2019::d02::main(input),
        (2019, 3) => aoc::y2019::d03::main(input),
        (2019, 4) => aoc::y2019::d04::main(input),
        (2019, 5) => aoc::y2019::d05::main(input),
        (2019, 6) => aoc::y2019::d06::main(input),
        (2019, 7) => aoc::y2019::d07::main(input),
        (2019, 8) => aoc::y2019::d08::main(input),
        (2019, 9) => aoc::y2019::d09::main(input),
        (2019, 10) => aoc::y2019::d10::main(input),
        (2019, 11) => aoc::y2019::d11::main(input),
        (2019, 12) => aoc::y2019::d12::main(input),
        (2019, 13) => aoc::y2019::d13::main(input),
        (2019, 14) => aoc::y2019::d14::main(input),
        (2019, 15) => aoc::y2019::d15::main(input),
        (2019, 16) => aoc::y2019::d16::main(input),
        (2019, 17) => aoc::y2019::d17::main(input),
        (2019, 18) => aoc::y2019::d18::main(input),
        (2019, 19) => aoc::y2019::d19::main(input),
        (2019, 20) => aoc::y2019::d20::main(input),
        (2019, 21) => aoc::y2019::d21::main(input),
        (2019, 22) => aoc::y2019::d22::main(input),
        (2019, 23) => aoc::y2019::d23::main(input),
        (2019, 24) => aoc::y2019::d24::main(input),
        (2019, 25) => aoc::y2019::d25::main(input),
        (2020, 1) => aoc::y2020::d01::main(input),
        (2021, 1) => aoc::y2021::d01::main(input),
        (2021, 2) => aoc::y2021::d02::main(input),
        (2021, 3) => aoc::y2021::d03::main(input),
        (2021, 4) => aoc::y2021::d04::main(input),
        (2021, 5) => aoc::y2021::d05::main(input),
        (2021, 6) => aoc::y2021::d06::main(input),
        (2021, 7) => aoc::y2021::d07::main(input),
        (2021, 8) => aoc::y2021::d08::main(input),
        (2021, 9) => aoc::y2021::d09::main(input),
        (2021, 10) => aoc::y2021::d10::main(input),
        (2021, 11) => aoc::y2021::d11::main(input),
        (2021, 12) => aoc::y2021::d12::main(input),
        (2021, 13) => aoc::y2021::d13::main(input),
        (2021, 14) => aoc::y2021::d14::main(input),
        (2021, 15) => aoc::y2021::d15::main(input),
        (2021, 16) => aoc::y2021::d16::main(input),
        (2021, 17) => aoc::y2021::d17::main(input),
        (2021, 18) => aoc::y2021::d18::main(input),
        (2021, 19) => aoc::y2021::d19::main(input),
        (2021, 20) => aoc::y2021::d20::main(input),
        (2021, 21) => aoc::y2021::d21::main(input),
        (2021, 22) => aoc::y2021::d22::main(input),
        (2021, 23) => aoc::y2021::d23::main(input),
        (2021, 24) => aoc::y2021::d24::main(input),
        (2021, 25) => aoc::y2021::d25::main(input),
        _ => panic!("year {} day {} not implemented", year, day),
    }
}
