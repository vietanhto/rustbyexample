use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

static NTHREAD: i32 = 8;

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..NTHREAD {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id)
        });
    }

    let mut ids = Vec::with_capacity(NTHREAD as usize);
    for _ in 0..NTHREAD {
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}
