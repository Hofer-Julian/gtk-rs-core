// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use Widget;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureZoom(Object<ffi::GtkGestureZoom, ffi::GtkGestureZoomClass, GestureZoomClass>) @extends Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_zoom_get_type(),
    }
}

impl GestureZoom {
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureZoom {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_zoom_new(widget.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_GESTURE_ZOOM: Option<&GestureZoom> = None;

pub trait GestureZoomExt: 'static {
    fn get_scale_delta(&self) -> f64;

    fn connect_scale_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureZoom>> GestureZoomExt for O {
    fn get_scale_delta(&self) -> f64 {
        unsafe {
            ffi::gtk_gesture_zoom_get_scale_delta(self.as_ref().to_glib_none().0)
        }
    }

    fn connect_scale_changed<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"scale-changed\0".as_ptr() as *const _,
                Some(transmute(scale_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn scale_changed_trampoline<P, F: Fn(&P, f64) + 'static>(this: *mut ffi::GtkGestureZoom, scale: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureZoom> {
    let f: &F = &*(f as *const F);
    f(&GestureZoom::from_glib_borrow(this).unsafe_cast(), scale)
}

impl fmt::Display for GestureZoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureZoom")
    }
}
