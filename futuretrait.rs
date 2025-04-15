use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Define a struct that represents an asynchronous computation
struct MyFuture {
    state: i32,
}

// Implement the Future trait for MyFuture
impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state < 5 {
            println!("Not ready yet, current state: {}", self.state);
            self.get_mut().state += 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            println!("Ready, returning result");
            Poll::Ready("Computation complete".to_string())
        }
    }
}

// Example of using the future
fn main() {
    let mut my_future = MyFuture { state: 0 };

    // Create a simple executor to poll the future
    let waker = futures::task::noop_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match Pin::new(&mut my_future).poll(&mut cx) {
            Poll::Pending => println!("Future is not ready yet."),
            Poll::Ready(result) => {
                println!("Future completed with result: {}", result);
                break;
            }
        }
    }
}

// Implement the Future trait for a struct
struct MyFuture2 {
    state: i32,
    completed: bool,
}

impl Future for MyFuture2 {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready("Computation complete".to_string())
        } else {
            self.get_mut().completed = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Example of using the future
fn main2() {
    let mut my_future = MyFuture2 {
        state: 0,
        completed: false,
    };

    // Create a simple executor to poll the future
    let waker = futures::task::noop_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        match Pin::new(&mut my_future).poll(&mut cx) {
            Poll::Pending => println!("Future is not ready yet."),
            Poll::Ready(result) => {
                println!("Future completed with result: {}", result);
                break;
            }
        }
    }
}
