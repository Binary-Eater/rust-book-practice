use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    /* Mutex corner case
     *
     * If a thread holding the mutex crashes, locking the mutex will result in
     * an error rather than being blocked forever.
     *
     * This example naively uses unwrap to handle the result. In production,
     * safer error handling should be considered.
     */
    {
        let mut num = m.lock().unwrap(); // num is a MutexGuard<i32> that implements Deref and Drop
        *num = 6;
        /* When num gets dropped, the lock is released. */
    }

    println!("m = {m:?}");

    /* Need shared ownership of the mutex value
     * let counter = Mutex::new(0);
     * let mut handles = vec![];
     *
     * for _ in 0..10 {
     *     let handle = thread::spawn(move || {
     *         let mut num = counter.lock().unwrap();
     *
     *         *num += 1;
     *     });
     *     handles.push(handle);
     * }
     *
     * for handle in handles {
     *     handle.join().unwrap();
     * }
     *
     * println!("Result: {}", *counter.lock().unwrap());
     */

    /* Rc is not atomic
     *
     * use std::rc::Rc;
     *
     * let counter = Rc::new(Mutex::new(0));
     * let mut handles = vec![];
     *
     * for _ in 0..10 {
     *     let counter = Rc::clone(&counter);
     *     let handle = thread::spawn(move || {
     *         let mut num = counter.lock().unwrap();
     *
     *         *num += 1;
     *     });
     *     handles.push(handle);
     * }
     *
     * for handle in handles {
     *     handle.join().unwrap();
     * }
     *
     * println!("Result: {}", *counter.lock().unwrap());
     */

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
