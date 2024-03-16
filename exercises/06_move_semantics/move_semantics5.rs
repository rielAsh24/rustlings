// move_semantics5.rs

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // println!("{}", y);
    let z = &mut x;
    *z += 1000;
    // println!("{}", z);
    assert_eq!(x, 1200);
}
