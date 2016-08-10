use std::mem;

use {Future, Task, empty, Poll};
use util::Collapsed;

/// Future for the `select` combinator, waiting for one of two futures to
/// complete.
///
/// This is created by this `Future::select` method.
pub struct Select<A, B> where A: Future, B: Future<Item=A::Item> {
    inner: Option<(Collapsed<A>, Collapsed<B>)>,
}

/// Future yielded as the second result in a `Select` future.
///
/// This sentinel future represents the completion of the second future to a
/// `select` which finished second.
pub struct SelectNext<A, B> where A: Future, B: Future<Item=A::Item> {
    inner: OneOf<A, B>,
}

enum OneOf<A, B> where A: Future, B: Future {
    A(Collapsed<A>),
    B(Collapsed<B>),
}

pub fn new<A, B>(a: A, b: B) -> Select<A, B>
    where A: Future,
          B: Future<Item=A::Item>
{
    let a = Collapsed::Start(a);
    let b = Collapsed::Start(b);
    Select {
        inner: Some((a, b)),
    }
}

impl<A, B> Future for Select<A, B>
    where A: Future,
          B: Future<Item=A::Item>,
{
    type Item = (A::Item, SelectNext<A, B>);

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item> {
        let (ret, is_a) = match self.inner {
            Some((ref mut a, ref mut b)) => {
                match a.poll(task) {
                    Poll::Ok(a) => (a, true),
                    Poll::NotReady => (try_poll!(b.poll(task)), false),
                }
            }
            None => panic!("cannot poll select twice"),
        };

        let (a, b) = self.inner.take().unwrap();
        let next = if is_a {OneOf::B(b)} else {OneOf::A(a)};
        let next = SelectNext { inner: next };
        Poll::Ok((ret, next))
    }

    fn schedule(&mut self, task: &mut Task) {
        match self.inner {
            Some((ref mut a, ref mut b)) => {
                a.schedule(task);
                b.schedule(task);
            }
            None => task.notify(),
        }
    }

    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item>>> {
        if let Some((ref mut a, ref mut b)) = self.inner {
            a.collapse();
            b.collapse();
        }
        None
    }
}

impl<A, B> Future for SelectNext<A, B>
    where A: Future,
          B: Future<Item=A::Item>,
{
    type Item = A::Item;

    fn poll(&mut self, task: &mut Task) -> Poll<Self::Item> {
        match self.inner {
            OneOf::A(ref mut a) => a.poll(task),
            OneOf::B(ref mut b) => b.poll(task),
        }
    }

    fn schedule(&mut self, task: &mut Task) {
        match self.inner {
            OneOf::A(ref mut a) => a.schedule(task),
            OneOf::B(ref mut b) => b.schedule(task),
        }
    }

    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item>>> {
        match self.inner {
            OneOf::A(ref mut a) => a.collapse(),
            OneOf::B(ref mut b) => b.collapse(),
        }
        match self.inner {
            OneOf::A(Collapsed::Tail(ref mut a)) |
            OneOf::B(Collapsed::Tail(ref mut a)) => {
                Some(mem::replace(a, Box::new(empty())))
            }
            _ => None,
        }
    }
}
