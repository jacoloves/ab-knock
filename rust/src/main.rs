#[allow(unused_imports)]
use proconio::input;

/* ↓aoj */
#[allow(unused_imports)]
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::HashSet;

#[allow(unused_imports)]
use std::vec;
#[allow(unused_imports)]
use std::{isize, usize};

#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::str::*;

use std::fmt::Debug;
use std::str::FromStr;

use regex::Regex;

#[allow(dead_code)]
fn read_line<T: std::str::FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}
/* ↑AOJ */

fn main() {
    abc349_a();
}

#[allow(dead_code)]
fn abc349_b() {
    input! {
        s: String,
    }

    let mut mp = HashMap::new();

    for c in s.chars() {
        *mp.entry(c).or_insert(0) += 1;
    }
}

#[allow(dead_code)]
fn abc349_a() {
    input! {
        n: usize,
        v: [isize; n-1],
    }

    let mut ans = 0;

    for e in v.iter() {
        ans -= e;
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn abc347_b() {
    input! {
        s: String,
    }

    let mut ss = HashSet::new();

    let len = s.len();

    for i in 0..len {
        for j in 1..=len - i {
            let substr = &s[i..i + j];
            ss.insert(substr);
        }
    }

    println!("{}", ss.len());
}

#[allow(dead_code)]
fn abc347_a() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut ans: Vec<usize> = Vec::new();

    for i in a.iter() {
        if i % k == 0 {
            ans.push(i / k);
        }
    }

    for i in ans.iter() {
        print!("{} ", i);
    }

    println!();
}

#[allow(dead_code)]
fn sq(x: isize) -> isize {
    x * x
}

#[allow(dead_code)]
fn abc348_b() {
    input! {
        n: usize,
    }

    let mut x: Vec<isize> = Vec::new();
    let mut y: Vec<isize> = Vec::new();

    for _ in 0..n {
        input! {
            a: isize,
            b: isize,
        }

        x.push(a);
        y.push(b);
    }

    for i in 0..n {
        let mut m = 0;
        let mut id = 0;

        for j in 0..n {
            if i == j {
                continue;
            }
            let d = sq(x[i] - x[j]) + sq(y[i] - y[j]);
            if d > m {
                m = d;
                id = j;
            }
        }

        println!("{}", id + 1);
    }
}

#[allow(dead_code)]
fn abc348_a() {
    input! {
        n: usize,
    }

    let mut ans_str: String = "".to_string();

    for i in 1..=n {
        if i % 3 == 0 {
            ans_str.push('x');
        } else {
            ans_str.push('o');
        }
    }

    println!("{}", ans_str);
}

#[allow(dead_code)]
fn abc337_b() {
    input! {
        s: String,
    }

    let mut phase = 0;
    for c in s.chars() {
        match phase {
            0 => {
                if c == 'A' {
                    phase = 1;
                } else {
                    println!("No");
                    return;
                }
            }
            1 => {
                if c == 'B' {
                    phase = 2;
                } else if c == 'A' {
                } else {
                    println!("No");
                    return;
                }
            }
            2 => {
                if c == 'C' {
                    phase = 3;
                } else if c == 'B' {
                } else {
                    println!("No");
                    return;
                }
            }
            3 => {
                if c != 'C' {
                    println!("No");
                    return;
                }
            }
            _ => unreachable!(),
        }
    }

    if s == "A" {
        println!("Yes");
        return;
    }

    if phase == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn abc337_a() {
    input! {
        n: usize,
    }

    let mut x = 0;
    let mut y = 0;

    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }

        x += a;
        y += b;
    }

    if x > y {
        println!("Takahashi");
    } else if x < y {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}

#[allow(dead_code)]
fn abc338_b() {
    input! {
        s: String,
    }

    let mut mp = HashMap::new();

    for c in s.chars() {
        *mp.entry(c).or_insert(0) += 1;
    }

    let mut max_num = -1;
    let mut max_char = ' ';

    for (&c, &count) in &mp {
        if max_num < count {
            max_num = count;
            max_char = c;
        } else if max_num == count {
            if max_char > c {
                max_char = c;
            }
        }
    }

    println!("{}", max_char);
}

#[allow(dead_code)]
fn abc338_a() {
    input! {
        s: String,
    }

    let mut s_cnt = 0;

    for c in s.chars() {
        if c.is_uppercase() && s_cnt == 0 {
            s_cnt += 1;
        }

        if c.is_lowercase() && s_cnt != 0 {
            s_cnt += 1;
        }
    }

    if s_cnt == s.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[allow(dead_code)]
fn abc339_b() {
    input! {
        h: usize,
        w: usize,
        mut n: usize,
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; w]; h];

    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;

    while n > 0 {
        n -= 1;

        if grid[x][y] == '.' {
            grid[x][y] = '#';
            dir = (dir + 1) % 4;
        } else {
            grid[x][y] = '.';
            dir = (dir + 3) % 4;
        }

        if dir == 0 {
            x = (x - 1 + h) % h;
        } else if dir == 1 {
            y = (y + 1) % w;
        } else if dir == 2 {
            x = (x + 1) % h;
        } else {
            y = (y - 1 + w) % w;
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}

#[allow(dead_code)]
fn abc339_a() {
    input! {
        s: String,
    }

    let v: Vec<String> = s.split('.').map(|x| x.to_string()).collect();

    println!("{}", v[v.len() - 1]);
}

#[allow(dead_code)]
fn abc340_b() {
    input! {
        n: usize,
    }

    let mut v: Vec<usize> = Vec::new();
    let mut a: Vec<usize> = Vec::new();

    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
        }

        if x == 1 {
            v.push(y);
        } else {
            v.reverse();
            a.push(v[y - 1]);
            v.reverse();
        }
    }

    for i in a.into_iter() {
        println!("{}", i);
    }
}

#[allow(dead_code)]
fn abc340_a() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    for i in (a..=b).step_by(c) {
        print!("{} ", i);
    }

    println!();
}

