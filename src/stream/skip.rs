use {Task, Poll};
use stream::Stream;

/// A stream combinator which skips a number of elements before continuing.
///
/// This structure is produced by the `Stream::skip` method.
pub struct Skip<S> {
    stream: S,
    remaining: u64,
}

pub fn new<S>(s: S, amt: u64) -> Skip<S>
    where S: Stream,
{
    Skip {
        stream: s,
        remaining: amt,
    }
}

impl<S> Stream for Skip<S>
    where S: Stream,
{
    type Item = S::Item;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<S::Item>> {
        while self.remaining > 0 {
            match try_poll!(self.stream.poll(task)) {
                Some(_) => self.remaining -= 1,
                None => return Poll::Ok(None),
            }
        }

        self.stream.poll(task)
    }

    fn schedule(&mut self, task: &mut Task) {
        self.stream.schedule(task)
    }
}
