use crate::common;

pub fn main(input: &str) -> String {
    let v = common::items::<String>(input, "\n");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[String]) -> u32 {
    let (most, least) = common(v);
    multiply_bits(&most, &least)
}

fn find_b(v: &[String]) -> u32 {
    let mut o2: Vec<String> = v.to_vec();
    let mut co2: Vec<String> = v.to_vec();
    for i in 0..(v[0].len()) {
        if o2.len() > 1 {
            let (gamma, _) = common(&o2);
            o2 = filter_bits(i, &o2, &gamma);
        }
        if co2.len() > 1 {
            let (_, epsilon) = common(&co2);
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

fn common(v: &[String]) -> (String, String) {
    let mut counts = vec![common::Counter::<char>::new(); v[0].len()];
    for x in v {
        for (i, ch) in x.chars().enumerate() {
            counts[i].inc(&ch);
        }
    }
    let v: Vec<(char, char)> = counts
        .iter()
        .map(|c| {
            let (char_m, freq_m) = *c.most_common().first().unwrap();
            let (char_l, freq_l) = *c.most_common().last().unwrap();
            if freq_m == freq_l {
                ('1', '0')
            } else {
                (char_m, char_l)
            }
        })
        .collect();
    v.iter().cloned().unzip()
}

fn filter_bits(i: usize, v: &[String], bits: &str) -> Vec<String> {
    let b = bits.as_bytes()[i];
    v.iter().filter(|x| x.as_bytes()[i] == b).cloned().collect()
}
