use std::{
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

fn child_routine(tx: Sender<i32>, start: i32, step: usize) {
    for v in start..start + step as i32 {
        match tx.send(v) {
            Ok(_) => println!("Child: sent {}", v),
            Err(e) => panic!("Child didn't send: {}", e),
        };
    }
}

fn main() {
    let (tx, rx) = channel::<i32>();
    let tx1 = Sender::clone(&tx);
    let tx2 = Sender::clone(&tx);
    drop(tx); // ... or the program will not end
    let child1 = spawn(move || child_routine(tx1, 1, 5));
    let child2 = spawn(move || child_routine(tx2, 7, 5));
    loop {
        match rx.recv() {
            Ok(v) => println!("Parent: recv {}", v),
            Err(_) => break,
        }
    }

    for i in vec![child1, child2].drain(..) {
        i.join().unwrap();
    }
}
