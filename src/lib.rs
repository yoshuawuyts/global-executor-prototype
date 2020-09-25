//! A global futures executor
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

mod task;
mod task_id;
mod join_handle;

pub use task::Task;
pub use task_id::TaskId;
pub use join_handle::JoinHandle;

use std::fmt::Debug;
use std::{future::Future, pin::Pin};

/// A futures executor that can be registered as the standard library’s default
/// through the `#[global_executor]` attribute.
pub trait GlobalExec {
    /// Spawns a task.
    ///
    /// This function is similar to [`std::thread::spawn`], except it spawns an asynchronous task.
    ///
    /// [`std::thread`]: https://doc.rust-lang.org/std/thread/fn.spawn.html
    fn spawn<F, T>(&self, fut: F, policy: TaskPolicy) -> RawTask<T>
    where
        F: Future<Output = T> + Send + Sized + 'static,
        T: Send + 'static,
    {
        let fut = async_std::task::spawn(fut);
        RawTask { fut: Box::pin(fut) }
    }

    /// Spawns a task onto the thread-local executor.
    fn spawn_local<F, T>(&self, _fut: F, _policy: TaskPolicy) -> JoinHandle<T>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        // let fut = async_std::task::spawn_local(fut);
        // RawTask { fut: Box::pin(fut) }
        todo!();
    }

    /// Run blocking code on the executor.
    fn unblock<T, F>(&self, f: F, policy: TaskPolicy) -> RawTask<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(async { f() }, policy)
    }
}

/// Scheduling policy for a task.
#[derive(Debug)]
pub struct TaskPolicy {
    _priv: (),
}

/// An internal struct constructed by executors.
pub struct RawTask<T>
where
    T: Send + 'static,
{
    fut: Pin<Box<dyn Future<Output = T> + Send + 'static>>,
}

impl<T> Debug for RawTask<T>
where
    T: Send + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawTask").finish()
    }
}

impl<T: Send + 'static> Unpin for RawTask<T> {}

impl<T> Future for RawTask<T>
where
    T: Send + 'static,
{
    type Output = T;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.fut.as_mut().poll(cx)
    }
}

fn create_task<E: GlobalExec>(exec: &E) {}
