use {Task, IntoFuture, Future, Poll};
use stream::Stream;

/// A stream combinator which chains a computation onto each item produced by a
/// stream.
///
/// This structure is produced by the `Stream::then` method.
pub struct Then<S, F, U>
    where U: IntoFuture,
{
    stream: S,
    future: Option<U::Future>,
    f: F,
}

pub fn new<S, F, U>(s: S, f: F) -> Then<S, F, U>
    where S: Stream,
          F: FnMut(S::Item) -> U + Send + 'static,
          U: IntoFuture,
{
    Then {
        stream: s,
        future: None,
        f: f,
    }
}

impl<S, F, U> Stream for Then<S, F, U>
    where S: Stream,
          F: FnMut(S::Item) -> U + Send + 'static,
          U: IntoFuture,
{
    type Item = U::Item;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<U::Item>> {
        if self.future.is_none() {
            let item = match try_poll!(self.stream.poll(task)) {
                None => return Poll::Ok(None),
                Some(e) => e,
            };
            self.future = Some((self.f)(item).into_future());
        }
        assert!(self.future.is_some());
        let res = self.future.as_mut().unwrap().poll(task);
        if res.is_ready() {
            self.future = None;
        }
        res.map(Some)
    }

    fn schedule(&mut self, task: &mut Task) {
        match self.future {
            Some(ref mut s) => s.schedule(task),
            None => self.stream.schedule(task),
        }
    }
}
