use {Future, Poll, Task};

pub fn forget<T: Future>(t: T) {
    let thunk = ThunkFuture { inner: t.boxed() }.boxed();
    Task::new().run(thunk)
}

// FIXME(rust-lang/rust#34416) should just be able to use map/map_err, but that
//                             causes trans to go haywire.
struct ThunkFuture<T> {
    inner: Box<Future<Item=T>>,
}

impl<T: Send + 'static> Future for ThunkFuture<T> {
    type Item = ();

    fn poll(&mut self, task: &mut Task) -> Poll<()> {
        self.inner.poll(task).map(|_| ())
    }

    fn schedule(&mut self, task: &mut Task) {
        self.inner.schedule(task)
    }

    fn tailcall(&mut self) -> Option<Box<Future<Item=()>>> {
        if let Some(f) = self.inner.tailcall() {
            self.inner = f;
        }
        None
    }
}
