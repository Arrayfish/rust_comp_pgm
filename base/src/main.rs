use proconio::input;

fn main() {
    let mut a = 3; let mut b = 5;
std::mem::swap(&mut a, &mut b);
assert_eq!(a, 5);
assert_eq!(b, 3);
}
