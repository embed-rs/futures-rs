
#[macro_export]
macro_rules! try_poll {
    ($e:expr) => (match $e {
        $crate::Poll::NotReady => return $crate::Poll::NotReady,
        $crate::Poll::Ok(t) => t,
    })
}

/// Possible return values from the `Future::poll` method.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Poll<T> {
    /// Indicates that the future is not ready yet, ask again later.
    NotReady,

    /// Indicates that the future has completed successfully, and this value is
    /// what the future completed with.
    Ok(T),
}

impl<T> Poll<T> {
    /// Change the success type of this `Poll` value with the closure provided
    pub fn map<F, U>(self, f: F) -> Poll<U>
        where F: FnOnce(T) -> U
    {
        match self {
            Poll::NotReady => Poll::NotReady,
            Poll::Ok(t) => Poll::Ok(f(t)),
        }
    }

    /// Returns whether this is `Poll::NotReady`
    pub fn is_not_ready(&self) -> bool {
        match *self {
            Poll::NotReady => true,
            _ => false,
        }
    }

    /// Returns whether this is either `Poll::Ok` or `Poll::Err`
    pub fn is_ready(&self) -> bool {
        !self.is_not_ready()
    }

    /// Unwraps this `Poll`, panicking if it's not ready.
    pub fn unwrap(self) -> T {
        match self {
            Poll::Ok(t) => t,
            Poll::NotReady => panic!("unwrapping a Poll that wasn't ready"),
        }
    }
}

impl<T> From<T> for Poll<T> {
    fn from(r: T) -> Poll<T> {
        Poll::Ok(r)
    }
}
