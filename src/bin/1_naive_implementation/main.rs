use std::{collections::BTreeMap, env, fs::File, io::{BufRead, BufReader}};

use rand_distr::num_traits::ToPrimitive;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    
    let filepath = args[1].trim();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    
    let mut map: BTreeMap<String, (f32, f32, f32, i32)> = BTreeMap::new();
    for line in reader.lines() {
        let liner = line.unwrap();
        let lines: Vec<&str> = liner.split(";").collect();
        let (name, value) = (lines[0], lines[1]);
        
        map.entry(name.to_string()).or_insert( (f32::MAX, f32::MIN, 0.0, 0));
        let v = value.parse::<f32>().unwrap(); 
        
        let station = map.get_mut(name).unwrap();
        station.0 = station.0.min(v);
        station.1 = station.1.max(v);
        station.2 = station.2 + v;
        station.3 = station.3 + 1;
    }

    // Compute solution
    for (name, (min, max, sum, count)) in map {
        let mean = sum / count.to_f32().unwrap();
        println!("{name}={min:.1}/{max:.1}/{mean:.1}");
    }
}