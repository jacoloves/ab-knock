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
        mut n: usize,
        q: usize,
        t: [usize; q],
    }

    let mut hm = HashMap::new();

    for i in t {
        *hm.entry(i).or_insert(0) += 1;
    }

    for (_, v) in hm.iter() {
        if v % 2 != 0 {
            n -= 1;
        }
    }

    println!("{}", n);
}

fn a() {
    input! {
        s: String,
    }

    if s == "ABC316" {
        println!("No");
        return;
    }

    let re = Regex::new(r"ABC\d+").unwrap();

    if let Some(matched) = re.find(&s) {
        let number: usize = matched.as_str()[3..].parse().unwrap();
        if number < 350 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
