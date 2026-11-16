use std::sync::Mutex;

pub fn inc_mutex(mutex: &Mutex<usize>) -> usize {
    *mutex.lock().unwrap() += 1;
    *mutex.lock().unwrap()
}