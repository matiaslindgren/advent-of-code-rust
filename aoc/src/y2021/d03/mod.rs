use crate::common::{items, Counter};

pub fn main(input: &str) -> String {
    let v = items::<String>(input, "\n");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[String]) -> u32 {
    let (gamma, epsilon) = reduce_by_bit_freq(v);
    multiply_bits(&gamma, &epsilon)
}

fn find_b(v: &[String]) -> u32 {
    let mut o2: Vec<String> = v.to_vec();
    let mut co2: Vec<String> = v.to_vec();
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
    multiply_bits(&o2[0], &co2[0])
}

fn multiply_bits(gamma: &str, epsilon: &str) -> u32 {
    let g = u32::from_str_radix(gamma, 2).unwrap();
    let e = u32::from_str_radix(epsilon, 2).unwrap();
    g * e
}

fn reduce_by_bit_freq(v: &[String]) -> (String, String) {
    let mut counts = vec![Counter::<char>::new(); v[0].len()];
    for x in v {
        for (i, ch) in x.chars().enumerate() {
            counts[i].inc(&ch);
        }
    }
    counts
        .iter()
        .map(|c| {
            let freqs = c.most_common();
            let (char_m, freq_m) = *freqs.first().unwrap();
            let (char_l, freq_l) = *freqs.last().unwrap();
            if freq_m == freq_l {
                ('1', '0')
            } else {
                (char_m, char_l)
            }
        })
        .unzip()
}

fn filter_bits(i: usize, v: &[String], bits: &str) -> Vec<String> {
    let b = bits.as_bytes()[i];
    v.iter().filter(|x| x.as_bytes()[i] == b).cloned().collect()
}
