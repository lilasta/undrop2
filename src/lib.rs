#![no_std]

use core::ops::{Deref, DerefMut};

const MAGIC_CHAR: char = '☔';

#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Undroppable<T: ?Sized, const S: char>(T);

impl<T, const S: char> Undroppable<T, { S }> {
    /// Wrap a value in `Undroppable`.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T: ?Sized, const S: char> Undroppable<T, { S }> {
    const CHECK: () = assert!(S == MAGIC_CHAR, "Undroppable!");
}

impl<T> Undroppable<T, MAGIC_CHAR> {
    /// Drop the inner value explicitly.
    #[inline(always)]
    pub fn drop(mut this: Self) {
        let inner_ptr = core::ptr::addr_of_mut!(this.0);
        core::mem::forget(this);
        unsafe { core::ptr::drop_in_place(inner_ptr) };
    }

    /// Forget the inner value explicitly.
    /// This is equivalent to calling `core::mem::forget`.
    #[inline(always)]
    pub const fn forget(this: Self) {
        core::mem::forget(this);
    }

    /// Extracts the inner value from the `Undroppable` container.
    #[inline(always)]
    pub fn into_inner(this: Self) -> T {
        let inner_ptr = core::ptr::addr_of!(this.0);
        Self::forget(this);
        unsafe { core::ptr::read(inner_ptr) }
    }
}

impl<T: ?Sized, const S: char> Deref for Undroppable<T, { S }> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        let _ = Self::CHECK;
        &self.0
    }
}

impl<T: ?Sized, const S: char> DerefMut for Undroppable<T, { S }> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ = Self::CHECK;
        &mut self.0
    }
}

impl<T: ?Sized, const S: char> Drop for Undroppable<T, { S }> {
    #[inline(always)]
    fn drop(&mut self) {
        let _ = Self::CHECK;
    }
}
