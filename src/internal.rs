use crate::GlobalExec;

/// The goal of this is to provide a `#[global_executor] trait that works the way
/// `#[global_allocator]` does. Essentially that becomes a dyn trait backed by a
/// static. I don't know how to do that haha.
static EXECUTOR: Box<dyn GlobalExec>;

/// Hi
// static EXECUTOR: *const ();

/// The internal
struct InternalExec;

impl InternalExec {
    pub fn set_executor(exec: impl GlobalExec) {
        unsafe {
            *EXECUTOR = &exec as *const ();
        }
    }
    pub fn create_task<E: GlobalExec>(exec: &E) {

    }
}
