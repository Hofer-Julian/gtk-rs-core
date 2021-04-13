// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Widget;
use crate::Window;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct WindowGroup(Object<ffi::GtkWindowGroup, ffi::GtkWindowGroupClass>);

    match fn {
        get_type => || ffi::gtk_window_group_get_type(),
    }
}

impl WindowGroup {
    #[doc(alias = "gtk_window_group_new")]
    pub fn new() -> WindowGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_window_group_new()) }
    }
}

impl Default for WindowGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_WINDOW_GROUP: Option<&WindowGroup> = None;

pub trait WindowGroupExt: 'static {
    #[doc(alias = "gtk_window_group_add_window")]
    fn add_window<P: IsA<Window>>(&self, window: &P);

    #[doc(alias = "gtk_window_group_get_current_device_grab")]
    fn get_current_device_grab(&self, device: &gdk::Device) -> Option<Widget>;

    #[doc(alias = "gtk_window_group_get_current_grab")]
    fn current_grab(&self) -> Option<Widget>;

    #[doc(alias = "gtk_window_group_list_windows")]
    fn list_windows(&self) -> Vec<Window>;

    #[doc(alias = "gtk_window_group_remove_window")]
    fn remove_window<P: IsA<Window>>(&self, window: &P);
}

impl<O: IsA<WindowGroup>> WindowGroupExt for O {
    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_window_group_add_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_current_device_grab(&self, device: &gdk::Device) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_device_grab(
                self.as_ref().to_glib_none().0,
                device.to_glib_none().0,
            ))
        }
    }

    fn current_grab(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_window_group_get_current_grab(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_window_group_list_windows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_window_group_remove_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for WindowGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowGroup")
    }
}
