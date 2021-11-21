mod cli;
mod input;
mod output;

fn main() {
    let (year, day) = cli::parse_args();
    let input = input::get(year, day);
    let output = output::get(&input, year, day);
    println!("{}", output);
}
