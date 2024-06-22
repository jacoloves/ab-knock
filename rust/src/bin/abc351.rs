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

fn main() {
    b();
}

fn b() {
    input! {
        n: usize,
        a: [String; n],
        b: [String; n],
    }

    for i in 0..n {
        if a[i] == b[i] {
            continue;
        }

        let mut cnt = 0;

        for (c1, c2) in a[i].chars().zip(b[i].chars()) {
            cnt += 1;
            if c1 != c2 {
                println!("{} {}", i + 1, cnt);
                return;
            }
        }
    }
}

fn a() {
    input! {
        a: [usize; 9],
        b: [usize; 8]
    }

    let a_sum: usize = a.iter().sum();
    let b_sum: usize = b.iter().sum();

    let ans = a_sum - b_sum + 1;

    println!("{ans}");
}
