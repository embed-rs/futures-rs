use {Task, Poll};
use stream::{Stream, Fuse};

/// An adapter for merging the output of two streams.
///
/// The merged stream produces items from one or both of the underlying
/// streams as they become available. Errors, however, are not merged: you
/// get at most one error at a time.
pub struct Merge<S1, S2: Stream> {
    stream1: Fuse<S1>,
    stream2: Fuse<S2>,
}

pub fn new<S1, S2>(stream1: S1, stream2: S2) -> Merge<S1, S2>
    where S1: Stream, S2: Stream
{
    Merge {
        stream1: stream1.fuse(),
        stream2: stream2.fuse(),
    }
}

/// An item returned from a merge stream, which represents an item from one or
/// both of the underlying streams.
pub enum MergedItem<I1, I2> {
    /// An item from the first stream
    First(I1),
    /// An item from the second stream
    Second(I2),
    /// Items from both streams
    Both(I1, I2),
}

impl<S1, S2> Stream for Merge<S1, S2>
    where S1: Stream, S2: Stream
{
    type Item = MergedItem<S1::Item, S2::Item>;

    fn poll(&mut self, task: &mut Task) -> Poll<Option<Self::Item>> {
        match self.stream1.poll(task) {
            Poll::NotReady => match self.stream2.poll(task) {
                Poll::NotReady => Poll::NotReady,
                Poll::Ok(Some(item2)) => Poll::Ok(Some(MergedItem::Second(item2))),
                Poll::Ok(None) => Poll::NotReady,
            },
            Poll::Ok(Some(item1)) => match self.stream2.poll(task) {
                Poll::NotReady => Poll::Ok(Some(MergedItem::First(item1))),
                Poll::Ok(Some(item2)) => Poll::Ok(Some(MergedItem::Both(item1, item2))),
                Poll::Ok(None) => Poll::Ok(Some(MergedItem::First(item1))),
            },
            Poll::Ok(None) => match self.stream2.poll(task) {
                Poll::NotReady => Poll::NotReady,
                Poll::Ok(Some(item2)) => Poll::Ok(Some(MergedItem::Second(item2))),
                Poll::Ok(None) => Poll::Ok(None),
            },
        }
    }

    fn schedule(&mut self, task: &mut Task) {
        self.stream1.schedule(task);
        self.stream2.schedule(task);
    }
}
