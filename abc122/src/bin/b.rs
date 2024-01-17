use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = 0;
    let mut cnt = 0;
    for c in s.chars() {
        cnt = match c {
            'A' | 'C' | 'G' | 'T' => cnt + 1,
            _ => 0,
        };
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}
