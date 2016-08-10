use {Future, Task, Poll};
use util::Collapsed;

/// Future for the `map` combinator, changing the type of a future.
///
/// This is created by this `Future::map` method.
pub struct Map<A, F> where A: Future {
    future: Collapsed<A>,
    f: Option<F>,
}

pub fn new<A, F>(future: A, f: F) -> Map<A, F>
    where A: Future,
{
    Map {
        future: Collapsed::Start(future),
        f: Some(f),
    }
}

impl<U, A, F> Future for Map<A, F>
    where A: Future,
          F: FnOnce(A::Item) -> U + Send + 'static,
          U: Send + 'static,
{
    type Item = U;

    fn poll(&mut self, task: &mut Task) -> Poll<U> {
        let result = try_poll!(self.future.poll(task));
        self.f.take().expect("cannot poll Map twice")(result).into()
    }

    fn schedule(&mut self, task: &mut Task) {
        self.future.schedule(task)
    }

    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item>>> {
        self.future.collapse();
        None
    }
}
