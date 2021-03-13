// 3/13 AC

// https://atcoder.jp/contests/abc195/submissions/20877671

use std::str::FromStr;

fn main() {
    
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let mut it = buf.split_whitespace().map(|n| usize::from_str(n).unwrap());

    let (m, h) = (it.next().unwrap(), it.next().unwrap());

    if h % m == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
