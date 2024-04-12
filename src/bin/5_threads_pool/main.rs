#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use std::{ env, fs::File, io::{BufRead, BufReader}, thread, time::Duration};
use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};


const CPU_COUNT: usize = 8;

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
    
    let filepath = args[1].trim();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut handlers = vec![];
    for _ in 0..CPU_COUNT {
        let (data_sender, data_receiver): (Sender<Vec<String>>, Receiver<Vec<String>>) = bounded(CPU_COUNT);
        let (answer_sender, answer_receiver): (Sender<Answer>, Receiver<Answer>) = bounded(CPU_COUNT);
    
        let t = thread::spawn(move || {
            let mut map: Answer = Answer::with_capacity_and_hasher(10000, Default::default());
            while let Ok(data) = data_receiver.recv_timeout(Duration::from_millis(100)) {
                for line in data {
                    let l: Vec<&str> = line.split(";").collect();
                    let v = parse_measure(l[1]);
                    
                    let station = map.entry(l[0].to_string()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
                    station[0] = station[0].min(v);
                    station[1] = station[1].max(v);
                    station[2] = station[2] + v;
                    station[3] = station[3] + 1;
                }
            }
            let _ = answer_sender.send(map);
        });
        handlers.push((t, data_sender, answer_receiver));
    }

    // let mut map: FxHashMap<String, [i64; 4]> = FxHashMap::with_capacity_and_hasher(10000, Default::default());
    let mut i = 0;
    for chunk in &reader.lines().chunks(128) {
        let data = chunk.map(|l| {l.unwrap()}).collect::<Vec<String>>();
        let (_, data_sender, _) = &handlers[i];
        let _ = data_sender.send(data).unwrap();
        i = (i+1) % 8;
    }

    let map: Answer = Answer::with_capacity_and_hasher(10000, Default::default());
    for (t, _, answer_receiver) in handlers {
        // drop(tx);
        t.join().unwrap();
        let partial = answer_receiver.recv().unwrap();
        for key in map.keys() {
            let mut station = map[key];
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