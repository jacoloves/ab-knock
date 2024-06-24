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
    a();
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
