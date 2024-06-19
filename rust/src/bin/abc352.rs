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
        s: String,
        t: String,
    }

    let mut num = 0;
    let mut v = Vec::new();
    let s_chars: Vec<char> = s.chars().collect();

    for (i, c) in t.char_indices() {
        if num < s_chars.len() && s_chars[num] == c {
            v.push(i + 1);
            num += 1;
        }
    }

    for e in v {
        print!("{} ", e);
    }
}

fn a() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    let mut low_num = 0;
    let mut high_num = 0;

    if x > y {
        low_num = y;
        high_num = x;
    } else {
        low_num = x;
        high_num = y;
    }

    for i in low_num..=high_num {
        if i == z {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