#[allow(dead_code)]
fn abc341_b() {
    input! {
        n: usize,
        mut a: [usize; n],
        st: [[usize; 2]; n-1],
    }

    let mut s: Vec<usize> = Vec::new();
    let mut t: Vec<usize> = Vec::new();

    for i in 0..n - 1 {
        s.push(st[i][0]);
        t.push(st[i][1]);
    }

    for i in 0..n - 1 {
        a[i + 1] += (a[i] / s[i]) * t[i];
    }

    println!("{}", a[n - 1]);
}

#[allow(dead_code)]
fn abc341_a() {
    input! {
        n: usize,
    }

    let mut s: String = "1".to_string();

    for _ in 0..n {
        s.push_str("01");
    }

    println!("{}", s);
}

#[allow(dead_code)]
fn abc342_b() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        ab: [[usize; 2]; q],
    }

    let mut v: Vec<usize> = Vec::new();
    for i in 0..q {
        let a = ab[i][0];
        let b = ab[i][1];

        for j in 0..n {
            if a == p[j] {
                v.push(a);
                break;
            } else if b == p[j] {
                v.push(b);
                break;
            }
        }
    }

    for i in v.into_iter() {
        println!("{}", i);
    }
}

#[allow(dead_code)]
fn abc342_a() {
    input! {
        s: String,
    }

    if s.chars().nth(0) == s.chars().nth(1) {
        for (i, c) in s.chars().enumerate() {
            if c != s.chars().nth(0).unwrap() {
                println!("{}", i + 1);
                break;
            }
        }
    } else {
        if s.chars().nth(0) == s.chars().nth(2) {
            println!("2");
        } else {
            println!("1");
        }
    }
}

#[allow(dead_code)]
fn abc343_b() {
    input! {
        n: usize,
        graph: [[u8; n]; n],
    }

    for i in 0..n {
        let mut neighbors = Vec::new();
        for j in 0..n {
            if graph[i][j] == 1 {
                neighbors.push(j + 1);
            }
        }

        for neighbor in neighbors {
            print!("{} ", neighbor);
        }
        println!();
    }
}

#[allow(dead_code)]
fn abc343_a() {
    input! {
        a: usize,
        b: usize,
    }

    let vec_smp = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let sum = a + b;

    for i in vec_smp.into_iter() {
        if sum != i {
            println!("{}", i);
            return;
        }
    }
}

#[allow(dead_code)]
fn abc345_b() {
    input! {
        x: i128,
    }

    if x % 10 == 0 {
        println!("{}", x / 10);
    } else if x >= 0 {
        println!("{}", x / 10 + 1);
    } else {
        println!("{}", x / 10);
    }
}

#[allow(dead_code)]
fn abc345_a() {
    input! {
        s: String,
    }

    if s.len() < 3 {
        println!("No");
        return;
    }

    let bytes = s.as_bytes();

    if bytes[0] != b'<' || bytes[s.len() - 1] != b'>' {
        println!("No");
        return;
    }

    for &b in &bytes[1..s.len() - 1] {
        if b != b'=' {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

#[allow(dead_code)]
fn abc344_b() {
    let mut vn: Vec<usize> = Vec::new();
    loop {
        input! {
            n: usize,
        }

        if n == 0 {
            vn.push(n);
            break;
        } else {
            vn.push(n);
        }
    }

    let rev_vn = vn.iter().rev();

    for v in rev_vn {
        println!("{}", v);
    }
}

#[allow(dead_code)]
fn abc344_a() {
    input! {
        s: String,
    }

    let re = Regex::new(r"\|[^|]*\|").unwrap();
    let trans_s: String = re.replace_all(&s, "").to_string();

    println!("{}", trans_s);
}
