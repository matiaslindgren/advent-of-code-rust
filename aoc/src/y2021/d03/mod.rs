use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let v: Vec<&str> = input.lines().collect();
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[&str]) -> u32 {
    let (gamma, epsilon) = reduce_by_bit_freq(v);
    multiply_bits(&gamma, &epsilon)
}

fn find_b(v: &[&str]) -> u32 {
    let mut o2: Vec<&str> = v.to_vec();
    let mut co2: Vec<&str> = v.to_vec();
    for i in 0..(v[0].len()) {
        if o2.len() > 1 {
            let (gamma, _) = reduce_by_bit_freq(&o2);
            o2 = filter_bits(i, &o2, &gamma);
        }
        if co2.len() > 1 {
            let (_, epsilon) = reduce_by_bit_freq(&co2);
            co2 = filter_bits(i, &co2, &epsilon);
        }
        if co2.len() == 1 && o2.len() == 1 {
            break;
        }
    }
    multiply_bits(o2[0], co2[0])
}

fn multiply_bits<'a>(gamma: &'a str, epsilon: &'a str) -> u32 {
    let g = u32::from_str_radix(gamma, 2).unwrap();
    let e = u32::from_str_radix(epsilon, 2).unwrap();
    g * e
}

type Counter = HashMap<char, usize>;

fn reduce_by_bit_freq(v: &[&str]) -> (String, String) {
    let mut counts = vec![Counter::default(); v[0].len()];
    for x in v {
        for (i, ch) in x.chars().enumerate() {
            if let Some(n) = counts[i].get_mut(&ch) {
                *n += 1;
            } else {
                counts[i].insert(ch, 1);
            }
        }
    }
    counts
        .into_iter()
        .map(|c| {
            let mut freqs: Vec<(char, usize)> = c.into_iter().collect();
            freqs.sort_unstable_by_key(|(_, n)| *n);
            let (char_m, freq_m) = *freqs.last().unwrap();
            let (char_l, freq_l) = *freqs.first().unwrap();
            if freq_m == freq_l {
                ('1', '0')
            } else {
                (char_m, char_l)
            }
        })
        .unzip()
}

fn filter_bits<'a>(i: usize, v: &[&'a str], bits: &str) -> Vec<&'a str> {
    let b = bits.as_bytes()[i];
    v.iter().cloned().filter(|x| x.as_bytes()[i] == b).collect()
}
