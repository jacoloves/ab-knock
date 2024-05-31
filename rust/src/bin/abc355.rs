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

#[allow(unused_imports)]
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
    a();
}

#[allow(dead_code)]
fn a() {
    input! {
        a: usize,
        b: usize,
    }

    let mut v = [false; 3];
    v[a - 1] = true;
    v[b - 1] = true;

    let mut cnt = 0;
    for i in 0..3 {
        if v[i] {
            cnt += 1;
        }
    }

    if cnt == 2 {
        for j in 0..3 {
            if !v[j] {
                println!("{}", j + 1);
                return;
            }
        }
    } else {
        println!("-1");
    }
}
