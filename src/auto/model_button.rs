// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::value::SetValueOptional;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::GString;
use glib::StaticType;
use glib::ToValue;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use gobject_sys;
use gtk_sys;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use std::mem::transmute;
use Actionable;
use Align;
use Bin;
use Buildable;
use Button;
#[cfg(any(feature = "v3_16", feature = "dox"))]
use ButtonRole;
use Container;
use PositionType;
use ReliefStyle;
use ResizeMode;
use Widget;

glib_wrapper! {
    pub struct ModelButton(Object<gtk_sys::GtkModelButton, ModelButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_model_button_get_type(),
    }
}

impl ModelButton {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn new() -> ModelButton {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(gtk_sys::gtk_model_button_new()).unsafe_cast() }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"active\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `active` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"active\0".as_ptr() as *const _,
                Value::from(&active).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_centered(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"centered\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `centered` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_centered(&self, centered: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"centered\0".as_ptr() as *const _,
                Value::from(&centered).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_icon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"icon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon` getter")
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_icon<P: IsA<gio::Icon> + SetValueOptional>(&self, icon: Option<&P>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"icon\0".as_ptr() as *const _,
                Value::from(icon).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_iconic(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"iconic\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `iconic` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_iconic(&self, iconic: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"iconic\0".as_ptr() as *const _,
                Value::from(&iconic).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_inverted(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"inverted\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `inverted` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"inverted\0".as_ptr() as *const _,
                Value::from(&inverted).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_menu_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"menu-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `menu-name` getter")
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_menu_name(&self, menu_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"menu-name\0".as_ptr() as *const _,
                Value::from(menu_name).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_role(&self) -> ButtonRole {
        unsafe {
            let mut value = Value::from_type(<ButtonRole as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"role\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `role` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_role(&self, role: ButtonRole) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"role\0".as_ptr() as *const _,
                Value::from(&role).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `text` getter")
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"text\0".as_ptr() as *const _,
                Value::from(text).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn get_property_use_markup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"use-markup\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `use-markup` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn set_property_use_markup(&self, use_markup: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"use-markup\0".as_ptr() as *const _,
                Value::from(&use_markup).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_active_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_centered_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_centered_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::centered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_centered_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_icon_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_iconic_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_iconic_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::iconic\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_iconic_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_inverted_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_inverted_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_menu_name_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_name_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::menu-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menu_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_role_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_role_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::role\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_role_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn connect_property_text_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn connect_property_use_markup_notify<F: Fn(&ModelButton) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_markup_trampoline<F: Fn(&ModelButton) + 'static>(
            this: *mut gtk_sys::GtkModelButton,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_markup_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
impl Default for ModelButton {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct ModelButtonBuilder {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    active: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    centered: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    icon: Option<gio::Icon>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    iconic: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    inverted: Option<bool>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    menu_name: Option<String>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    role: Option<ButtonRole>,
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    text: Option<String>,
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    use_markup: Option<bool>,
    always_show_image: Option<bool>,
    image: Option<Widget>,
    image_position: Option<PositionType>,
    label: Option<String>,
    relief: Option<ReliefStyle>,
    use_underline: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    action_name: Option<String>,
    action_target: Option<glib::Variant>,
}

impl ModelButtonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> ModelButton {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref active) = self.active {
                properties.push(("active", active));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref centered) = self.centered {
                properties.push(("centered", centered));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref icon) = self.icon {
                properties.push(("icon", icon));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref iconic) = self.iconic {
                properties.push(("iconic", iconic));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref inverted) = self.inverted {
                properties.push(("inverted", inverted));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref menu_name) = self.menu_name {
                properties.push(("menu-name", menu_name));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref role) = self.role {
                properties.push(("role", role));
            }
        }
        #[cfg(any(feature = "v3_16", feature = "dox"))]
        {
            if let Some(ref text) = self.text {
                properties.push(("text", text));
            }
        }
        #[cfg(any(feature = "v3_24", feature = "dox"))]
        {
            if let Some(ref use_markup) = self.use_markup {
                properties.push(("use-markup", use_markup));
            }
        }
        if let Some(ref always_show_image) = self.always_show_image {
            properties.push(("always-show-image", always_show_image));
        }
        if let Some(ref image) = self.image {
            properties.push(("image", image));
        }
        if let Some(ref image_position) = self.image_position {
            properties.push(("image-position", image_position));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref relief) = self.relief {
            properties.push(("relief", relief));
        }
        if let Some(ref use_underline) = self.use_underline {
            properties.push(("use-underline", use_underline));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        {
            if let Some(ref focus_on_click) = self.focus_on_click {
                properties.push(("focus-on-click", focus_on_click));
            }
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref action_name) = self.action_name {
            properties.push(("action-name", action_name));
        }
        if let Some(ref action_target) = self.action_target {
            properties.push(("action-target", action_target));
        }
        glib::Object::new(ModelButton::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = Some(centered);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn icon<P: IsA<gio::Icon>>(mut self, icon: &P) -> Self {
        self.icon = Some(icon.clone().upcast());
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn iconic(mut self, iconic: bool) -> Self {
        self.iconic = Some(iconic);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = Some(inverted);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn menu_name(mut self, menu_name: &str) -> Self {
        self.menu_name = Some(menu_name.to_string());
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn role(mut self, role: ButtonRole) -> Self {
        self.role = Some(role);
        self
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn use_markup(mut self, use_markup: bool) -> Self {
        self.use_markup = Some(use_markup);
        self
    }

    pub fn always_show_image(mut self, always_show_image: bool) -> Self {
        self.always_show_image = Some(always_show_image);
        self
    }

    pub fn image<P: IsA<Widget>>(mut self, image: &P) -> Self {
        self.image = Some(image.clone().upcast());
        self
    }

    pub fn image_position(mut self, image_position: PositionType) -> Self {
        self.image_position = Some(image_position);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn relief(mut self, relief: ReliefStyle) -> Self {
        self.relief = Some(relief);
        self
    }

    pub fn use_underline(mut self, use_underline: bool) -> Self {
        self.use_underline = Some(use_underline);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn action_name(mut self, action_name: &str) -> Self {
        self.action_name = Some(action_name.to_string());
        self
    }

    pub fn action_target(mut self, action_target: &glib::Variant) -> Self {
        self.action_target = Some(action_target.clone());
        self
    }
}

impl fmt::Display for ModelButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ModelButton")
    }
}
