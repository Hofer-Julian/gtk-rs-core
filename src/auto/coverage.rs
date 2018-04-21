// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CoverageLevel;
use ffi;
use glib::translate::*;
use glib_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Coverage(Shared<ffi::PangoCoverage>);

    match fn {
        ref => |ptr| ffi::pango_coverage_ref(ptr),
        unref => |ptr| ffi::pango_coverage_unref(ptr),
    }
}

impl Coverage {
    pub fn copy(&self) -> Option<Coverage> {
        unsafe {
            from_glib_full(ffi::pango_coverage_copy(self.to_glib_none().0))
        }
    }

    pub fn get(&self, index_: i32) -> CoverageLevel {
        unsafe {
            from_glib(ffi::pango_coverage_get(self.to_glib_none().0, index_))
        }
    }

    pub fn max(&self, other: &Coverage) {
        unsafe {
            ffi::pango_coverage_max(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    pub fn set(&self, index_: i32, level: CoverageLevel) {
        unsafe {
            ffi::pango_coverage_set(self.to_glib_none().0, index_, level.to_glib());
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        unsafe {
            let mut bytes = ptr::null_mut();
            let mut n_bytes = mem::uninitialized();
            ffi::pango_coverage_to_bytes(self.to_glib_none().0, &mut bytes, &mut n_bytes);
            FromGlibContainer::from_glib_full_num(bytes, n_bytes as usize)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Coverage> {
        let n_bytes = bytes.len() as i32;
        unsafe {
            from_glib_full(ffi::pango_coverage_from_bytes(bytes.to_glib_none().0, n_bytes))
        }
    }

    pub fn new() -> Coverage {
        unsafe {
            from_glib_none(ffi::pango_coverage_new())
        }
    }
}

impl Default for Coverage {
    fn default() -> Self {
        Self::new()
    }
}
