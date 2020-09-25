//! A global futures executor
//!
//! # Examples
//!
//! ```
//! // tbi
//! ```

#![forbid(future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

mod join_handle;
mod raw_handle;
mod task;
mod task_id;
mod task_policy;

pub use join_handle::JoinHandle;
pub use raw_handle::RawHandle;
pub use task::Task;
pub use task_id::TaskId;
pub use task_policy::TaskPolicy;

use std::future::Future;

/// A futures executor that can be registered as the standard library’s default
/// through the `#[global_executor]` attribute.
pub trait GlobalExec {
    /// Spawns a task.
    ///
    /// This function is similar to [`std::thread::spawn`], except it spawns an asynchronous task.
    ///
    /// [`std::thread`]: https://doc.rust-lang.org/std/thread/fn.spawn.html
    fn spawn<F, T>(&self, fut: F, policy: TaskPolicy) -> RawHandle<T>
    where
        F: Future<Output = T> + Send + Sized + 'static,
        T: Send + 'static,
    {
        let fut = async_std::task::spawn(fut);
        RawHandle { fut: Box::pin(fut) }
    }

    /// Spawns a task onto the thread-local executor.
    fn spawn_local<F, T>(&self, _fut: F, _policy: TaskPolicy) -> JoinHandle<T>
    where
        F: Future<Output = T> + 'static,
        T: 'static;
        // let fut = async_std::task::spawn_local(fut);
        // RawHandle { fut: Box::pin(fut) }

    /// Run blocking code on the executor.
    fn unblock<T, F>(&self, f: F, policy: TaskPolicy) -> RawHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.spawn(async { f() }, policy)
    }
}

fn create_task<E: GlobalExec>(exec: &E) {}
