// This file was generated by gir (af5277e) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererCombo(Object<ffi::GtkCellRendererCombo>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_combo_get_type(),
    }
}

impl CellRendererCombo {
    pub fn new() -> CellRendererCombo {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_combo_new()).downcast_unchecked()
        }
    }
}
