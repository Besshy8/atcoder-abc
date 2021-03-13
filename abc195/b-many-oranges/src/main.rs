// 3/13 WA TLE

// https://atcoder.jp/contests/abc195/submissions/20903129

// 浮動小数点非対応、実行時間

// 関数でエラー。借用周り

use std::str::FromStr;

fn main() {
    
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    let mut it = buf.split_whitespace().map(|n| usize::from_str(n).unwrap());

    let (a, b, w) = (it.next().unwrap_or(0),it.next().unwrap_or(0),it.next().unwrap_or(0));

    let mut max = 0;
    let mut min = 1000000000;
    let mut flag = false;

    for i in a..b+1 {
        for j in a..b+1 {
            for k in 0..w*1000/a+1 {
                for l in 0..w*1000/b+1 {
                    if (i*k + j*l) == w*1000 {                        
                        // max_min(i+j, &mut max, &mut min);
                        // println!("{}", i+j);
                        flag = true;
                        if k+l >= max {
                            max = k+l;
                        }
                        if k+l <= min {
                            min = k+l;
                        }
                    }
                }
            }
        }
    }

    if flag == false {
        println!("UNSATISFIABLE");
    } else {
        println!("{}", max);
        println!("{}",min);
    }

}

// 以下,関数

/*

fn max_min(num: usize, max: &mut usize, min: &mut usize) {
    if num >= *max {
        max = &mut num;
    }
    if num <= *min {
        min = &mut num;
    }
}

*/

/*

   Compiling b-many-oranges v0.1.0 (file:///app)
warning: value assigned to `max` is never read
  --> src/main.rs:34:9
   |
34 |         max = &mut num;
   |         ^^^
   |
   = note: #[warn(unused_assignments)] on by default

warning: value assigned to `min` is never read
  --> src/main.rs:37:9
   |
37 |         min = &mut num;
   |         ^^^

error[E0597]: `num` does not live long enough
  --> src/main.rs:34:20
   |
34 |         max = &mut num;
   |                    ^^^ does not live long enough
...
39 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 32:1...
  --> src/main.rs:32:1
   |
32 | / fn max_min(num: usize, max: &mut usize, min: &mut usize) {
33 | |     if num >= *max {
34 | |         max = &mut num;
35 | |     }
...  |
38 | |     }
39 | | }
   | |_^

error[E0597]: `num` does not live long enough
  --> src/main.rs:37:20
   |
37 |         min = &mut num;
   |                    ^^^ does not live long enough
38 |     }
39 | }
   | - borrowed value only lives until here
   |
note: borrowed value must be valid for the anonymous lifetime #2 defined on the function body at 32:1...
  --> src/main.rs:32:1
   |
32 | / fn max_min(num: usize, max: &mut usize, min: &mut usize) {
33 | |     if num >= *max {
34 | |         max = &mut num;
35 | |     }
...  |
38 | |     }
39 | | }
   | |_^

error[E0384]: cannot assign twice to immutable variable `max`
  --> src/main.rs:34:9
   |
32 | fn max_min(num: usize, max: &mut usize, min: &mut usize) {
   |                        --- first assignment to `max`
33 |     if num >= *max {
34 |         max = &mut num;
   |         ^^^^^^^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `min`
  --> src/main.rs:37:9
   |
32 | fn max_min(num: usize, max: &mut usize, min: &mut usize) {
   |                                         --- first assignment to `min`
...
37 |         min = &mut num;
   |         ^^^^^^^^^^^^^^ cannot assign twice to immutable variable

error: aborting due to 4 previous errors

error: Could not compile `b-many-oranges`.

*/