use std::{sync::mpsc, thread::spawn};

fn main() {
    let (tx, rx) = mpsc::channel();
    let child = spawn(move || {
        let v = 4;
        match tx.send(v) {
            Ok(_) => println!("Child: sent {}", v),
            Err(e) => eprintln!("Child didn't send: {}", e),
        };
    });
    println!("Parent: recv {}", rx.recv().unwrap());
    child.join().unwrap();
}
