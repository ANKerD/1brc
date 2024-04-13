#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use memmap::MmapOptions;
use std::{ env, fs::File, io::{stdout, Read, Seek, SeekFrom, Write}, thread};
use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};

const THREAD_COUNT: usize = 16;
const CHUNK_SIZE: usize = 1024 * 1024 * 4;

type Answer = FxHashMap::<Vec<u8>, [i64; 4]>;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    
    let filepath = args[1].trim().to_owned();
    let (answer_sender, answer_receiver): (Sender<Answer>, Receiver<Answer>) = bounded(0);
    
    let f = File::open(filepath.clone()).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&f).unwrap() };
    let file_size = mmap.len() as usize;
    
    let (chunk_sender, chunk_receiver): (Sender<(usize, usize)>, Receiver<(usize, usize)>) = bounded(0);
    let mut start: usize = 0;
    let mut end: usize = (start + CHUNK_SIZE).min(file_size);
    while start < mmap.len() {
        while end < file_size && mmap[end] != b'\n' {
            end += 1;
        }
        chunk_sender.send((start, end)).unwrap();
        start = end + 1;
    }
    drop(chunk_sender);

    for j in 0..THREAD_COUNT {
        let sender = answer_sender.clone();
        let receiver = chunk_receiver.clone();
        let mmap = unsafe { MmapOptions::new().map(&f).unwrap() };
        let worker = move |_thread_id| {
            let mut map = Answer::with_capacity_and_hasher(10000, Default::default());
            let (mut start, end);
            match receiver.recv() {
                Ok(chunk) => {
                    (start, end) = chunk;
                }
                Err(_) => return
            }
            let mut txt_buffer: Vec<u8> = Vec::with_capacity(30);
            while start < end {
                let mut measure: i32 = 0;
                let mut semicolon = false;
                let mut neg = false;
                loop {
                    let c = mmap[start];
                    // println!("[{thread_id}] start={start} c={c} {}", c as char);
                    
                    if c == b';' {
                        semicolon = true
                    } else if c == b'-' { 
                        neg = true;
                    } else if c == b'.' || c == b'\n' { 
                    } else if !semicolon {
                        txt_buffer.push(c);
                    } else if semicolon {
                        measure = measure * 10 + (c - b'0') as i32;
                    } else {
                        println!("bad state reading c={c}");
                    }
                    
                    start += 1;
                    if c == b'\n' {break;}
                }
                if neg { measure = -measure; }

                // TODO review this clone
                let station = map.entry(txt_buffer.clone()).or_insert( [i64::MAX, i64::MIN, 0, 0]);
                station[0] = station[0].min(measure as i64);
                station[1] = station[1].max(measure as i64);
                station[2] = station[2] + measure as i64;
                station[3] = station[3] + 1;
                txt_buffer.clear()
            }
            sender.send(map).unwrap();
            drop(sender);
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
   }
}