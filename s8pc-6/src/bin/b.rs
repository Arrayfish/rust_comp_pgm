use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [[i64; 2]; n],
    }
  // median
  let mut a: Vec<i64> = ab.iter().map(|x| x[0]).collect();
  a.sort();
  let mut b: Vec<i64> = ab.iter().map(|x| x[1]).collect();
  b.sort();
  let start = a[n/2];
  let end = b[n/2];
  
  let mut ans : u64 = 0;
  for i in 0..n{
    ans += ((ab[i][0] - (start as i64)).abs() + (ab[i][0] - ab[i][1]).abs() + (ab[i][1] - (end as i64)).abs()) as u64;
  }
  println!("{}", ans);
  
}
