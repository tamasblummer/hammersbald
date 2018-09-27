extern crate blockchain_store;
extern crate rand;
extern crate simple_logger;
extern crate log;

use blockchain_store::infile::InFile;
use blockchain_store::bcdb::BCDBFactory;

use rand::{thread_rng, Rng};

use std::time::{Instant};

pub fn main () {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let mut db = InFile::new_db("testdb").unwrap();
    db.init().unwrap();

    // transaction size assumed 300 bytes
    let data = [0x0u8;300];

    // simulating a blockchain ingest

    // number of transactions
    let ntx = 500000000;
    // transactions per block
    let tb = 1000;
    // load batch size (in number of blocks)
    let bat = 1000;

    // check keys
    let mut check = Vec::with_capacity((ntx as usize)/100);


    println!("Inserting data ...");
    let mut n = 0;
    let mut now = Instant::now();
    let mut elapsed;
    let mut key = [0u8;32];
    for i in 0 .. ntx {
        thread_rng().fill(&mut key);
        if i % 100 == 0 {
            check.push (key.clone());
        }
        db.put(&key, &data).unwrap();
        n += 1;

        if n % (bat*tb) == 0 {
            db.batch().unwrap();
            elapsed = now.elapsed().as_secs();
            println!("Stored {} million transactions in {} seconds, {} inserts/second.", n/1000000, elapsed, n/elapsed);
        }
    }

    db.batch().unwrap();
    elapsed = now.elapsed().as_secs();
    println!("Stored {} million transactions in {} seconds, {} inserts/second ", ntx/1000000, elapsed, ntx/elapsed);

    println!("Shuffle keys...");
    thread_rng().shuffle(&mut check);
    println!("Reading data in random order...");
    now = Instant::now();
    for key in &check {
        db.get(key).unwrap();
    }
    elapsed = now.elapsed().as_secs();
    if elapsed > 0 {
        println!("Read {} million transactions in {} seconds, {} read/second ", ntx / 1000000, elapsed, ntx / elapsed);
    }

    db.shutdown();
}