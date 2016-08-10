use std::mem;

use {Future, empty, Poll, Task};

impl<T> Future for Box<Future<Item=T>>
    where T: Send + 'static,
{
    type Item = T;

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item> {
        (**self).poll(task)
    }

    fn schedule(&mut self, task: &mut Task) {
        (**self).schedule(task)
    }

    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item>>> {
        if let Some(f) = (**self).tailcall() {
            return Some(f)
        }
        Some(mem::replace(self, Box::new(empty())))
    }
}

impl<F: Future> Future for Box<F> {
    type Item = F::Item;

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item> {
        (**self).poll(task)
    }

    fn schedule(&mut self, task: &mut Task) {
        (**self).schedule(task)
    }

    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item>>> {
        (**self).tailcall()
    }
}
