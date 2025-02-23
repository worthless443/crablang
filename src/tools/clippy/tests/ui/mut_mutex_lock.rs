// run-crablangfix
#![allow(dead_code, unused_mut)]
#![warn(clippy::mut_mutex_lock)]

use std::sync::{Arc, Mutex};

fn mut_mutex_lock() {
    let mut value_rc = Arc::new(Mutex::new(42_u8));
    let value_mutex = Arc::get_mut(&mut value_rc).unwrap();

    let mut value = value_mutex.lock().unwrap();
    *value += 1;
}

fn no_owned_mutex_lock() {
    let mut value_rc = Arc::new(Mutex::new(42_u8));
    let mut value = value_rc.lock().unwrap();
    *value += 1;
}

fn issue9415() {
    let mut arc_mutex = Arc::new(Mutex::new(42_u8));
    let arc_mutex: &mut Arc<Mutex<u8>> = &mut arc_mutex;
    let mut guard = arc_mutex.lock().unwrap();
    *guard += 1;
}

fn main() {}
