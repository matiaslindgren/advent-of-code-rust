use crate::y2019::intcode::parse_opcode;

#[test]
fn test_parse_opcode_only() {
    let op_str_list = vec!["1", "01", "2", "02", "99"];
    for op_str in op_str_list {
        let (op, modes) = parse_opcode(op_str);
        assert_eq!(op, op_str.parse::<u8>().expect("fail"));
        assert_eq!(modes.len(), 4);
        for m in modes {
            assert_eq!(m, 0);
        }
    }
}

#[test]
fn test_parse_opcode_modes1() {
    let (op, modes) = parse_opcode("11199");
    assert_eq!(op, 99);
    assert_eq!(modes.len(), 4);
    assert_eq!(modes[0], 1);
    assert_eq!(modes[1], 1);
    assert_eq!(modes[2], 1);
    assert_eq!(modes[3], 0);
}

#[test]
fn test_parse_opcode_modes2() {
    let (op, modes) = parse_opcode("1001");
    assert_eq!(op, 1);
    assert_eq!(modes.len(), 4);
    assert_eq!(modes[0], 0);
    assert_eq!(modes[1], 1);
    assert_eq!(modes[2], 0);
    assert_eq!(modes[3], 0);
}
