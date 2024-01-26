use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        x: i64,
        y: i64,
    }
    let mut ans = 0;
    // パターン1 全部別々で買う
    ans = a*x + b*y; 
    // パターン2 a,bどちらか少ない方の数までc*2を買ってそれ以上は単品
    let amari = if x < y{
        b*(y-x)
    }else{
        a*(x-y)
    };
    ans = ans.min(amari + c*2*x.min(y));
    // パターン3 全部c 
    ans = ans.min(c*2*x.max(y));
    println!("{}",ans);
}
