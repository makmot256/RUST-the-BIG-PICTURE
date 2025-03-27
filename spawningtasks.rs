use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];

    for i in 0..1000 {
        let handle = thread::spawn(move || {
            let ten_millis = Duration::from_millis(10);
            thread::sleep(ten_millis);
            println!("Task #{} finished", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
