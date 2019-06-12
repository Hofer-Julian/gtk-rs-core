// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Converter;
use FileInfo;
use ZlibCompressorFormat;

glib_wrapper! {
    pub struct ZlibDecompressor(Object<gio_sys::GZlibDecompressor, gio_sys::GZlibDecompressorClass, ZlibDecompressorClass>) @implements Converter;

    match fn {
        get_type => || gio_sys::g_zlib_decompressor_get_type(),
    }
}

impl ZlibDecompressor {
    pub fn new(format: ZlibCompressorFormat) -> ZlibDecompressor {
        unsafe { from_glib_full(gio_sys::g_zlib_decompressor_new(format.to_glib())) }
    }
}

pub const NONE_ZLIB_DECOMPRESSOR: Option<&ZlibDecompressor> = None;

pub trait ZlibDecompressorExt: 'static {
    fn get_file_info(&self) -> Option<FileInfo>;

    fn get_property_format(&self) -> ZlibCompressorFormat;

    fn connect_property_file_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ZlibDecompressor>> ZlibDecompressorExt for O {
    fn get_file_info(&self) -> Option<FileInfo> {
        unsafe {
            from_glib_none(gio_sys::g_zlib_decompressor_get_file_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_format(&self) -> ZlibCompressorFormat {
        unsafe {
            let mut value = Value::from_type(<ZlibCompressorFormat as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"format\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn connect_property_file_info_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_info_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gio_sys::GZlibDecompressor,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ZlibDecompressor>,
        {
            let f: &F = &*(f as *const F);
            f(&ZlibDecompressor::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::file-info\0".as_ptr() as *const _,
                Some(transmute(notify_file_info_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ZlibDecompressor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZlibDecompressor")
    }
}
