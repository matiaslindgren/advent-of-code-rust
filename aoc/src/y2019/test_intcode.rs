use crate::y2019::intcode::IntCode;

fn assert_outputs(prog: &str, expected_outputs: &[i64]) {
    let mut ic = IntCode::new(prog);
    println!("{}", ic.dump_memory());
    let mut outputs: Vec<i64> = vec![];
    while !ic.terminated {
        if let Some(o) = ic.run() {
            outputs.push(o);
        }
    }
    for (a, b) in outputs.iter().zip(expected_outputs) {
        assert_eq!(a, b);
    }
}

#[test]
fn test1() {
    let prog = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let outputs: Vec<i64> = prog.split(',').filter_map(|x| str::parse(x).ok()).collect();
    assert_outputs(prog, &outputs);
}

#[test]
fn test2() {
    assert_outputs("1102,34915192,34915192,7,4,7,99,0", &[1219070632396864]);
    assert_outputs("104,1125899906842624,99", &[1125899906842624]);
}
