use std::{
    process::exit,
    sync::{Arc, RwLock},
    thread,
};

fn reader_loop(lock: &RwLock<()>) {
    loop {
        let _guard = lock.try_read().unwrap();
    }
}

fn writer_exit(lock: &RwLock<()>) {
    let _guard = lock.write().unwrap();
    eprintln!("writer: exit");
    exit(0);
}

fn main() {
    let w_lock = Arc::new(RwLock::new(()));

    for _ in 0..30 {
        // more than the number of physical CPU cores
        let r_lock = w_lock.clone();
        let _r_handle = thread::spawn(move || reader_loop(&r_lock));
    }

    let w_handle = thread::spawn(move || {
        writer_exit(w_lock.as_ref());
    });

    w_handle.join().unwrap();
}
