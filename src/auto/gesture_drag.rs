// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureDrag(Object<ffi::GtkGestureDrag, ffi::GtkGestureDragClass>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_drag_get_type(),
    }
}

impl GestureDrag {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureDrag {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_drag_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureDragExt: 'static {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_offset(&self) -> Option<(f64, f64)>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_start_point(&self) -> Option<(f64, f64)>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureDrag>> GestureDragExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_offset(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_offset(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_start_point(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::uninitialized();
            let mut y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_drag_get_start_point(self.to_glib_none().0, &mut x, &mut y));
            if ret { Some((x, y)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"drag-begin\0".as_ptr() as *const _,
                transmute(drag_begin_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"drag-end\0".as_ptr() as *const _,
                transmute(drag_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"drag-update\0".as_ptr() as *const _,
                transmute(drag_update_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn drag_begin_trampoline<P>(this: *mut ffi::GtkGestureDrag, start_x: libc::c_double, start_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureDrag> {
    let f: &&(Fn(&P, f64, f64) + 'static) = transmute(f);
    f(&GestureDrag::from_glib_borrow(this).downcast_unchecked(), start_x, start_y)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn drag_end_trampoline<P>(this: *mut ffi::GtkGestureDrag, offset_x: libc::c_double, offset_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureDrag> {
    let f: &&(Fn(&P, f64, f64) + 'static) = transmute(f);
    f(&GestureDrag::from_glib_borrow(this).downcast_unchecked(), offset_x, offset_y)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn drag_update_trampoline<P>(this: *mut ffi::GtkGestureDrag, offset_x: libc::c_double, offset_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureDrag> {
    let f: &&(Fn(&P, f64, f64) + 'static) = transmute(f);
    f(&GestureDrag::from_glib_borrow(this).downcast_unchecked(), offset_x, offset_y)
}

impl fmt::Display for GestureDrag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureDrag")
    }
}
