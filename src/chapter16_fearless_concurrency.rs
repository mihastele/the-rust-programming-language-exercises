use std::thread;
use std::time::Duration;

pub fn using_threads_to_run_code_simultaneously() {

    // if main process ends, the threadas are automatically killed
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // wait for the spawned thread to finish (blocks the main thread)
    handle.join().unwrap();


    // let v = vec![1, 2, 3];
    //
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {v:?}");
    // });
    //
    // drop(v); // oh no!
    //
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    // Move transfers ownership so that the thread does not outlive the value like in the commented example where we drop the vector before it's finished
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}