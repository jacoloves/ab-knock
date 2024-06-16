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
        a: usize,
        t: [usize; n],
    }

    let mut sum = t[0] + a;
    for i in 1..n {
        println!("{sum}");
        if sum >= t[i] {
            sum += a;
        } else {
            sum = t[i] + a;
        }
    }
    println!("{sum}");
}

fn a() {
    input! {
        s: String,
        t: String,
    }

    if s == "AtCoder" && t == "Land" {
        println!("Yes");
    } else {
        println!("No");
    }
}
