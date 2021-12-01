use crate::common;

const GRID: &str = concat!(
    "#Iiiiiiiii\n",
    "...A......\n",
    "...B..a...\n",
    ".EDCG....a\n",
    "H.F.c.b...\n",
    "h....c....\n",
    "h.efd.c.gb\n",
    "h......c..\n",
    "h...f...c.\n",
    "h..e..d..c",
);

fn expect_chars(pos: (i64, i64), chars: &str) {
    let m = common::maze(GRID);
    let path = m.line_to((0, 0), pos);
    assert_eq!(chars.len(), path.len());
    for (ch, &(y1, x1, ch1)) in chars.chars().zip(path.iter()) {
        if ch != '.' {
            assert_eq!(ch1, ch, "({},{}) {}", y1, x1, ch1);
        }
    }
}

#[test]
fn test_a() {
    expect_chars((3, 9), "#Aaa");
}
#[test]
fn test_b() {
    expect_chars((6, 9), "#Bbb");
}
#[test]
fn test_c() {
    expect_chars((9, 9), "#..Ccccccc");
}
#[test]
fn test_d() {
    expect_chars((9, 6), "#Ddd");
}
#[test]
fn test_e() {
    expect_chars((9, 3), "#Eee");
}
#[test]
fn test_f() {
    expect_chars((8, 4), "#.Fff");
}
#[test]
fn test_g() {
    expect_chars((6, 8), "#Gg");
}
#[test]
fn test_h() {
    expect_chars((9, 0), "#...Hhhhhh");
}
#[test]
fn test_i() {
    expect_chars((0, 9), "#Iiiiiiiii");
}
