#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use std::{ env, fs::File, io::{BufRead, BufReader}, thread};
use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};

const CPU_COUNT: usize = 1;

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

type Answer = FxHashMap<String, [i64; 4]>;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    // println!("fredo fred");
    
    let filepath = args[1].trim();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let (data_sender, data_receiver): (Sender<Result<String, _>>, Receiver<Result<String, _>>) = bounded(10000);
    let (answer_sender, answer_receiver): (Sender<Box<Answer>>, Receiver<Box<Answer>>) = bounded(CPU_COUNT);
    for _ in 0..CPU_COUNT {
        let receiver = data_receiver.clone();
        let sender = answer_sender.clone();
        let _ = thread::spawn(move || {
            let mut map = Box::new(Answer::with_capacity_and_hasher(10000, Default::default()));
            while let Ok(line) = receiver.recv() {
                let liner = line.unwrap();
                // println!("thread line {liner}");
                let l: Vec<&str> = liner.split(";").collect();
                let v = parse_measure(l[1]);
                
                let station = map.entry(l[0].to_string()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
                station[0] = station[0].min(v);
                station[1] = station[1].max(v);
                station[2] = station[2] + v;
                station[3] = station[3] + 1;
            }
            
            let _ = sender.send(map.clone());
            drop(sender);
        });
    }

    for l in reader.lines(){
        let _ = data_sender.try_send(l);
    };
    drop(data_sender);

    let mut map: Answer = Answer::with_capacity_and_hasher(10000, Default::default());
    for _ in 0..CPU_COUNT {
        let partial = answer_receiver.recv().unwrap();
        for key in partial.keys() {
            let station = map.entry(key.to_string()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
            let partial_station = partial[key];

            station[0] = station[0].min(partial_station[0]);
            station[1] = station[1].max(partial_station[1]);
            station[2] = station[2] + partial_station[2];
            station[3] = station[3] + partial_station[3];
        }
    }
    for key in map.keys().sorted() {
        let [min, max, sum, count] = map[key];
        let mean = (sum as f32) / (count as f32) /10.0;
        println!("{key}={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
    }
}