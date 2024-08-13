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

#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::fmt::Debug;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    b();
}

fn b() {
    input! {
     x1: usize, y1: usize, z1: usize,
     x2: usize, y2: usize, z2: usize,
     x3: usize, y3: usize, z3: usize,
     x4: usize, y4: usize, z4: usize,
    }

    fn f(l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
        !(r1 <= l2 || r2 <= l1)
    }

    if f(x1, x2, x3, x4) && f(y1, y2, y3, y4) && f(z1, z2, z3, z4) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn a() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n]
    }

    let mut i = 1;
    for e in a {
        if i == k {
            print!("{} ", e);
            print!("{} ", x);
        } else {
            print!("{} ", e);
        }
        i += 1;
    }
    println!();
}
