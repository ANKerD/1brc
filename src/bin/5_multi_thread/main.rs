#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use std::{ env, fs::File, io::{BufRead, BufReader}, thread};
use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};

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
fn parse_measure(txt: &Vec<u8>, i: usize) -> i64 {
    let mut ans: i64 = 0;
    let mut neg= false;
    for j in i..txt.len() {
        let c = txt[i];
        if c == b'-' {
            neg = true
        } else if c != b'.' {
            ans = (ans * 10).wrapping_add((c - b'0').into());
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
    // println!("fredo fred");
    
    let filepath = args[1].trim();
    let file = File::open(filepath).unwrap();
    let mut reader = BufReader::new(file);

    // let (data_sender, data_receiver): (Sender<Result<String, _>>, Receiver<Result<String, _>>) = bounded(10000);
    let (data_sender, data_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = bounded(10000);
    let (answer_sender, answer_receiver): (Sender<u32>, Receiver<u32>) = bounded(0);
    let _ = thread::spawn(move || {
        let mut map = FxHashMap::<Vec<u8>, [i64; 4]>::with_capacity_and_hasher(10000, Default::default());
        while let Ok(line) = data_receiver.recv() {
            // let mut key: u128 = 0;
            let mut i = 0; 
            while line[i] != b';'{
                i += 1;
            }
            
            let v = parse_measure(&line, i);
            // let liner = line.unwrap();
            // let l: Vec<&str> = liner.split(";").collect();
            // let v = parse_measure(l[1]);
            
            let station = map.entry(line[0..i].to_vec()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
            station[0] = station[0].min(v);
            station[1] = station[1].max(v);
            station[2] = station[2] + v;
            station[3] = station[3] + 1;
        }
        
        for key in map.keys().sorted() {
            let [min, max, sum, count] = map[key];
            let mean = (sum as f32) / (count as f32) /10.0;
            // let name = key.;
            println!("{:?}={:.1}/{:.1}/{mean:.1}", key, min as f32 /10.0, max as f32 /10.0);
        }
        let _ = answer_sender.send(0);
        drop(answer_sender);
    });
    
    let mut buf = vec![];
    let _a = reader.read_until(b'\n', &mut buf);
    for l in reader.read_until(b'\n', &mut buf) {
        println!("{:?}", l);
        let _ = data_sender.send(buf.clone());
    };
    // let read_until = reader.read_until(b'\n', &mut &buf);

    drop(data_sender);
    let _ = answer_receiver.recv();
}