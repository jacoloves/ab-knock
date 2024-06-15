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
    b()
}

fn b() {
    input! {
        n: usize,
        k: usize,
    }

    let mut cnt = 0;

    let mut empty_sheets = k;

    for _ in 0..n {
        input! { a: usize }
        if empty_sheets < a {
            cnt += 1;
            empty_sheets = k;
        }
        empty_sheets -= a;
    }
    cnt += 1;

    println!("{cnt}");
}

fn a() {
    input! {
        n: usize,
        v: [usize; n],
    }

    for i in 1..n {
        if v[0] < v[i] {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
