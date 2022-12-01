use std::thread;
use std::time::Instant;
use std::sync::{Arc, Mutex};
fn main() {
    let mut thread_vec = vec![];
    let now = Instant::now();
    let result_vec = Arc::new(Mutex::new(Vec::<i128>::new()));
    let number_of_thread = 16;
    let bigsum = 10000000000;
    for index in 1..(number_of_thread + 1) {
        let smaller_arc = result_vec.clone();
        let thread_state = thread::spawn(move || {
            let start = (index - 1)*(bigsum / number_of_thread) + 1;
            let end;
            if index == number_of_thread {
                end = bigsum;
            } else { 
                end = index*(bigsum / number_of_thread);
            }
            // println!("start: {} end: {}", start, end);
            let mut result= 0;
            for i in start..(end + 1) {
                result += i;
            }
            let mut mutex_guard_result = smaller_arc.lock().unwrap();
            (*mutex_guard_result).push(result);
        });
        thread_vec.push(thread_state);
    }
    for thread_state in thread_vec {
        thread_state.join().unwrap();
    }
    let mut result = 0;
    for i in &*(result_vec.lock().unwrap()) {
        result += i;
    }
    println!("Result: {}", result);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.3?}", elapsed);
}
