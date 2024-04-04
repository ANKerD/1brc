#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{collections::HashMap, env, fs::File, io::{BufRead, BufReader}};

use rand_distr::num_traits::ToPrimitive;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    
    let filepath = args[1].trim();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    
    let mut map: HashMap<String, (f32, f32, f32, i32)> = HashMap::new();
    for line in reader.lines() {
        let liner = line.unwrap();
        let lines: Vec<&str> = liner.split(";").collect();
        let v = lines[1].parse::<f32>().unwrap(); 
        
        let station = map.entry(lines[0].to_string()).or_insert( (f32::MAX, f32::MIN, 0.0, 0));
        station.0 = station.0.min(v);
        station.1 = station.1.max(v);
        station.2 = station.2 + v;
        station.3 = station.3 + 1;
    }

    for key in map.keys().sorted() {
        let (min, max, sum, count) = map[key];
        let mean = sum / count.to_f32().unwrap();
        println!("{key}={min:.1}/{max:.1}/{mean:.1}");
    }
}