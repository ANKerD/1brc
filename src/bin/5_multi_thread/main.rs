#[global_allocator]
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate fxhash;
use fxhash::FxHashMap;
use std::{ env, fs::File, io::{stdout, Read, Seek, SeekFrom, Write}, thread};
use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};

const THREAD_COUNT: usize = 16;
const CHUNK_SIZE: u64 = 1024 * 1024 * 4;

type Answer = FxHashMap::<Vec<u8>, [i64; 4]>;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("Wrong parameters provided")
    }
    
    let filepath = args[1].trim().to_owned();
    let (answer_sender, answer_receiver): (Sender<Answer>, Receiver<Answer>) = bounded(0);
    
    for j in 0..THREAD_COUNT {
        let fp = filepath.clone();
        let sender = answer_sender.clone();
        let worker = move |thread_id| {
            let mut map = Answer::with_capacity_and_hasher(10000, Default::default());
            
            let mut f = File::open(fp.clone()).unwrap();
            let file_size = f.metadata().unwrap().len() as usize;
            let start: usize = file_size / THREAD_COUNT * thread_id;
            let end: usize = if thread_id+1 == THREAD_COUNT{ file_size } else {file_size / THREAD_COUNT * (thread_id+1)};
            // println!("1. {start} {end} {file_size} {thread_id}");
            
            let _ = f.seek(SeekFrom::Start(start.try_into().unwrap())).unwrap();
            let mut buf = vec![0_u8; CHUNK_SIZE.try_into().unwrap()];
            let mut cnt = 0;

            cnt += f.read(&mut buf).unwrap();

            let mut index = 0;
            if thread_id != 0 {
                while buf[index] != b'\n' {
                    index += 1;
                }
                index += 1;
            }
            let mut txt_buffer: Vec<u8> = Vec::with_capacity(30);
            while start + cnt < end {
                let mut measure: i32 = 0;
                let mut semicolon = false;
                let mut neg = false;
                loop {
                    let c = buf[index];
                    // println!("[{thread_id}] index={index} c={c} {}", c as char);
                    
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
                    
                    index += 1;
                    
                    if index >= buf.len() {
                        cnt += f.read(&mut buf).unwrap();
                        index = 0;
                    }
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