use proconio::input;

fn main() {
    input!{
        n: i32
    }
    let mut ans = 0;
    for i in 1..n+1{
        // 偶数ならパス
        if i % 2 == 0{
            continue;
        }
        let mut cnt = 0;
        for j in 1..i+1{
            if i % j==0{
                cnt+=1 ;
            }
        }
        if cnt == 8{
            ans += 1;
        }
    }
    println!("{}", ans)
}
