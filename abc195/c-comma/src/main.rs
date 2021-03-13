// 3/13 TLE 実行速度

// https://atcoder.jp/contests/abc195/submissions/20890252

// 桁数を数えてコンマをうつ必要がありそう。

fn main() {
    
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).ok();
    let c : i64 = n.trim().parse().unwrap(); // i32にするとおかしな結果になる

    // println!("{}", c);

    let mut count = 0;

    for j in 1..c+1 {
        if j >= 1000 {
            count += 1;
        } 
        if j >= 1000000 {
            count += 1;
        }
        if j >= 1000000000 {
            count += 1;
        }
        if j >= 1000000000000 {
            count += 1;
        }
        if j >= 1000000000000000 {
            count += 1;
        }
    }
    println!("{}", count);
}
