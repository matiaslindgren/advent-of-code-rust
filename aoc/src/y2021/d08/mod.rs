pub fn main(input: &str) -> String {
    let outputs = parse_outputs(input);
    let a = find_a(&outputs);
    let b = find_b(&outputs);
    format!("{} {}", a, b)
}

fn find_a(outputs: &[String]) -> usize {
    let pattern = "1478";
    outputs
        .iter()
        .flat_map(|s| s.chars())
        .filter(|&c| pattern.contains(c))
        .count()
}

fn find_b(outputs: &[String]) -> usize {
    outputs.iter().flat_map(|o| o.parse::<usize>()).sum()
}

fn parse_outputs(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once(" | ")
                .expect("expected line split by ' | '");
            let l = l.split(' ').map(str::to_string);
            let r = r.split(' ').map(str::to_string);
            let e = Entry {
                inputs: l.collect(),
                output: r.collect(),
            };
            decode_entry(&e)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Entry {
    inputs: Vec<String>,
    output: Vec<String>,
}

impl Entry {
    fn get_digit(&self, num_wires: usize) -> String {
        self.get_digits(num_wires)[0].to_string()
    }

    fn get_digits(&self, num_wires: usize) -> Vec<String> {
        self.inputs
            .iter()
            .filter(|i| i.len() == num_wires)
            .cloned()
            .collect()
    }
}

struct Segments {
    s: Vec<String>,
}

impl Segments {
    fn new() -> Self {
        Self {
            s: vec!["abcdefg".to_string(); 7],
        }
    }

    fn retain(&mut self, wires: &str, segments: &[usize]) {
        for (i, s) in self.s.iter_mut().enumerate() {
            if segments.contains(&i) {
                s.retain(|c| wires.contains(c))
            } else {
                s.retain(|c| !wires.contains(c));
            }
        }
    }

    fn firsts(&self) -> Vec<char> {
        self.s.iter().flat_map(|s| s.chars().next()).collect()
    }

    fn decode(&self, wires: &str) -> char {
        let s: String = self
            .firsts()
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| {
                if wires.contains(c) {
                    Some((i as u8 + 48) as char)
                } else {
                    None
                }
            })
            .collect();
        match s.as_str() {
            "012456" => '0',
            "25" => '1',
            "02346" => '2',
            "02356" => '3',
            "1235" => '4',
            "01356" => '5',
            "013456" => '6',
            "025" => '7',
            "0123456" => '8',
            "012356" => '9',
            _ => panic!("unknown digit"),
        }
    }
}

fn decode_entry(e: &Entry) -> String {
    let mut segments = Segments::new();
    let d1 = e.get_digit(2);
    segments.retain(&d1, &[2, 5]);
    let d4 = e.get_digit(4);
    segments.retain(&d4, &[1, 2, 3, 5]);
    let d7 = e.get_digit(3);
    segments.retain(&d7, &[0, 2, 5]);

    let fives = e.get_digits(5);
    segments.s[3]
        .retain(|c| d4.contains(c) && fives.iter().all(|s| s.contains(c)));
    let mut known: String = [segments.firsts()[0], segments.firsts()[3]]
        .iter()
        .collect();
    segments.s[6]
        .retain(|c| !known.contains(c) && fives.iter().all(|s| s.contains(c)));
    known.push(segments.firsts()[6]);
    segments.s[1].retain(|c| !known.contains(c));
    known.push(segments.firsts()[1]);
    segments.s[4].retain(|c| !known.contains(c));
    known.push(segments.firsts()[4]);

    let sixes = e.get_digits(6);
    segments.s[5]
        .retain(|c| !known.contains(c) && sixes.iter().all(|s| s.contains(c)));
    known.push(segments.firsts()[5]);
    segments.s[2].retain(|c| !known.contains(c));

    e.output.iter().map(|d| segments.decode(d)).collect()
}
