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
        v: [usize; n*2],
    }

    let mut cnt = 0;
    for i in 2..n * 2 {
        if v[i] == v[i - 2] {
            cnt += 1;
        }
    }

    println!("{cnt}");
}

fn a() {
    input! {
        n: usize,
        sv: [String; n],
    }

    let target = "Takahashi".to_string();

    let mut cnt = 0;
    for e in sv {
        if e == target {
            cnt += 1;
        }
    }

    println!("{cnt}");
}
