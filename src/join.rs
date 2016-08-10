#![allow(non_snake_case)]

use std::mem;

use {Future, Task, Poll};
use util::Collapsed;

macro_rules! generate {
    ($(($Join:ident, $new:ident, <A, $($B:ident),*>),)*) => ($(
        /// Future for the `join` combinator, waiting for two futures to
        /// complete.
        ///
        /// This is created by this `Future::join` method.
        pub struct $Join<A, $($B),*>
            where A: Future,
                  $($B: Future),*
        {
            a: MaybeDone<A>,
            $($B: MaybeDone<$B>,)*
        }

        pub fn $new<A, $($B),*>(a: A, $($B: $B),*) -> $Join<A, $($B),*>
            where A: Future,
                  $($B: Future),*
        {
            let a = Collapsed::Start(a);
            $(let $B = Collapsed::Start($B);)*
            $Join {
                a: MaybeDone::NotYet(a),
                $($B: MaybeDone::NotYet($B)),*
            }
        }

        impl<A, $($B),*> $Join<A, $($B),*>
            where A: Future,
                  $($B: Future),*
        {
            fn erase(&mut self) {
                self.a = MaybeDone::Gone;
                $(self.$B = MaybeDone::Gone;)*
            }
        }

        impl<A, $($B),*> Future for $Join<A, $($B),*>
            where A: Future,
                  $($B: Future),*
        {
            type Item = (A::Item, $($B::Item),*);

            fn poll(&mut self, task: &mut Task) -> Poll<Self::Item> {
                let mut all_done = self.a.poll(task);
                $(
                    all_done = all_done && self.$B.poll(task);
                )*

                if all_done {
                    Poll::Ok((self.a.take(), $(self.$B.take()),*))
                } else {
                    Poll::NotReady
                }
            }

            fn schedule(&mut self, task: &mut Task) {
                if let MaybeDone::NotYet(ref mut a) = self.a {
                    a.schedule(task);
                }
                $(
                    if let MaybeDone::NotYet(ref mut a) = self.$B {
                        a.schedule(task);
                    }
                )*
            }

            fn tailcall(&mut self)
                        -> Option<Box<Future<Item=Self::Item>>> {
                self.a.collapse();
                $(self.$B.collapse();)*
                None
            }
        }
    )*)
}

generate! {
    (Join, new, <A, B>),
    (Join3, new3, <A, B, C>),
    (Join4, new4, <A, B, C, D>),
    (Join5, new5, <A, B, C, D, E>),
}

enum MaybeDone<A: Future> {
    NotYet(Collapsed<A>),
    Done(A::Item),
    Gone,
}

impl<A: Future> MaybeDone<A> {
    fn poll(&mut self, task: &mut Task) -> bool {
        let res = match *self {
            MaybeDone::NotYet(ref mut a) => a.poll(task),
            MaybeDone::Done(_) => return true,
            MaybeDone::Gone => panic!("cannot poll Join twice"),
        };
        match res {
            Poll::Ok(res) => {
                *self = MaybeDone::Done(res);
                true
            }
            Poll::NotReady => false,
        }
    }

    fn take(&mut self) -> A::Item {
        match mem::replace(self, MaybeDone::Gone) {
            MaybeDone::Done(a) => a,
            _ => panic!(),
        }
    }

    fn collapse(&mut self) {
        match *self {
            MaybeDone::NotYet(ref mut a) => a.collapse(),
            _ => {}
        }
    }
}
