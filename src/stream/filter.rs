use {Task, Poll};
use stream::Stream;

/// A stream combinator used to filter the results of a stream and only yield
/// some values.
///
/// This structure is produced by the `Stream::filter` method.
pub struct Filter<S, F> {
    stream: S,
    f: F,
}

pub fn new<S, F>(s: S, f: F) -> Filter<S, F>
    where S: Stream,
          F: FnMut(&S::Item) -> bool + Send + 'static,
{
    Filter {
        stream: s,
        f: f,
    }
}

impl<S, F> Stream for Filter<S, F>
    where S: Stream,
          F: FnMut(&S::Item) -> bool + Send + 'static,
{
    type Item = S::Item;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<S::Item>> {
        loop {
            match try_poll!(self.stream.poll(task)) {
                Some(e) => {
                    if (self.f)(&e) {
                        return Poll::Ok(Some(e))
                    }
                }
                None => return Poll::Ok(None),
            }
        }
    }

    fn schedule(&mut self, task: &mut Task) {
        self.stream.schedule(task)
    }
}
