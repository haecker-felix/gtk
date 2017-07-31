// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use Container;
use MenuShell;
use PackDirection;
use Widget;
use ffi;
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MenuBar(Object<ffi::GtkMenuBar>): MenuShell, Container, Widget;

    match fn {
        get_type => || ffi::gtk_menu_bar_get_type(),
    }
}

impl MenuBar {
    pub fn new() -> MenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_bar_new()).downcast_unchecked()
        }
    }

    pub fn new_from_model<P: IsA<gio::MenuModel>>(model: &P) -> MenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_bar_new_from_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait MenuBarExt {
    fn get_child_pack_direction(&self) -> PackDirection;

    fn get_pack_direction(&self) -> PackDirection;

    fn set_child_pack_direction(&self, child_pack_dir: PackDirection);

    fn set_pack_direction(&self, pack_dir: PackDirection);
}

impl<O: IsA<MenuBar>> MenuBarExt for O {
    fn get_child_pack_direction(&self) -> PackDirection {
        unsafe {
            from_glib(ffi::gtk_menu_bar_get_child_pack_direction(self.to_glib_none().0))
        }
    }

    fn get_pack_direction(&self) -> PackDirection {
        unsafe {
            from_glib(ffi::gtk_menu_bar_get_pack_direction(self.to_glib_none().0))
        }
    }

    fn set_child_pack_direction(&self, child_pack_dir: PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_child_pack_direction(self.to_glib_none().0, child_pack_dir.to_glib());
        }
    }

    fn set_pack_direction(&self, pack_dir: PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_pack_direction(self.to_glib_none().0, pack_dir.to_glib());
        }
    }
}
