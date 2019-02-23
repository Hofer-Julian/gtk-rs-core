// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use gdk;
use gdk_ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ColorChooser(Interface<ffi::GtkColorChooser>);

    match fn {
        get_type => || ffi::gtk_color_chooser_get_type(),
    }
}

pub const NONE_COLOR_CHOOSER: Option<&ColorChooser> = None;

pub trait ColorChooserExt: 'static {
    fn get_rgba(&self) -> gdk::RGBA;

    fn get_use_alpha(&self) -> bool;

    fn set_rgba(&self, color: &gdk::RGBA);

    fn set_use_alpha(&self, use_alpha: bool);

    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorChooser>> ColorChooserExt for O {
    fn get_rgba(&self) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_color_chooser_get_rgba(self.as_ref().to_glib_none().0, color.to_glib_none_mut().0);
            color
        }
    }

    fn get_use_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_color_chooser_get_use_alpha(self.as_ref().to_glib_none().0))
        }
    }

    fn set_rgba(&self, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_color_chooser_set_rgba(self.as_ref().to_glib_none().0, color.to_glib_none().0);
        }
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_chooser_set_use_alpha(self.as_ref().to_glib_none().0, use_alpha.to_glib());
        }
    }

    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"color-activated\0".as_ptr() as *const _,
                Some(transmute(color_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rgba\0".as_ptr() as *const _,
                Some(transmute(notify_rgba_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-alpha\0".as_ptr() as *const _,
                Some(transmute(notify_use_alpha_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn color_activated_trampoline<P, F: Fn(&P, &gdk::RGBA) + 'static>(this: *mut ffi::GtkColorChooser, color: *mut gdk_ffi::GdkRGBA, f: glib_ffi::gpointer)
where P: IsA<ColorChooser> {
    let f: &F = &*(f as *const F);
    f(&ColorChooser::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(color))
}

unsafe extern "C" fn notify_rgba_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkColorChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooser> {
    let f: &F = &*(f as *const F);
    f(&ColorChooser::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_alpha_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkColorChooser, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ColorChooser> {
    let f: &F = &*(f as *const F);
    f(&ColorChooser::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ColorChooser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorChooser")
    }
}
