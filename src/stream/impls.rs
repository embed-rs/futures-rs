use {Task, Poll};
use stream::Stream;

impl<S: ?Sized + Stream> Stream for Box<S> {
    type Item = S::Item;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<Self::Item>> {
        (**self).poll(task)
    }

    fn schedule(&mut self, task: &mut Task) {
        (**self).schedule(task)
    }
}
