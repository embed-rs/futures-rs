use {Task, Poll};
use stream::Stream;
use alloc::boxed::Box;

impl<S: ?Sized + Stream> Stream for Box<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<Self::Item>, Self::Error> {
        (**self).poll(task)
    }

    fn schedule(&mut self, task: &mut Task) {
        (**self).schedule(task)
    }
}
