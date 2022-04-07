// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Colorspace;
use crate::InterpType;
use crate::PixbufFormat;
use crate::PixbufRotation;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GdkPixbuf")]
    pub struct Pixbuf(Object<ffi::GdkPixbuf>) @implements gio::Icon, gio::LoadableIcon;

    match fn {
        type_ => || ffi::gdk_pixbuf_get_type(),
    }
}

impl Pixbuf {
    #[doc(alias = "gdk_pixbuf_new")]
    pub fn new(
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
    ) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new(
                colorspace.into_glib(),
                has_alpha.into_glib(),
                bits_per_sample,
                width,
                height,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_new_from_bytes")]
    #[doc(alias = "new_from_bytes")]
    pub fn from_bytes(
        data: &glib::Bytes,
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
        rowstride: i32,
    ) -> Pixbuf {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new_from_bytes(
                data.to_glib_none().0,
                colorspace.into_glib(),
                has_alpha.into_glib(),
                bits_per_sample,
                width,
                height,
                rowstride,
            ))
        }
    }

    //#[doc(alias = "gdk_pixbuf_new_from_data")]
    //#[doc(alias = "new_from_data")]
    //pub fn from_data(data: &[u8], colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32, height: i32, rowstride: i32, destroy_fn: Option<Box_<dyn FnOnce(&Vec<u8>) + 'static>>) -> Pixbuf {
    //    unsafe { TODO: call ffi:gdk_pixbuf_new_from_data() }
    //}

    #[doc(alias = "gdk_pixbuf_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> Result<Pixbuf, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_resource(resource_path.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_new_from_resource_at_scale")]
    #[doc(alias = "new_from_resource_at_scale")]
    pub fn from_resource_at_scale(
        resource_path: &str,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
    ) -> Result<Pixbuf, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_resource_at_scale(
                resource_path.to_glib_none().0,
                width,
                height,
                preserve_aspect_ratio.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_new_from_stream")]
    #[doc(alias = "new_from_stream")]
    pub fn from_stream(
        stream: &impl IsA<gio::InputStream>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<Pixbuf, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_stream(
                stream.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_new_from_stream_at_scale")]
    #[doc(alias = "new_from_stream_at_scale")]
    pub fn from_stream_at_scale(
        stream: &impl IsA<gio::InputStream>,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
    ) -> Result<Pixbuf, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_new_from_stream_at_scale(
                stream.as_ref().to_glib_none().0,
                width,
                height,
                preserve_aspect_ratio.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_new_from_xpm_data")]
    #[doc(alias = "new_from_xpm_data")]
    pub fn from_xpm_data(data: &[&str]) -> Pixbuf {
        unsafe { from_glib_full(ffi::gdk_pixbuf_new_from_xpm_data(data.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_pixbuf_add_alpha")]
    #[must_use]
    pub fn add_alpha(&self, substitute_color: bool, r: u8, g: u8, b: u8) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_add_alpha(
                self.to_glib_none().0,
                substitute_color.into_glib(),
                r,
                g,
                b,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_apply_embedded_orientation")]
    #[must_use]
    pub fn apply_embedded_orientation(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_apply_embedded_orientation(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_composite")]
    pub fn composite(
        &self,
        dest: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
        dest_width: i32,
        dest_height: i32,
        offset_x: f64,
        offset_y: f64,
        scale_x: f64,
        scale_y: f64,
        interp_type: InterpType,
        overall_alpha: i32,
    ) {
        unsafe {
            ffi::gdk_pixbuf_composite(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
                offset_x,
                offset_y,
                scale_x,
                scale_y,
                interp_type.into_glib(),
                overall_alpha,
            );
        }
    }

    #[doc(alias = "gdk_pixbuf_composite_color")]
    pub fn composite_color(
        &self,
        dest: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
        dest_width: i32,
        dest_height: i32,
        offset_x: f64,
        offset_y: f64,
        scale_x: f64,
        scale_y: f64,
        interp_type: InterpType,
        overall_alpha: i32,
        check_x: i32,
        check_y: i32,
        check_size: i32,
        color1: u32,
        color2: u32,
    ) {
        unsafe {
            ffi::gdk_pixbuf_composite_color(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
                offset_x,
                offset_y,
                scale_x,
                scale_y,
                interp_type.into_glib(),
                overall_alpha,
                check_x,
                check_y,
                check_size,
                color1,
                color2,
            );
        }
    }

    #[doc(alias = "gdk_pixbuf_composite_color_simple")]
    #[must_use]
    pub fn composite_color_simple(
        &self,
        dest_width: i32,
        dest_height: i32,
        interp_type: InterpType,
        overall_alpha: i32,
        check_size: i32,
        color1: u32,
        color2: u32,
    ) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_composite_color_simple(
                self.to_glib_none().0,
                dest_width,
                dest_height,
                interp_type.into_glib(),
                overall_alpha,
                check_size,
                color1,
                color2,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_copy")]
    #[must_use]
    pub fn copy(&self) -> Option<Pixbuf> {
        unsafe { from_glib_full(ffi::gdk_pixbuf_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_pixbuf_copy_area")]
    pub fn copy_area(
        &self,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
        dest_pixbuf: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
    ) {
        unsafe {
            ffi::gdk_pixbuf_copy_area(
                self.to_glib_none().0,
                src_x,
                src_y,
                width,
                height,
                dest_pixbuf.to_glib_none().0,
                dest_x,
                dest_y,
            );
        }
    }

    #[doc(alias = "gdk_pixbuf_copy_options")]
    pub fn copy_options(&self, dest_pixbuf: &Pixbuf) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_copy_options(
                self.to_glib_none().0,
                dest_pixbuf.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_fill")]
    pub fn fill(&self, pixel: u32) {
        unsafe {
            ffi::gdk_pixbuf_fill(self.to_glib_none().0, pixel);
        }
    }

    #[doc(alias = "gdk_pixbuf_flip")]
    #[must_use]
    pub fn flip(&self, horizontal: bool) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_flip(
                self.to_glib_none().0,
                horizontal.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_get_bits_per_sample")]
    #[doc(alias = "get_bits_per_sample")]
    pub fn bits_per_sample(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_bits_per_sample(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_get_byte_length")]
    #[doc(alias = "get_byte_length")]
    pub fn byte_length(&self) -> usize {
        unsafe { ffi::gdk_pixbuf_get_byte_length(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_get_colorspace")]
    #[doc(alias = "get_colorspace")]
    pub fn colorspace(&self) -> Colorspace {
        unsafe { from_glib(ffi::gdk_pixbuf_get_colorspace(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_pixbuf_get_has_alpha")]
    #[doc(alias = "get_has_alpha")]
    pub fn has_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gdk_pixbuf_get_has_alpha(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_pixbuf_get_height")]
    #[doc(alias = "get_height")]
    pub fn height(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_height(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_get_n_channels")]
    #[doc(alias = "get_n_channels")]
    pub fn n_channels(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_n_channels(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_get_option")]
    #[doc(alias = "get_option")]
    pub fn option(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_get_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gdk_pixbuf_get_options")]
    //#[doc(alias = "get_options")]
    //pub fn options(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call ffi:gdk_pixbuf_get_options() }
    //}

    #[doc(alias = "gdk_pixbuf_get_rowstride")]
    #[doc(alias = "get_rowstride")]
    pub fn rowstride(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_rowstride(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_get_width")]
    #[doc(alias = "get_width")]
    pub fn width(&self) -> i32 {
        unsafe { ffi::gdk_pixbuf_get_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_pixbuf_new_subpixbuf")]
    #[must_use]
    pub fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new_subpixbuf(
                self.to_glib_none().0,
                src_x,
                src_y,
                width,
                height,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_read_pixel_bytes")]
    pub fn read_pixel_bytes(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_full(ffi::gdk_pixbuf_read_pixel_bytes(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_pixbuf_remove_option")]
    pub fn remove_option(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_remove_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_rotate_simple")]
    #[must_use]
    pub fn rotate_simple(&self, angle: PixbufRotation) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_rotate_simple(
                self.to_glib_none().0,
                angle.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_saturate_and_pixelate")]
    pub fn saturate_and_pixelate(&self, dest: &Pixbuf, saturation: f32, pixelate: bool) {
        unsafe {
            ffi::gdk_pixbuf_saturate_and_pixelate(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                saturation,
                pixelate.into_glib(),
            );
        }
    }

    //#[doc(alias = "gdk_pixbuf_save")]
    //pub fn save(&self, filename: impl AsRef<std::path::Path>, type_: &str, error: Option<&mut glib::Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:gdk_pixbuf_save() }
    //}

    //#[doc(alias = "gdk_pixbuf_save_to_buffer")]
    //pub fn save_to_buffer(&self, type_: &str, error: Option<&mut glib::Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Vec<u8>> {
    //    unsafe { TODO: call ffi:gdk_pixbuf_save_to_buffer() }
    //}

    //#[doc(alias = "gdk_pixbuf_save_to_callback")]
    //pub fn save_to_callback<P: FnMut(&Vec<u8>, usize, &glib::Error) -> bool>(&self, save_func: P, type_: &str, error: Option<&mut glib::Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:gdk_pixbuf_save_to_callback() }
    //}

    //#[doc(alias = "gdk_pixbuf_save_to_callbackv")]
    //pub fn save_to_callbackv<P: FnMut(&Vec<u8>, usize, &glib::Error) -> bool>(&self, save_func: P, type_: &str, option_keys: &[&str], option_values: &[&str]) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:gdk_pixbuf_save_to_callbackv() }
    //}

    //#[doc(alias = "gdk_pixbuf_save_to_stream")]
    //pub fn save_to_stream(&self, stream: &impl IsA<gio::OutputStream>, type_: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, error: Option<&mut glib::Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:gdk_pixbuf_save_to_stream() }
    //}

    //#[doc(alias = "gdk_pixbuf_save_to_stream_async")]
    //pub fn save_to_stream_async<P: FnOnce(Result<(), glib::Error>) + 'static>(&self, stream: &impl IsA<gio::OutputStream>, type_: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gdk_pixbuf_save_to_stream_async() }
    //}

    //
    //pub fn save_to_stream_future(&self, stream: &(impl IsA<gio::OutputStream> + Clone + 'static), type_: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {

    //let stream = stream.clone();
    //let type_ = String::from(type_);
    //Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
    //    obj.save_to_stream_async(
    //        &stream,
    //        &type_,
    //        Some(cancellable),
    //        ,
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );
    //}))
    //}

    #[doc(alias = "gdk_pixbuf_scale")]
    pub fn scale(
        &self,
        dest: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
        dest_width: i32,
        dest_height: i32,
        offset_x: f64,
        offset_y: f64,
        scale_x: f64,
        scale_y: f64,
        interp_type: InterpType,
    ) {
        unsafe {
            ffi::gdk_pixbuf_scale(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
                offset_x,
                offset_y,
                scale_x,
                scale_y,
                interp_type.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_pixbuf_scale_simple")]
    #[must_use]
    pub fn scale_simple(
        &self,
        dest_width: i32,
        dest_height: i32,
        interp_type: InterpType,
    ) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_scale_simple(
                self.to_glib_none().0,
                dest_width,
                dest_height,
                interp_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_set_option")]
    pub fn set_option(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_pixbuf_set_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pixel-bytes")]
    pub fn pixel_bytes(&self) -> Option<glib::Bytes> {
        glib::ObjectExt::property(self, "pixel-bytes")
    }

    #[doc(alias = "gdk_pixbuf_calculate_rowstride")]
    pub fn calculate_rowstride(
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
    ) -> i32 {
        unsafe {
            ffi::gdk_pixbuf_calculate_rowstride(
                colorspace.into_glib(),
                has_alpha.into_glib(),
                bits_per_sample,
                width,
                height,
            )
        }
    }

    #[doc(alias = "gdk_pixbuf_get_formats")]
    #[doc(alias = "get_formats")]
    pub fn formats() -> Vec<PixbufFormat> {
        unsafe { FromGlibPtrContainer::from_glib_container(ffi::gdk_pixbuf_get_formats()) }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
    #[doc(alias = "gdk_pixbuf_init_modules")]
    pub fn init_modules(path: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gdk_pixbuf_init_modules(path.to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for Pixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Pixbuf")
    }
}
