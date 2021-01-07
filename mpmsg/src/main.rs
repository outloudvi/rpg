use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread::spawn,
};

fn child_routine(tx: Sender<i32>, start: i32, step: usize, cnt: Arc<Mutex<i32>>) {
    for v in start..start + step as i32 {
        match tx.send(v) {
            Ok(_) => {
                println!("Child: sent {}", v);
                *cnt.lock().unwrap() += 1;
            }
            Err(e) => panic!("Child didn't send: {}", e),
        };
    }
}

static STEP_SETTING: usize = 5;

fn main() {
    let cnt = Arc::new(Mutex::new(0));
    let (tx, rx) = channel::<i32>();
    let mut thread_pool = vec![];
    for i in 0..2 {
        let this_tx = tx.clone();
        let this_cnt = cnt.clone();
        let child = spawn(move || {
            child_routine(this_tx, i * (STEP_SETTING as i32), STEP_SETTING, this_cnt)
        });
        thread_pool.push(child);
    }
    drop(tx); // ... or the program will not end
    loop {
        match rx.recv() {
            Ok(v) => println!("Parent: recv {}", v),
            Err(_) => break,
        }
    }

    for i in thread_pool.drain(..) {
        i.join().unwrap();
    }
    println!("Parent: cnt is {}", *cnt.lock().unwrap());
}
