///! A OnceCell that implements Deref, panicking if not set yet.
use std::ops::Deref;

use once_cell::sync::OnceCell;

/// A OnceCell that implements Deref, panicking if not set yet.
pub struct AlwaysCell<T> {
    inner: OnceCell<T>,
}

impl<T> AlwaysCell<T> {
    pub const fn new() -> Self {
        Self {
            inner: OnceCell::new(),
        }
    }

    /// Sets the contents of this cell to value.
    /// Panics if this cell was already set.
    pub fn set(self_: &Self, value: T) {
        self_.inner
            .set(value)
            .ok()
            .expect("called AlwaysCell::set more than once");
    }

    /// Sets the contents of this cell to value.
    /// Returns Ok(()) if the cell was empty and Err(value) if it was full.
    pub fn try_set(self_: &Self, value: T) -> Result<(), T> {
        self_.inner.set(value)
    }

    /// Returns true if this cell is already set
    pub fn is_set(self_: &Self) -> bool {
        self_.inner.get().is_some()
    }

    /// Returns Some(T) if set, otherwise None
    pub fn get(self_: &Self) -> Option<&T> {
        self_.inner.get()
    }
}

impl<T> Deref for AlwaysCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
            .get()
            .expect("AlwaysCell::deref called before `set`")
    }
}
