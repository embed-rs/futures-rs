use {Task, Poll};
use stream::Stream;

/// A combinator used to filter the results of a stream and simultaneously map
/// them to a different type.
///
/// This structure is returned by the `Stream::filter_map` method.
pub struct FilterMap<S, F> {
    stream: S,
    f: F,
}

pub fn new<S, F, B>(s: S, f: F) -> FilterMap<S, F>
    where S: Stream,
          F: FnMut(S::Item) -> Option<B> + Send + 'static,
{
    FilterMap {
        stream: s,
        f: f,
    }
}

impl<S, F, B> Stream for FilterMap<S, F>
    where S: Stream,
          F: FnMut(S::Item) -> Option<B> + Send + 'static,
          B: Send + 'static,
{
    type Item = B;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<B>> {
        loop {
            match try_poll!(self.stream.poll(task)) {
                Some(e) => {
                    if let Some(e) = (self.f)(e) {
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
