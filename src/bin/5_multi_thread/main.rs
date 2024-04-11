#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use std::{ env, fs::File, io::{stdout, Read, Seek, SeekFrom, Write}, thread};
use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};

const THREAD_COUNT: u64 = 2;
// const CHUNK_SIZE: u64 = 1024 * 1024 * 4; // 4 MB
const CHUNK_SIZE: u64 = 10; // test

fn v8_as_text(arr: &Vec<u8>) {
    let _ = stdout().write(arr);
    println!("");
}

#[inline(always)]
fn parse_measure(txt: &Vec<u8>, index: &usize) -> i64 {
    let mut ans: i64 = 0;
    let mut neg= false;
    match txt[index+1] == b'-' {
        true => {
            neg = true;
        }
        false => {
            ans = (txt[index+1] - b'0').into();
        }
    }
    for i in (index+2)..(txt.len()-1) {
        let c = txt[i];
        if c != b'.' {
            let d: i64 = (c - b'0').into();
            ans = ans * 10 + d;
        }
    }
    if neg {
        return -ans as i64;
    } else {
        return ans as i64;
    }
}

type Answer = FxHashMap::<Vec<u8>, [i64; 4]>;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    // println!("fredo fred");
    
    let filepath = args[1].trim().to_owned();
    // let file = File::open(filepath).unwrap();
    // let mut reader = BufReader::new(file);

    // let (data_sender, data_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = bounded(10000);
    let (answer_sender, answer_receiver): (Sender<Answer>, Receiver<Answer>) = bounded(0);
    let mut h = vec![];
    for _ in 0..THREAD_COUNT {
        let (data_sender, data_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = bounded(10000);
        let sender = answer_sender.clone();
        h.push(data_sender);
        let _ = thread::spawn(move || {
            let mut map = Answer::with_capacity_and_hasher(10000, Default::default());
            while let Ok(line) = data_receiver.recv() {
                // let mut key: u128 = 0;
                let mut i = 0; 
                while line[i] != b';'{
                    i += 1;
                }
                let v = parse_measure(&line, &i);
                
                let station = map.entry(line[0..i].to_vec()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
                station[0] = station[0].min(v);
                station[1] = station[1].max(v);
                station[2] = station[2] + v;
                station[3] = station[3] + 1;
            }

            let _ = sender.send(map);
            drop(sender);
        });
    }
    
    // let mut buf = vec![];
    // while let Ok(l) = reader.read_until(b'\n', &mut buf) {
    //     if l == 0 {
    //         break;
    //     }
    //     let _ = data_sender.try_send(buf.clone());
    //     buf.clear();
    // };
    // drop(data_sender);

    for j in 0..THREAD_COUNT {
        let fp = filepath.clone();
        let worker = move |i| { 
            let mut f = File::open(fp.clone()).unwrap();
            let file_size = f.metadata().unwrap().len();
            let start = file_size / THREAD_COUNT * i;
            let end = if i+1 == THREAD_COUNT{ file_size } else {file_size / THREAD_COUNT * (i+1)};
            println!("{start} {end} {file_size}");
            let seek = f.seek(SeekFrom::Start(start)).unwrap();
            let mut buf = vec![0_u8; CHUNK_SIZE.try_into().unwrap()];
            // let mut buf = vec![];
            loop {
                let n = f.read(&mut buf).unwrap();
                if n == 0 { break; }
                println!("{buf:?}");
                v8_as_text(&buf);
                // println!("{pos} {file_size} {i} {}", b'\n');
                // pos += CHUNK_SIZE;
                
                println!("\n");
            }
        };
        let _ = thread::spawn(move || {worker(j)});
    }
    
    let mut map = Answer::with_capacity_and_hasher(10000, Default::default());
    for _ in 0..THREAD_COUNT {
        let partial = answer_receiver.recv().unwrap();
        for key in partial.keys() {
            let p = partial[key];
            let station = map.entry(key.to_vec()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
            station[0] = station[0].min(p[0]);
            station[1] = station[1].max(p[1]);
            station[2] = station[2] + p[2];
            station[3] = station[3] + p[3];
        }
    }

    for key in map.keys().sorted() {
        let [min, max, sum, count] = map[key];
        // let mean = (sum as f32) / (count as f32) /10.0;
        let mean = (sum as f32) / (count as f32) /10.0;
        let _ = stdout().write(key);
        println!("={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);

        // let mn = i64::abs(sum - mean*count);
        // let _ = stdout().write(key);
        // println!("={}.{}/{}.{}/{}.{}", min/10, i64::abs(min%10), max/10,i64::abs(max%10), mean, mn);
        // println!("{key}={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
    }
}