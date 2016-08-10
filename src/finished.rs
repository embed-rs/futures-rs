use std::marker;

use {Future, Task, Poll};

/// A future representing a finished successful computation.
///
/// Created by the `finished` function.
pub struct Finished<T> {
    t: Option<T>,
}

/// Creates a "leaf future" from an immediate value of a finished and
/// successful computation.
///
/// The returned future is similar to `done` where it will immediately run a
/// scheduled callback with the provided value.
///
/// # Examples
///
/// ```
/// use futures::*;
///
/// let future_of_1 = finished::<u32, u32>(1);
/// ```
pub fn finished<T>(t: T) -> Finished<T>
    where T: Send + 'static,
{
    Finished { t: Some(t) }
}

impl<T> Future for Finished<T>
    where T: Send + 'static,
{
    type Item = T;


    fn poll(&mut self, _: &mut Task) -> Poll<T> {
        Poll::Ok(self.t.take().expect("cannot poll Finished twice"))
    }

    fn schedule(&mut self, task: &mut Task) {
        task.notify();
    }
}
