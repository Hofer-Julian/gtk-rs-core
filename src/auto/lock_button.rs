// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use gio;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct LockButton(Object<ffi::GtkLockButton, ffi::GtkLockButtonClass, LockButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_lock_button_get_type(),
    }
}

impl LockButton {
    pub fn new<P: IsA<gio::Permission>>(permission: Option<&P>) -> LockButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_lock_button_new(permission.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_LOCK_BUTTON: Option<&LockButton> = None;

pub trait LockButtonExt: 'static {
    fn get_permission(&self) -> Option<gio::Permission>;

    fn set_permission<P: IsA<gio::Permission>>(&self, permission: Option<&P>);

    fn get_property_text_lock(&self) -> Option<GString>;

    fn set_property_text_lock(&self, text_lock: Option<&str>);

    fn get_property_text_unlock(&self) -> Option<GString>;

    fn set_property_text_unlock(&self, text_unlock: Option<&str>);

    fn get_property_tooltip_lock(&self) -> Option<GString>;

    fn set_property_tooltip_lock(&self, tooltip_lock: Option<&str>);

    fn get_property_tooltip_not_authorized(&self) -> Option<GString>;

    fn set_property_tooltip_not_authorized(&self, tooltip_not_authorized: Option<&str>);

    fn get_property_tooltip_unlock(&self) -> Option<GString>;

    fn set_property_tooltip_unlock(&self, tooltip_unlock: Option<&str>);

    fn connect_property_permission_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_not_authorized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tooltip_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LockButton>> LockButtonExt for O {
    fn get_permission(&self) -> Option<gio::Permission> {
        unsafe {
            from_glib_none(ffi::gtk_lock_button_get_permission(self.as_ref().to_glib_none().0))
        }
    }

    fn set_permission<P: IsA<gio::Permission>>(&self, permission: Option<&P>) {
        unsafe {
            ffi::gtk_lock_button_set_permission(self.as_ref().to_glib_none().0, permission.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn get_property_text_lock(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"text-lock\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_text_lock(&self, text_lock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"text-lock\0".as_ptr() as *const _, Value::from(text_lock).to_glib_none().0);
        }
    }

    fn get_property_text_unlock(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"text-unlock\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_text_unlock(&self, text_unlock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"text-unlock\0".as_ptr() as *const _, Value::from(text_unlock).to_glib_none().0);
        }
    }

    fn get_property_tooltip_lock(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tooltip-lock\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tooltip_lock(&self, tooltip_lock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tooltip-lock\0".as_ptr() as *const _, Value::from(tooltip_lock).to_glib_none().0);
        }
    }

    fn get_property_tooltip_not_authorized(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tooltip-not-authorized\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tooltip_not_authorized(&self, tooltip_not_authorized: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tooltip-not-authorized\0".as_ptr() as *const _, Value::from(tooltip_not_authorized).to_glib_none().0);
        }
    }

    fn get_property_tooltip_unlock(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tooltip-unlock\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_tooltip_unlock(&self, tooltip_unlock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"tooltip-unlock\0".as_ptr() as *const _, Value::from(tooltip_unlock).to_glib_none().0);
        }
    }

    fn connect_property_permission_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::permission\0".as_ptr() as *const _,
                Some(transmute(notify_permission_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-lock\0".as_ptr() as *const _,
                Some(transmute(notify_text_lock_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-unlock\0".as_ptr() as *const _,
                Some(transmute(notify_text_unlock_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tooltip_lock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-lock\0".as_ptr() as *const _,
                Some(transmute(notify_tooltip_lock_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tooltip_not_authorized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-not-authorized\0".as_ptr() as *const _,
                Some(transmute(notify_tooltip_not_authorized_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tooltip_unlock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip-unlock\0".as_ptr() as *const _,
                Some(transmute(notify_tooltip_unlock_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_permission_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    let f: &F = &*(f as *const F);
    f(&LockButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_lock_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    let f: &F = &*(f as *const F);
    f(&LockButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_unlock_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    let f: &F = &*(f as *const F);
    f(&LockButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tooltip_lock_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    let f: &F = &*(f as *const F);
    f(&LockButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tooltip_not_authorized_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    let f: &F = &*(f as *const F);
    f(&LockButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_tooltip_unlock_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLockButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LockButton> {
    let f: &F = &*(f as *const F);
    f(&LockButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for LockButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LockButton")
    }
}
