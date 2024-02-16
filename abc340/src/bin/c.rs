use proconio::input;



fn main() {
    input!{
        n: u64
    }
    let mut ans: u64 = 0;
    let mut even_num: u64 = 0;
    let mut even_cnt: u64 = 0;
    let mut odd_num: u64 = 0;
    let mut odd_cnt: u64 = 0;
    ans += n;
    if n % 2 == 0{
        even_num = n/2;
        even_cnt = 2;
        
    }else{
        even_num = n/2;
        even_cnt = 1;
        odd_num = n/2 + 1;
        odd_cnt = 1;
    }
    while even_num > 1 || odd_num > 1{
        // add to ans
        if even_cnt{
            ans += even_num * even_cnt;
        }
        if odd_cnt {
            ans += odd_num * odd_cnt;
        }
        // change state
        // 3パターン
        // 1. 偶数から偶数
        // 2. 偶数から奇数
        // 3. 奇数と偶数
        let mut next_even_cnt = 0;
        let mut next_even_cnt = 0;
        if odd_cnt{
            even_cnt = even_cnt * 2 + odd_cnt;
            odd_cnt = odd_cnt;
        }
        if even_cnt
    }
}
