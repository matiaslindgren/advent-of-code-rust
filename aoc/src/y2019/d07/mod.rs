use crate::y2019::intcode;
use itertools::Itertools;

pub fn main(prog: &str) -> String {
    let a = find_a(prog);
    let b = find_b(prog);
    format!("{} {}", a, b)
}

fn find_a(prog: &str) -> i64 {
    (0..=4).permutations(5).unique().into_iter().fold(
        0,
        |max_signal, phases| {
            let out = phases
                .iter()
                .fold(0, |signal, &phase| intcode::run(prog, &[phase, signal]));
            max_signal.max(out)
        },
    )
}

fn find_b(prog: &str) -> i64 {
    (5..=9).permutations(5).unique().into_iter().fold(
        0,
        |max_signal, phases| {
            let mut amps = vec![intcode::IntCode::new(prog); 5];
            for (amp, phase) in (amps.iter_mut()).zip(phases) {
                amp.push_input(phase);
            }
            let mut signal = 0;
            while !amps.iter().all(|a| a.done) {
                for amp in amps.iter_mut() {
                    amp.push_input(signal);
                    signal = amp.run().unwrap_or(signal);
                }
            }
            max_signal.max(signal)
        },
    )
}
