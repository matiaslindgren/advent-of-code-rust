use crate::common;

pub fn main(input: &str) -> String {
    let v = common::items::<String>(input, "\n");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[String]) -> u32 {
    let (most, least) = common(&v);
    multiply_bits(&most, &least)
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
            let (m, mc) = *c.most_common().first().unwrap();
            let (l, lc) = *c.most_common().last().unwrap();
            if mc == lc {
                ('1', '0')
            } else {
                (m, l)
            }
        })
        .collect();
    let most: String = v.iter().map(|(m, _)| m).collect();
    let least: String = v.iter().map(|(_, l)| l).collect();
    (most, least)
}

fn find_b(v: &[String]) -> u32 {
    let mut o2: Vec<String> = v.iter().cloned().collect();
    let mut co2: Vec<String> = v.iter().cloned().collect();
    for i in 0..(v[0].len()) {
        if o2.len() > 1 {
            let (most, _) = common(&o2);
            let mut res: Vec<String> = vec![];
            for x in o2 {
                if x.as_bytes()[i] == most.as_bytes()[i] {
                    res.push(x.to_string());
                }
            }
            o2 = res;
        }
        if co2.len() > 1 {
            let (_, least) = common(&co2);
            let mut res: Vec<String> = vec![];
            for x in co2 {
                if x.as_bytes()[i] == least.as_bytes()[i] {
                    res.push(x.to_string());
                }
            }
            co2 = res;
        }
        if co2.len() == 1 && o2.len() == 1 {
            break;
        }
    }
    multiply_bits(&o2[0], &co2[0])
}
