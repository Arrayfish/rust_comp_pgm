use proconio::input;

fn main() {
    input! {
        n: i32,
        s: String
    }
    // とり得る値は1000通り
    // 正直にやると1000パターン全て試す必要がある。先にやってみる
    let digits = ['0','1','2','3','4','5','6','7','8','9'];
    // 左から読むので、
    let mut ans = 0;
    for dig2 in digits{
        for dig1 in digits{
            for dig0 in digits{
                let mut ok_digit: i32 = 2;
                for c in s.chars(){
                    match ok_digit{
                        2 => {
                            if dig2 == c{
                                ok_digit = 1
                            }
                        }
                        1 => {
                            if dig1 == c{
                                ok_digit = 0
                            }
                        }
                        0 => {
                            if dig0 == c{
                                ans += 1;
                                break;
                            }
                        }
                        _ => ()

                    }
                }
            }
        }
    }
    println!("{}", ans);
    
}
