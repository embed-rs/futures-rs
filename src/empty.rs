use std::marker;

use {Future, Task, Poll};

/// A future which is never resolved.
///
/// This future can be created with the `empty` function.
pub struct Empty<T>
    where T: Send + 'static,
{
    _data: marker::PhantomData<T>,
}

/// Creates a future which never resolves, representing a computation that never
/// finishes.
///
/// The returned future will never resolve with a success but is still
/// susceptible to cancellation. That is, if a callback is scheduled on the
/// returned future, it is only run once the future is dropped (canceled).
pub fn empty<T: Send + 'static>() -> Empty<T> {
    Empty {_data: marker::PhantomData }
}

impl<T> Future for Empty<T>
    where T: Send + 'static,
{
    type Item = T;

    fn poll(&mut self, _: &mut Task) -> Poll<T> {
        Poll::NotReady
    }

    fn schedule(&mut self, task: &mut Task) {
        drop(task);
    }
}
