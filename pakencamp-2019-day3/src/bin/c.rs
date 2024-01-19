use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u64; m ]; n]
    }
    let mut ans: u64 = 0;
    // mの中で2つの組み合わせをみる
    for i in 0..m{
        for j in i+1..m{
            let mut sum = 0;
            for ni in 0..n{
                sum += a[ni][i].max(a[ni][j]);
            }
            ans = ans.max(sum);
        }
    }
    println!("{}", ans);
}
