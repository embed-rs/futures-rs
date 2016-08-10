use {Task, Poll};
use stream::Stream;

/// A stream which is just a shim over an underlying instance of `Iterator`.
///
/// This stream will never block and is always ready.
pub struct IterStream<I> {
    iter: I,
}

/// Converts an `Iterator` into a `Stream` which is always ready to yield the
/// next value.
///
/// Iterators in Rust don't express the ability to block, so this adapter simply
/// always calls `iter.next()` and returns that. Additionally, the error type is
/// generic here as it will never be returned, instead the type of the iterator
/// will always be returned upwards as a successful value.
pub fn iter<I, T, E>(i: I) -> IterStream<I>
    where I: Iterator<Item=Result<T, E>>,
          I: Send + 'static,
          T: Send + 'static,
          E: Send + 'static,
{
    IterStream {
        iter: i,
    }
}

impl<I, T> Stream for IterStream<I>
    where I: Iterator<Item=T>,
          I: Send + 'static,
          T: Send + 'static,
{
    type Item = T;

    fn poll(&mut self, _task: &mut Task) -> Poll<Option<T>> {
        Poll::Ok(self.iter.next())
    }

    fn schedule(&mut self, task: &mut Task) {
        task.notify()
    }
}
