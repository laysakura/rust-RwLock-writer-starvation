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
    let r_lock = Arc::new(RwLock::new(()));
    let w_lock = r_lock.clone();

    let r_handle = thread::spawn(move || {
        reader_loop(r_lock.as_ref());
    });

    let w_handle = thread::spawn(move || {
        writer_exit(w_lock.as_ref());
    });

    r_handle.join().unwrap();
    w_handle.join().unwrap();
}
