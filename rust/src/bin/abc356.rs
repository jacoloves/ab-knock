use itertools::Itertools;
#[allow(unused_imports)]
use proconio::input;
use proconio::marker::Usize1;

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

fn a() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    let mut a = Vec::new();
    for i in 1..=n {
        a.push(i);
    }

    let mut b = Vec::new();
    for i in 1..=n {
        if i >= l && i <= r {
            b.push(i);
        }
    }

    b.sort_by(|a, b| b.cmp(a));

    let mut cnt = 0;
    for i in 1..=n {
        if i >= l && i <= r {
            print!("{} ", b[cnt]);
            cnt += 1;
        } else {
            print!("{} ", a[i - 1]);
        }
    }

    println!();
}

#[allow(dead_code)]
fn c() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let ks = (0..m)
        .into_iter()
        .map(|_| {
            input! {
                c: usize,
                a: [Usize1; c],
                r: char
            }
            (a, r == 'o')
        })
        .collect_vec();

    let mut res = 0;

    'i: for i in 0..1 << n {
        for (v, r) in &ks {
            let num = v.iter().filter(|&&x| i >> x & 1 == 1).count();
            if !((num >= k && *r) || (num < k && !*r)) {
                continue 'i;
            }
        }
        res += 1;
    }

    println!("{}", res);
}
