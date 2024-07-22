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
        t: String
    }

    let s_len = s.len();
    for w in 1..s_len {
        for c in 0..w {
            let mut now = String::new();
            let mut i = c;
            while i < s_len {
                if let Some(ch) = s.chars().nth(i) {
                    now.push(ch);
                }
                i += w;
            }
            if now == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn a() {
    input! {
        s: String,
    }

    let mut chars = s.chars();

    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let third = chars.next().unwrap();

    if first == 'R' {
        println!("Yes");
        return;
    }

    if first == 'M' {
        println!("No");
        return;
    }

    if first == 'S' && second == 'M' {
        println!("No");
        return;
    }

    println!("Yes");
}
