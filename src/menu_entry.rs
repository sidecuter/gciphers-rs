/* menu_entry.rs
 *
 * Copyright 2024 Alexander Svobodov
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;
use gtk::glib::GString;

mod imp {
    use super::*;
    use glib::Properties;
    use std::cell::RefCell;
    use gtk::glib::GString;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::GCiphersMenuEntry)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/menu_entry.ui")]
    pub struct GCiphersMenuEntry {
        #[template_child]
        pub item_label: TemplateChild<gtk::Label>,
        #[property(name="lvalue", get, set)]
        pub lvalue: RefCell<GString>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersMenuEntry {
        const NAME: &'static str = "GCiphersMenuEntry";
        type Type = super::GCiphersMenuEntry;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for GCiphersMenuEntry {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.bind_property("lvalue", &self.item_label.get(), "label")
                .sync_create()
                .build();
        }
    }
    impl WidgetImpl for GCiphersMenuEntry {}
    impl ListBoxRowImpl for GCiphersMenuEntry {}
}

glib::wrapper! {
    pub struct GCiphersMenuEntry(ObjectSubclass<imp::GCiphersMenuEntry>)
        @extends gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersMenuEntry {
    pub fn new(name: &str) -> Self {
        glib::Object::builder()
            .property("lvalue", &GString::from(name))
            .build()
    }
}
