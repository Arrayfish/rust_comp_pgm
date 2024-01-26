fn main() {
    let a: i32 = 8;
    let b: usize = 3;
    let c :f64= (a as f64) / (b as f64);
    println!("{}", c.round());
}
