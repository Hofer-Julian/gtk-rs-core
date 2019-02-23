// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use IconSize;
use Orientable;
use Orientation;
use ToolItem;
use ToolShell;
use ToolbarStyle;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Toolbar(Object<ffi::GtkToolbar, ffi::GtkToolbarClass, ToolbarClass>) @extends Container, Widget, @implements Buildable, Orientable, ToolShell;

    match fn {
        get_type => || ffi::gtk_toolbar_get_type(),
    }
}

impl Toolbar {
    pub fn new() -> Toolbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_toolbar_new()).unsafe_cast()
        }
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_TOOLBAR: Option<&Toolbar> = None;

pub trait ToolbarExt: 'static {
    fn get_drop_index(&self, x: i32, y: i32) -> i32;

    fn get_item_index<P: IsA<ToolItem>>(&self, item: &P) -> i32;

    fn get_n_items(&self) -> i32;

    fn get_nth_item(&self, n: i32) -> Option<ToolItem>;

    fn get_show_arrow(&self) -> bool;

    fn insert<P: IsA<ToolItem>>(&self, item: &P, pos: i32);

    fn set_drop_highlight_item<P: IsA<ToolItem>>(&self, tool_item: Option<&P>, index_: i32);

    fn set_icon_size(&self, icon_size: IconSize);

    fn set_show_arrow(&self, show_arrow: bool);

    fn set_style(&self, style: ToolbarStyle);

    fn unset_icon_size(&self);

    fn unset_style(&self);

    fn get_property_icon_size_set(&self) -> bool;

    fn set_property_icon_size_set(&self, icon_size_set: bool);

    fn get_property_toolbar_style(&self) -> ToolbarStyle;

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle);

    fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool);

    fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool);

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_focus_home_or_end(&self, focus_home: bool) -> bool;

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Toolbar>> ToolbarExt for O {
    fn get_drop_index(&self, x: i32, y: i32) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_drop_index(self.as_ref().to_glib_none().0, x, y)
        }
    }

    fn get_item_index<P: IsA<ToolItem>>(&self, item: &P) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_item_index(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0)
        }
    }

    fn get_n_items(&self) -> i32 {
        unsafe {
            ffi::gtk_toolbar_get_n_items(self.as_ref().to_glib_none().0)
        }
    }

    fn get_nth_item(&self, n: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_toolbar_get_nth_item(self.as_ref().to_glib_none().0, n))
        }
    }

    fn get_show_arrow(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_toolbar_get_show_arrow(self.as_ref().to_glib_none().0))
        }
    }

    fn insert<P: IsA<ToolItem>>(&self, item: &P, pos: i32) {
        unsafe {
            ffi::gtk_toolbar_insert(self.as_ref().to_glib_none().0, item.as_ref().to_glib_none().0, pos);
        }
    }

    fn set_drop_highlight_item<P: IsA<ToolItem>>(&self, tool_item: Option<&P>, index_: i32) {
        unsafe {
            ffi::gtk_toolbar_set_drop_highlight_item(self.as_ref().to_glib_none().0, tool_item.map(|p| p.as_ref()).to_glib_none().0, index_);
        }
    }

    fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_toolbar_set_icon_size(self.as_ref().to_glib_none().0, icon_size.to_glib());
        }
    }

    fn set_show_arrow(&self, show_arrow: bool) {
        unsafe {
            ffi::gtk_toolbar_set_show_arrow(self.as_ref().to_glib_none().0, show_arrow.to_glib());
        }
    }

    fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_toolbar_set_style(self.as_ref().to_glib_none().0, style.to_glib());
        }
    }

    fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_icon_size(self.as_ref().to_glib_none().0);
        }
    }

    fn unset_style(&self) {
        unsafe {
            ffi::gtk_toolbar_unset_style(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_icon_size_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-size-set\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"icon-size-set\0".as_ptr() as *const _, Value::from(&icon_size_set).to_glib_none().0);
        }
    }

    fn get_property_toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            let mut value = Value::from_type(<ToolbarStyle as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"toolbar-style\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"toolbar-style\0".as_ptr() as *const _, Value::from(&toolbar_style).to_glib_none().0);
        }
    }

    fn get_item_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"expand\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_expand<T: IsA<Widget>>(&self, item: &T, expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"expand\0".as_ptr() as *const _, Value::from(&expand).to_glib_none().0);
        }
    }

    fn get_item_homogeneous<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"homogeneous\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_item_homogeneous<T: IsA<Widget>>(&self, item: &T, homogeneous: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0 as *mut ffi::GtkContainer, item.to_glib_none().0 as *mut _, b"homogeneous\0".as_ptr() as *const _, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn connect_focus_home_or_end<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"focus-home-or-end\0".as_ptr() as *const _,
                Some(transmute(focus_home_or_end_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_focus_home_or_end(&self, focus_home: bool) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("focus-home-or-end", &[&focus_home]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_orientation_changed<F: Fn(&Self, Orientation) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"orientation-changed\0".as_ptr() as *const _,
                Some(transmute(orientation_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_popup_context_menu<F: Fn(&Self, i32, i32, i32) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"popup-context-menu\0".as_ptr() as *const _,
                Some(transmute(popup_context_menu_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_style_changed<F: Fn(&Self, ToolbarStyle) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"style-changed\0".as_ptr() as *const _,
                Some(transmute(style_changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute(notify_icon_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon-size-set\0".as_ptr() as *const _,
                Some(transmute(notify_icon_size_set_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_arrow_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-arrow\0".as_ptr() as *const _,
                Some(transmute(notify_show_arrow_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::toolbar-style\0".as_ptr() as *const _,
                Some(transmute(notify_toolbar_style_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn focus_home_or_end_trampoline<P, F: Fn(&P, bool) -> bool + 'static>(this: *mut ffi::GtkToolbar, focus_home: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast(), from_glib(focus_home)).to_glib()
}

unsafe extern "C" fn orientation_changed_trampoline<P, F: Fn(&P, Orientation) + 'static>(this: *mut ffi::GtkToolbar, orientation: ffi::GtkOrientation, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast(), from_glib(orientation))
}

unsafe extern "C" fn popup_context_menu_trampoline<P, F: Fn(&P, i32, i32, i32) -> Inhibit + 'static>(this: *mut ffi::GtkToolbar, x: libc::c_int, y: libc::c_int, button: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast(), x, y, button).to_glib()
}

unsafe extern "C" fn style_changed_trampoline<P, F: Fn(&P, ToolbarStyle) + 'static>(this: *mut ffi::GtkToolbar, style: ffi::GtkToolbarStyle, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast(), from_glib(style))
}

unsafe extern "C" fn notify_icon_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_icon_size_set_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_arrow_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_toolbar_style_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkToolbar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Toolbar> {
    let f: &F = &*(f as *const F);
    f(&Toolbar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Toolbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Toolbar")
    }
}
