// //! This module contains Rust definitions whose C equivalents live in
// //! lisp.h.

use std::ops::{Deref, DerefMut};

use libc::{c_void, intptr_t, ptrdiff_t};

pub use crate::remacs_sys::*;
use crate::{
    remacs_sys::EmacsInt,
    remacs_sys::{make_string, Qnil, Qt, VALMASK},
};
// TODO: tweak Makefile to rebuild C files if this changes.

pub const INTMASK: EmacsInt = EMACS_INT_MAX >> (INTTYPEBITS - 1);
pub const PSEUDOVECTOR_FLAG: usize = 0x4000_0000_0000_0000;

/// Emacs values are represented as tagged pointers. A few bits are
/// used to represent the type, and the remaining bits are either used
/// to store the value directly (e.g. integers) or the address of a
/// more complex data type (e.g. a cons cell).
///
/// TODO: example representations
///
/// `EmacsInt` represents an integer big enough to hold our tagged
/// pointer representation.
///
/// In Emacs C, this is `EMACS_INT`.
///
/// `EmacsUint` represents the unsigned equivalent of `EmacsInt`.
/// In Emacs C, this is `EMACS_UINT`.
///
/// Their definition are determined in a way consistent with Emacs C.
/// Under casual systems, they're the type isize and usize respectively.
#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(pub EmacsInt);

impl LispObject {
    pub const fn from_C(n: EmacsInt) -> Self {
        Self(n)
    }

    pub const fn to_C(self) -> EmacsInt {
        self.0
    }
}

impl From<()> for LispObject {
    fn from(_v: ()) -> Self {
        Qnil
    }
}

impl From<LispObject> for bool {
    fn from(o: LispObject) -> Self {
        o.is_not_nil()
    }
}

impl From<bool> for LispObject {
    fn from(v: bool) -> Self {
        if v {
            Qt
        } else {
            Qnil
        }
    }
}

impl LispObject {
    pub fn is_nil(self) -> bool {
        self == Qnil
    }

    pub fn is_not_nil(self) -> bool {
        self != Qnil
    }

    pub fn is_t(self) -> bool {
        self == Qt
    }

    pub fn eq(self, other: impl Into<Self>) -> bool {
        self == other.into()
    }
}

impl LispObject {
    pub fn get_untaggedptr(self) -> *mut c_void {
        (self.to_C() & VALMASK) as intptr_t as *mut c_void
    }
}

// ExternalPtr

#[repr(transparent)]
pub struct ExternalPtr<T>(*mut T);

impl<T> Copy for ExternalPtr<T> {}

// Derive fails for this type so do it manually
impl<T> Clone for ExternalPtr<T> {
    fn clone(&self) -> Self {
        Self::new(self.0)
    }
}

impl<T> ExternalPtr<T> {
    pub const fn new(p: *mut T) -> Self {
        Self(p)
    }

    pub fn is_null(self) -> bool {
        self.0.is_null()
    }

    pub const fn as_ptr(self) -> *const T {
        self.0
    }

    pub fn as_mut(&mut self) -> *mut T {
        self.0
    }
}

impl<T> Deref for ExternalPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for ExternalPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

// pub import remacs_sys::*;
// export remacs_sys::*;

#[no_mangle]
pub extern "C" fn myrustfunc(obj: LispObject) -> LispObject {
    let s = "foo".as_ptr() as *mut libc::c_char;
    unsafe { make_string(s, libc::strlen(s) as ptrdiff_t) }
}
