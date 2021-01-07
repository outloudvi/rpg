use std::{
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

fn child_routine(tx: Sender<i32>) {
    let v = 4;
    match tx.send(v) {
        Ok(_) => println!("Child: sent {}", v),
        Err(e) => eprintln!("Child didn't send: {}", e),
    };
}

fn main() {
    let (tx, rx) = channel::<i32>();
    let child = spawn(move || child_routine(tx));
    println!("Parent: recv {}", rx.recv().unwrap());
    child.join().unwrap();
}
