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
        mut scv: [(String, usize); n],
    }

    // Stringの部分で辞書順にソート
    scv.sort_by_key(|k| k.0.clone());

    // 合計値を計算
    let mut cnt = 0;
    for (_, v) in &scv {
        cnt += v;
    }

    // nで割った余りを計算
    let mod_cnt = cnt % n;

    println!("{}", scv[mod_cnt].0.clone());
}
