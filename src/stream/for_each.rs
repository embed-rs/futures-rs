use {Future, Task, Poll};
use stream::Stream;

/// A stream combinator which executes a unit closure over each item on a
/// stream.
///
/// This structure is returned by the `Stream::for_each` method.
pub struct ForEach<S, F> {
    stream: S,
    f: F,
}

pub fn new<S, F>(s: S, f: F) -> ForEach<S, F>
    where S: Stream,
          F: FnMut(S::Item) -> () + Send + 'static
{
    ForEach {
        stream: s,
        f: f,
    }
}

impl<S, F> Future for ForEach<S, F>
    where S: Stream,
          F: FnMut(S::Item) + Send + 'static
{
    type Item = ();

    fn poll(&mut self, task: &mut Task) -> Poll<()> {
        loop {
            match try_poll!(self.stream.poll(task)) {
                Some(e) => (self.f)(e),
                None => return Poll::Ok(()),
            }
        }
    }

    fn schedule(&mut self, task: &mut Task) {
        self.stream.schedule(task)
    }
}
