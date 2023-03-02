use ::std::{
    sync::atomic::{AtomicIsize, Ordering::SeqCst},
    thread,
};

fn main() {
    static X: AtomicIsize = AtomicIsize::new(10);

    thread::spawn(|| {
        //the mutation happens here:
        loop {
            X.fetch_add(1, SeqCst);
            println!("thread 2: {}", X.load(SeqCst));
        }
    });

    // but the value can still be read.
    loop_function(&X);
}

fn loop_function(x: &'_ AtomicIsize) {
    loop {
        println!("thread 1: {}", x.load(SeqCst));
    }
}
