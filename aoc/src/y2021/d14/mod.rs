use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let (template, rules) = parse(input);
    let a = apply_steps(&template, &rules, 10);
    let b = apply_steps(&template, &rules, 40);
    format!("{} {}", a, b)
}

fn parse(input: &str) -> (Vec<char>, Rules) {
    let (template, rules) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let mut pair = parts.next().unwrap().chars();
            let ch1 = pair.next().unwrap();
            let ch2 = pair.next().unwrap();
            let replacement = parts.next().unwrap().chars().next().unwrap();
            ((ch1, ch2), replacement)
        })
        .into_iter()
        .collect();
    (template.chars().collect(), rules)
}

type Rules = HashMap<(char, char), char>;

fn apply_steps(template: &[char], rules: &Rules, num_steps: usize) -> usize {
    let mut pair_freq: HashMap<(char, char), usize> =
        rules.keys().map(|&k| (k, 0)).collect();
    for pair in template.windows(2) {
        let key = (pair[0], pair[1]);
        *pair_freq.get_mut(&key).unwrap() += 1;
    }

    let mut char_freq: HashMap<char, usize> =
        rules.values().map(|&v| (v, 0)).collect();
    for ch in template.iter() {
        *char_freq.get_mut(ch).unwrap() += 1;
    }

    for _ in 0..num_steps {
        let mut pair_freq_next = pair_freq.clone();
        for (pair, &n) in pair_freq.iter().filter(|(_, &n)| n > 0) {
            let insert_ch = rules.get(pair).unwrap();
            *char_freq.get_mut(insert_ch).unwrap() += n;
            let (ch1, ch2) = pair;
            *pair_freq_next.get_mut(pair).unwrap() -= n;
            *pair_freq_next.get_mut(&(*ch1, *insert_ch)).unwrap() += n;
            *pair_freq_next.get_mut(&(*insert_ch, *ch2)).unwrap() += n;
        }
        pair_freq = pair_freq_next;
    }

    let max = char_freq.values().max().unwrap();
    let min = char_freq.values().min().unwrap();
    max - min
}
