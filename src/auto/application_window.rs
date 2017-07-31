// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use Application;
use Bin;
use Container;
#[cfg(feature = "v3_20")]
use ShortcutsWindow;
use Widget;
use Window;
use ffi;
use gio;
use gio_ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ApplicationWindow(Object<ffi::GtkApplicationWindow>): [
        Window,
        Bin,
        Container,
        Widget,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_window_get_type(),
    }
}

impl ApplicationWindow {
    pub fn new(application: &Application) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_application_window_new(application.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ApplicationWindowExt {
    #[cfg(feature = "v3_20")]
    fn get_help_overlay(&self) -> Option<ShortcutsWindow>;

    #[cfg(feature = "v3_6")]
    fn get_id(&self) -> u32;

    fn get_show_menubar(&self) -> bool;

    #[cfg(feature = "v3_20")]
    fn set_help_overlay<'a, P: Into<Option<&'a ShortcutsWindow>>>(&self, help_overlay: P);

    fn set_show_menubar(&self, show_menubar: bool);
}

impl<O: IsA<ApplicationWindow>> ApplicationWindowExt for O {
    #[cfg(feature = "v3_20")]
    fn get_help_overlay(&self) -> Option<ShortcutsWindow> {
        unsafe {
            from_glib_none(ffi::gtk_application_window_get_help_overlay(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    fn get_id(&self) -> u32 {
        unsafe {
            ffi::gtk_application_window_get_id(self.to_glib_none().0)
        }
    }

    fn get_show_menubar(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_window_get_show_menubar(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    fn set_help_overlay<'a, P: Into<Option<&'a ShortcutsWindow>>>(&self, help_overlay: P) {
        let help_overlay = help_overlay.into();
        let help_overlay = help_overlay.to_glib_none();
        unsafe {
            ffi::gtk_application_window_set_help_overlay(self.to_glib_none().0, help_overlay.0);
        }
    }

    fn set_show_menubar(&self, show_menubar: bool) {
        unsafe {
            ffi::gtk_application_window_set_show_menubar(self.to_glib_none().0, show_menubar.to_glib());
        }
    }
}
