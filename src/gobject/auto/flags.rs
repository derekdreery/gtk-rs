// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use StaticType;
use Type;
use gobject_ffi as ffi;
use gobject_ffi;
use translate::*;
use value::FromValue;
use value::FromValueOptional;
use value::SetValue;
use value::Value;

bitflags! {
    pub struct BindingFlags: u32 {
        const DEFAULT = 0;
        const BIDIRECTIONAL = 1;
        const SYNC_CREATE = 2;
        const INVERT_BOOLEAN = 4;
    }
}

#[doc(hidden)]
impl ToGlib for BindingFlags {
    type GlibType = ffi::GBindingFlags;

    fn to_glib(&self) -> ffi::GBindingFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GBindingFlags> for BindingFlags {
    fn from_glib(value: ffi::GBindingFlags) -> BindingFlags {
        BindingFlags::from_bits_truncate(value)
    }
}

impl StaticType for BindingFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_binding_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BindingFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BindingFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for BindingFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

