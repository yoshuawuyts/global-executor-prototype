use std::{fmt::Debug, future::Future};
use std::pin::Pin;

use crate::{Task, RawHandle};
use core::task::{Context, Poll};

/// A handle that awaits the result of a task.
///
/// Dropping a [`JoinHandle`] will detach the task, meaning that there is no longer
/// a handle to the task and no way to `join` on it.
///
/// Created when a task is [spawned].
///
/// [spawned]: fn.spawn.html
pub struct JoinHandle<T: 'static> {
    handle: Option<RawHandle<T>>,
    task: Task,
}

impl<T: 'static> Debug for JoinHandle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JoinHandle").finish()
    }
}

// type InnerHandle<T> = async_global_executor::Task<T>;

impl<T> JoinHandle<T> {
    /// Creates a new `JoinHandle`.
    pub(crate) fn new(inner: RawHandle<T>, task: Task) -> JoinHandle<T> {
        JoinHandle {
            handle: Some(inner),
            task,
        }
    }

    /// Returns a handle to the underlying task.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// #
    /// use async_std::task;
    ///
    /// let handle = task::spawn(async {
    ///     1 + 2
    /// });
    /// println!("id = {}", handle.task().id());
    /// #
    /// # })
    pub fn task(&self) -> &Task {
        &self.task
    }

    // /// Cancel this task.
    // pub async fn cancel(mut self) -> Option<T> {
    //     let handle = self.handle.take().unwrap();
    //     handle.cancel().await
    // }
}

// #[cfg(not(target_os = "unknown"))]
// impl<T> Drop for JoinHandle<T> {
//     fn drop(&mut self) {
//         if let Some(handle) = self.handle.take() {
//             handle.detach();
//         }
//     }
// }

impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.handle.as_mut().unwrap()).poll(cx)
    }
}
