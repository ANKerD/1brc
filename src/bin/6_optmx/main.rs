#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
// static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

extern crate fxhash;
// use fxhash::FxHashMap;
use std::{ env, fs::File, io::{ stdout, Read, Seek, SeekFrom, Write}, thread};
// use itertools::Itertools;
use crossbeam_channel::{bounded, Receiver, Sender};
use itertools::Itertools;

const BUFFER_OFFSET: usize = 100;

type Unit = i64;
const M1: Unit = 10039;
const M2: Unit = 10061;
const SZ: usize = 300 + M2 as usize;
type Record = [Unit; 5];
// type Answer<'a> = (Record, Vec<u8>);

#[derive(Debug)]
struct Answer {
    record: Record,
    key: &'static [u8],
}

impl Answer {
    fn new() -> Self {
        Self { record: [Unit::MAX, Unit::MIN, 0, 0, 0], key: &[] }
    }
    fn default() -> Self {
        // Initialize each vector with an empty Vec
        Self {
            record: [Unit::MAX, Unit::MIN, 0, 0, 0],
            key: &[],
            // key: 
        }
    }
}

impl Default for Answer {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Answer {}

impl Clone for Answer {
    fn clone(&self) -> Self {
        *self
    }
}

fn hash(v: &Vec<u8>, md: Unit) -> Unit {
    let mut ans: Unit = 0;
    for x in v {
        ans = (ans * 37 + (*x as Unit)) % md;
    }
    return ans;
}

fn get_idx(arr: &mut [Answer; SZ], key: &Vec<u8>) -> usize {
    let mut h1 = hash(key, M1) as usize;
    let h2 = hash(key, M2);
    while arr[h1].record[4] != 0 && arr[h1].record[4] != h2 {
        h1 += 1;
    }
    if arr[h1].record[4] == 0 {
        arr[h1].record = [Unit::MAX, Unit::MIN, 0, 0, 0];
    }
    arr[h1].record[4] = h2;
    return h1;
}

// fn put(arr: &mut [&Answer; SZ], key: &Vec<u8>, record: &Record) {
//     let idx = get_idx(arr, key);
//     arr[idx].record = *record;
//     arr[idx].key = key;
// }

// fn add(arr: &mut [&mut Answer; SZ], key: &Vec<u8>, value: Unit) {
//     let mut v = arr[get_idx(arr, key)];
//     v.record[0] = v.record[0].min(value);
//     v.record[1] = v.record[1].max(value);
//     v.record[2] = v.record[2] + value;
//     v.record[3] = v.record[3] + 1;
// }

// fn new() -> Answer<'static> {
//     return ([Unit::MAX, Unit::MIN, 0, 0, 0], vec![]);
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        panic!("Wrong parameters provided")
    }

    let threads_count: usize = args[2].trim().parse::<usize>().unwrap();
    let chunk_size: usize = 1024 * args[3].trim().parse::<usize>().unwrap() - BUFFER_OFFSET;
    
    let filepath = args[1].trim().to_owned();
    let (answer_sender, answer_receiver): (Sender<Box<Vec<Answer>>>, Receiver<Box<Vec<Answer>>>) = bounded(0);
    
    for j in 0..threads_count {
        let fp = filepath.clone();
        let sender = answer_sender.clone();
        let worker = move |thread_id| {
            // println!("[{thread_id}] start");
            // let mut andiso = &Answer { record: [0,0,0,0,0], key: &vec![] };
            let map= &mut [Answer::default(); SZ];
            // println!("[{thread_id}] allocated map");
            let mut f = File::open(fp.clone()).unwrap();
            let file_size = f.metadata().unwrap().len() as usize;
            let mut start: usize = chunk_size * thread_id;
            let mut end: usize = (start + chunk_size).min(file_size);
            // println!("[{thread_id}] 1. start={start} end={end}");
            while start  < end && end <= file_size  {
                let _ = f.seek(SeekFrom::Start(start.try_into().unwrap()));
                let mut buf = vec![0_u8; chunk_size + BUFFER_OFFSET]; // Add offset
                let mut index = 0;
            
                // println!("[{thread_id}] 2. start={start} end={end}");
                let _ = f.read(&mut buf).unwrap();

                while start > 0 && buf[index] != b'\n' && start + index < file_size {
                    index += 1;
                } 
                if buf[index] == b'\n' {
                    index += 1
                }
                let mut txt_buffer: Vec<u8> = Vec::with_capacity(30);
                while start + index < end && end <= file_size || start + index < end +1 && end != file_size {
                    // println!("[{thread_id}] 1. start={start} end={end} index={index}");
                    let mut measure: Unit = 0;
                    let mut semicolon = false;
                    let mut neg = false;
                    loop {
                        let c = buf[index];
                        // println!("[{thread_id}] start={start} c={c} {}", c as char);
                        
                        if c == b';' {
                            semicolon = true
                        } else if c == b'-' { 
                            neg = true;
                        } else if c == b'.' || c == b'\n' { 
                        } else if !semicolon {
                            txt_buffer.push(c);
                        } else if semicolon {
                            measure = measure * 10 + (c - b'0') as Unit;
                        } else {
                            println!("bad state reading c={c}");
                        }
                        index += 1;
                        if c == b'\n' {break;}
                    }
                    if neg { measure = -measure; }
                    // print!("[{thread_id}] 1. start={start} end={end} index={index} ");
                    // let _ = stdout().write(&txt_buffer);
                    // println!(" {measure}");
                        
                    // let station = get(&map, &txt_buffer);
                    let idx = get_idx(map, &txt_buffer.clone());
                    let v = map.get_mut(idx).unwrap();
                    v.key = Box::leak(Box::new(txt_buffer.clone()));
                    // println!("[{thread_id}] 2. key={:?}", v.key);
                    v.record[0] = v.record[0].min(measure);
                    v.record[1] = v.record[1].max(measure);
                    v.record[2] = v.record[2] + measure;
                    v.record[3] = v.record[3] + 1;
                    txt_buffer.clear();
                    // println!("[{thread_id}] 3. key={:?}", v.key);

                }
                start = start + threads_count * chunk_size;
                end = (start + chunk_size).min(file_size);
            }
            
            sender.send(Box::new(map.to_vec())).unwrap();
            drop(sender);
        };
        let _ = thread::spawn(move || {worker(j)});
    }
    
    // let mut map = &mut [Answer::default(); SZ];
    let mut map: Vec<Answer> = Vec::with_capacity(SZ);
    map.resize(SZ, Answer::new());
    for _ in 0..threads_count {
        let partial = answer_receiver.recv().unwrap();
        for (mut i, value) in partial.iter().enumerate() {
            let mut station = map.get_mut(i).unwrap();
            while station.record[4] != 0 && station.record[4] != value.record[4] {
                i += 1;
                station = map.get_mut(i).unwrap();
            }
            station.record[0] = station.record[0].min(value.record[0]);
            station.record[1] = station.record[1].max(value.record[1]);
            station.record[2] = station.record[2] + value.record[2];
            station.record[3] = station.record[3] + value.record[3];
            station.record[4] = value.record[4];
            station.key = value.key;
            // if value.record[4] != 0 || value.key.len() > 0 {
            //     println!("i={i}");
            //     println!("{:?}", value);
            //     println!("{:?}\n", station);
            // }
        }
    }
    // println!("fredo {:?}\n", map[9985]);

    let keys = map.iter().enumerate().map(|(index, st), | {(st.key, index)}).sorted();
    
    for (key, index) in keys.sorted() {
        if key.len() > 0 {
            let _ = stdout().write(key);
            let [min, max, sum, count, _] = map[index].record;
            let mean = (sum as f32) / (count as f32) /10.0;
            println!("={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
            // println!("key={:?} index={index}", key);
        }
        // let [min, max, sum, count] = map[key];
        // // let mean = (sum as f32) / (count as f32) /10.0;
        // let mean = (sum as f32) / (count as f32) /10.0;
        // let _ = stdout().write(key);
        // println!("={:.1}/{:.1}/{mean:.1}", min as f32 /10.0, max as f32 /10.0);
   }
}