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
    abc342_a();
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
