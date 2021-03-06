// Copyright 2013-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::{Cast, IsA};
use glib::translate::*;
use gtk_sys;
use libc::c_char;
use std::ptr;
use ButtonsType;
use DialogFlags;
use MessageDialog;
use MessageType;
use Widget;
use Window;

impl MessageDialog {
    pub fn new<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: &str,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(gtk_sys::gtk_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                type_.to_glib(),
                buttons.to_glib(),
                b"%s\0".as_ptr() as *const c_char,
                message.0,
                ptr::null::<c_char>(),
            ))
            .unsafe_cast()
        }
    }
}

pub trait MessageDialogExt: 'static {
    fn set_secondary_markup(&self, message: Option<&str>);

    fn set_secondary_text(&self, message: Option<&str>);
}

impl<O: IsA<MessageDialog>> MessageDialogExt for O {
    fn set_secondary_markup(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                gtk_sys::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    b"%s\0".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                gtk_sys::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }

    fn set_secondary_text(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                gtk_sys::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    b"%s\0".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                gtk_sys::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }
}
