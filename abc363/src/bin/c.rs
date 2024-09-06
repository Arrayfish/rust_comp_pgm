use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut ans: u32 = 0;
    // count kind of char and its number
    let mut char_cnt = HashMap::new();
    for ss in s.iter(){
        let cnt = char_cnt.entry(ss).or_insert(0);
        *cnt += 1;
    }
    // check all permutation
    for p in s.iter().permutations(n){
        // get all pertial string
        let mut has_kaibun = false;
        for i in 0..(n-k + 1){
            if is_kaibun(&p[i..(i+k)]){
                has_kaibun = true;
                break;
            }
        }
        if !has_kaibun{
            ans += 1;
        }
    }
    // eliminate dublicate
    let mut div_num = 1;
    for (_, cnt) in &char_cnt{
        div_num *= factorial(*cnt);
    }
    println!("{}", ans/div_num);
}

fn is_kaibun(s: &[&char]) -> bool{
    for i in 0..s.len()/2{
        if s[i] != s[s.len()-i-1] {
            return false;
        }
    }
    true
}

fn factorial(n: u32) -> u32 {
    if n <= 1{
        return 1;
    }
    let mut rtn = 1;
    for i in 1..n+1 {
        rtn = rtn*i;
    }
    rtn
}
