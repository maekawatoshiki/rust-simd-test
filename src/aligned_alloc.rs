use core::{cmp, mem};
use log;
use std::alloc::{self, handle_alloc_error, Layout};
use std::ops::{Deref, DerefMut};
use std::slice;

pub(crate) struct Memory<T> {
    ptr: *mut T,
    len: usize,
    align: usize,
}

impl<T> Memory<T> {
    #[inline]
    pub unsafe fn new(nelem: usize, align: usize) -> Self {
        let align = cmp::max(align, mem::align_of::<T>());
        #[cfg(debug_assertions)]
        let layout = Layout::from_size_align(mem::size_of::<T>() * nelem, align).unwrap();
        #[cfg(not(debug_assertions))]
        let layout = Layout::from_size_align_unchecked(mem::size_of::<T>() * nelem, align);
        log::debug!("Allocating nelem={}, layout={:?}", nelem, layout);
        let ptr = alloc::alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        assert!(ptr as usize % align == 0);
        Memory {
            ptr: ptr as *mut T,
            len: nelem,
            align,
        }
    }

    pub fn filled_with(mut self, x: T) -> Memory<T>
    where
        T: Copy,
    {
        for elem in &mut self[..] {
            *elem = x;
        }
        self
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }
}

impl<T> Drop for Memory<T> {
    fn drop(&mut self) {
        unsafe {
            let layout =
                Layout::from_size_align_unchecked(mem::size_of::<T>() * self.len, self.align);
            alloc::dealloc(self.ptr as _, layout);
        }
    }
}

impl<T> Deref for Memory<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for Memory<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}
