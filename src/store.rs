use std::marker;
use std::any::Any;

use {Task, TaskData, Poll, Future};

/// A combinator which will store some data into task-local storage.
///
/// This combinator is created by the `futures::store` method.
pub struct Store<T: Send + 'static> {
    item: Option<T>,
}

/// A combinator to store some data into task-local storage.
pub fn store<T, E>(t: T) -> Store<T>
    where T: Any + Send + 'static,
{
    Store { item: Some(t) }
}

impl<T> Future for Store<T>
    where T: Any + Send + 'static,
{
    type Item = TaskData<T>;

    fn poll(&mut self, task: &mut Task) -> Poll<TaskData<T>> {
        let item = self.item.take().expect("cannot poll Store twice");
        Poll::Ok(task.insert(item))
    }

    fn schedule(&mut self, task: &mut Task) {
        task.notify()
    }
}
