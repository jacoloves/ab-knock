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
    a()
}

fn a() {
    input! {
        h: usize,
    }

    let mut cnt = 0;
    let mut hight = 0;
    loop {
        hight += 2usize.pow(cnt);
        if hight > h {
            break;
        }
        cnt += 1;
    }
    println!("{}", cnt + 1);
}
