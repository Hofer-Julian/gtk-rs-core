// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use crate::KeymapKey;
use crate::ModifierIntent;
use crate::ModifierType;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    pub struct Keymap(Object<ffi::GdkKeymap>);

    match fn {
        get_type => || ffi::gdk_keymap_get_type(),
    }
}

impl Keymap {
    #[doc(alias = "gdk_keymap_get_caps_lock_state")]
    pub fn is_caps_locked(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_get_caps_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_get_direction")]
    pub fn direction(&self) -> pango::Direction {
        unsafe { from_glib(ffi::gdk_keymap_get_direction(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_get_modifier_mask")]
    pub fn get_modifier_mask(&self, intent: ModifierIntent) -> ModifierType {
        unsafe {
            from_glib(ffi::gdk_keymap_get_modifier_mask(
                self.to_glib_none().0,
                intent.to_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_keymap_get_modifier_state")]
    pub fn modifier_state(&self) -> u32 {
        unsafe { ffi::gdk_keymap_get_modifier_state(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_keymap_get_num_lock_state")]
    pub fn is_num_locked(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_get_num_lock_state(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_18")))]
    #[doc(alias = "gdk_keymap_get_scroll_lock_state")]
    pub fn is_scroll_locked(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_get_scroll_lock_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_have_bidi_layouts")]
    pub fn have_bidi_layouts(&self) -> bool {
        unsafe { from_glib(ffi::gdk_keymap_have_bidi_layouts(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_keymap_lookup_key")]
    pub fn lookup_key(&self, key: &KeymapKey) -> u32 {
        unsafe { ffi::gdk_keymap_lookup_key(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "gdk_keymap_translate_keyboard_state")]
    pub fn translate_keyboard_state(
        &self,
        hardware_keycode: u32,
        state: ModifierType,
        group: i32,
    ) -> Option<(u32, i32, i32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut effective_group = mem::MaybeUninit::uninit();
            let mut level = mem::MaybeUninit::uninit();
            let mut consumed_modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_keymap_translate_keyboard_state(
                self.to_glib_none().0,
                hardware_keycode,
                state.to_glib(),
                group,
                keyval.as_mut_ptr(),
                effective_group.as_mut_ptr(),
                level.as_mut_ptr(),
                consumed_modifiers.as_mut_ptr(),
            ));
            let keyval = keyval.assume_init();
            let effective_group = effective_group.assume_init();
            let level = level.assume_init();
            let consumed_modifiers = consumed_modifiers.assume_init();
            if ret {
                Some((
                    keyval,
                    effective_group,
                    level,
                    from_glib(consumed_modifiers),
                ))
            } else {
                None
            }
        }
    }

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[doc(alias = "gdk_keymap_get_default")]
    pub fn get_default() -> Option<Keymap> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_keymap_get_default()) }
    }

    #[doc(alias = "gdk_keymap_get_for_display")]
    pub fn get_for_display(display: &Display) -> Option<Keymap> {
        skip_assert_initialized!();
        unsafe { from_glib_none(ffi::gdk_keymap_get_for_display(display.to_glib_none().0)) }
    }

    pub fn connect_direction_changed<F: Fn(&Keymap) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn direction_changed_trampoline<F: Fn(&Keymap) + 'static>(
            this: *mut ffi::GdkKeymap,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"direction-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    direction_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_keys_changed<F: Fn(&Keymap) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn keys_changed_trampoline<F: Fn(&Keymap) + 'static>(
            this: *mut ffi::GdkKeymap,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"keys-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    keys_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_state_changed<F: Fn(&Keymap) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<F: Fn(&Keymap) + 'static>(
            this: *mut ffi::GdkKeymap,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Keymap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Keymap")
    }
}
