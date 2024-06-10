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
        s: String,
    }

    let mut up_cnt = 0;
    let mut low_cnt = 0;

    for ch in s.chars() {
        if ch.is_uppercase() {
            up_cnt += 1;
        } else {
            low_cnt += 1;
        }
    }

    if up_cnt > low_cnt {
        let uppser_string: String = s
            .chars()
            .map(|c| c.to_uppercase().collect::<String>())
            .collect();
        println!("{uppser_string}");
    } else {
        let lower_string: String = s
            .chars()
            .map(|c| c.to_lowercase().collect::<String>())
            .collect();
        println!("{lower_string}");
    }
}

fn a() {
    input! {
        n: usize,
        mut m: isize,
        h: [isize; n],
    }

    let mut cnt = 0;

    for e in h {
        if m - e >= 0 {
            cnt += 1;
            m -= e;
        } else {
            break;
        }
    }

    println!("{cnt}");
}
