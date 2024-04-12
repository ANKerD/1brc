#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use std::{collections::HashMap, env, fs::File, io::{BufRead, BufReader}, thread};
use itertools::Itertools;

#[inline(always)]
fn eval_char(c: char) -> i64 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 10,
    }
}

#[inline(always)]
fn parse_measure(txt: &str) -> i64 {
    let mut ans = 0;
    let mut neg= false;
    for c in txt.chars() {
        if c == '-' {
            neg = true
        } else if c != '.' {
            ans = ans * 10 + eval_char(c);
        }
    }
    if neg {
        return -ans as i64;
    } else {
        return ans as i64;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    
    let filepath = args[1].trim();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut map: FxHashMap<String, [i64; 4]> = FxHashMap::with_capacity_and_hasher(10000, Default::default());
    
    for line in reader.lines() {
        let liner = line.unwrap();
        let l: Vec<&str> = liner.split(";").collect();
        let v = parse_measure(l[1]);
        
        let station = map.entry(l[0].to_string()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
        station[0] = station[0].min(v);
        station[1] = station[1].max(v);
        station[2] = station[2] + v;
        station[3] = station[3] + 1;
    }

    // let mut hlds = vec![];

    // for i in 0..8 {
    //     let t = thread::spawn(move || {
    //         (move |j: i32| {
    //             println!("{}", j);
    //             return Box::new([j])
    //         })(i);   
    //     });
    //     hlds.push(t);
    // }

    // for t in hlds {
    //     _ = t.join();
    // }

    for key in map.keys().sorted() {
        let [min, max, sum, count] = map[key];
        let mean = (sum as f32) / (count as f32) /10.0;
        println!("{key}={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
    }
}