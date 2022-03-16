pub fn main(input: &str) -> String {
    let constraints = parse_constraints(input);
    let (a, b) = find_ab(&constraints);
    format!("{} {}", a, b)
}

fn parse_constraints(input: &str) -> Vec<Constraint> {
    let mut constraints = vec![];
    let mut stack = vec![];
    for (i, subprogram) in input.split("inp w\n").skip(1).enumerate() {
        let lines: Vec<&str> = subprogram.lines().collect();
        let parse_param = |i: usize| {
            lines[i]
                .split(' ')
                .last()
                .expect("expected line separated by ' '")
                .parse::<i8>()
                .expect("failed parsing param as i8")
        };
        let x1 = parse_param(4);
        let x2 = parse_param(14);
        let is_constraint = lines[3].ends_with("26");
        if is_constraint {
            let (j, x2) = stack
                .pop()
                .expect("first subprogram must contain 'div z 1'");
            constraints.push(((j, x2), (i, x1)));
        } else {
            stack.push((i, x2));
        }
    }
    constraints
}

type Constraint = ((usize, i8), (usize, i8));

fn find_ab(constraints: &[Constraint]) -> (String, String) {
    let mut max = vec![i8::min_value(); 14];
    let mut min = vec![i8::max_value(); 14];
    for &((j, x2), (i, x1)) in constraints.iter() {
        for j_num in 1..=9 {
            for i_num in 1..=9 {
                if j_num + x2 == i_num - x1 {
                    max[j] = max[j].max(j_num as i8);
                    max[i] = max[i].max(i_num as i8);
                    min[j] = min[j].min(j_num as i8);
                    min[i] = min[i].min(i_num as i8);
                }
            }
        }
    }
    (join_digits(&max), join_digits(&min))
}

fn join_digits(digits: &[i8]) -> String {
    digits
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join("")
}
