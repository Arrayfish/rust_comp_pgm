use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut k: Vec<i32> = Vec::new();
    let mut s: Vec<Vec<i32>> = Vec::new();
    for _ in 0..m{
        input!{
            kk: i32, ss: [i32; kk]
        }
        k.push(kk);
        s.push(ss);
    }
    input!{
        p: [i32; m],
    }
    // 2^n回試せば良い
    let mut ans =0;
    for i in 0..(2_i32.pow(n as u32)){
        let mut light_on_cnt = 0;
        for li in 0..m{
            let mut sw_cnt = 0;
            for sw_idx in 0..k[li]{
                if check_switch(i, s[li][sw_idx as usize]-1){
                    sw_cnt += 1;
                }
            }
            if sw_cnt % 2 == p[li]{
                light_on_cnt += 1;
            }
        }
        if light_on_cnt == m{
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn check_switch(a: i32, idx: i32) ->bool {
    (a & (1 << idx)) != 0
}
