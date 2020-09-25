use std::fmt::Debug;
use std::future::Future;
use std::{fmt, pin::Pin};

/// An internal struct constructed by executors.
pub struct RawHandle<T>
where
    T: 'static,
{
    pub(crate) fut: Pin<Box<dyn Future<Output = T> + 'static>>,
}

impl<T> Debug for RawHandle<T>
where
    T: Send + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawTask").finish()
    }
}

impl<T: 'static> Unpin for RawHandle<T> {}

unsafe impl<T: Send> Send for RawHandle<T> {}

impl<T> Future for RawHandle<T>
where
    T: 'static,
{
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.fut.as_mut().poll(cx)
    }
}
