#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::{FxHashMap};
use std::{env, fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

const MIN_MAX_OFFSET: i128 = 1000;
const MASK_16: i128 = 0xFFFF;
const MASK_32: i128 = 0xFFFFFFF;

fn eval_char(c: char) -> i32 {
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
fn parse_measure(txt: &str) -> i128 {
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
        return -ans as i128;
    } else {
        return ans as i128;
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

    // let mut map: FxHashMap<String, [i64; 4]> = FxHashMap::with_capacity_and_hasher(100000, Default::default());
    let mut map: FxHashMap<String, i128> = FxHashMap::with_capacity_and_hasher(100000, Default::default());
    // let mut map: HashMap<String, (i32, i32, i128, i32)> = HashMap::with_capacity_and_hasher(100000, Default::default());
    for line in reader.lines() {
        let liner = line.unwrap();
        let lines: Vec<&str> = liner.split(";").collect();
        let v = parse_measure(lines[1]);
        
        // let station = map.entry(lines[0].to_string()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
        // station[0] = station[0].min(v);
        // station[1] = station[1].max(v);
        // station[2] = station[2] + v;
        // station[3] = station[3] + 1;
        let station = map.entry(lines[0].to_string()).or_insert( 0xFFFF);
        let min = i128::min(v + MIN_MAX_OFFSET, MASK_16 & *station);
        let max = i128::min(v + MIN_MAX_OFFSET, MASK_16 & (*station >> 16));
        let count = (MASK_32 & (*station >> 32)) +1;
        let sum = (*station >> 64)+v;
        *station = (sum << 64) | (count << 32) | (max << 16) | (min);
    }

    // for key in map.keys().sorted() {
    //     let [min, max, sum, count] = map[key];
    //     let mean = (sum as f32) / (count as f32) /10.0;
    //     println!("{key}={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
    // }

    for key in map.keys().sorted() {
        let station = map[key];
        let min = (MASK_16 & station) -MIN_MAX_OFFSET;
        let max = (MASK_16 & (station >> 16)) -MIN_MAX_OFFSET;
        let count = MASK_32 & (station >> 32);
        let sum = station >> 64;

        let mean = (sum as f32) / (count as f32) /10.0;
        println!("{key}={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
    }
}