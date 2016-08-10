use {Task, Future, Poll};
use stream::Stream;

/// A combinator used to temporarily convert a stream into a future.
///
/// This future is returned by the `Stream::into_future` method.
pub struct StreamFuture<S> {
    stream: Option<S>,
}

pub fn new<S: Stream>(s: S) -> StreamFuture<S> {
    StreamFuture { stream: Some(s) }
}

impl<S: Stream> Future for StreamFuture<S> {
    type Item = (Option<S::Item>, S);

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item> {
        let item = {
            let s = self.stream.as_mut().expect("polling StreamFuture twice");
            try_poll!(s.poll(task))
        };
        let stream = self.stream.take().unwrap();

        Poll::Ok((item, stream))
    }

    fn schedule(&mut self, task: &mut Task) {
        if let Some(s) = self.stream.as_mut() {
            s.schedule(task)
        }
    }
}
