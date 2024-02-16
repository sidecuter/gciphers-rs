/* entry.rs
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

use gtk::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/entry.ui")]
    pub struct UIEntry {}

    #[glib::object_subclass]
    impl ObjectSubclass for UIEntry {
        const NAME: &'static str = "UIEntry";
        type Type = super::UIEntry;
        type ParentType = gtk::Entry;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UIEntry {}
    impl WidgetImpl for UIEntry {}
    impl EntryImpl for UIEntry {}
}

glib::wrapper! {
    pub struct UIEntry(ObjectSubclass<imp::UIEntry>)
        @extends gtk::Entry, gtk::Widget,
        @implements gtk::Accessible, gtk::CellEditable, gtk::Editable, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl UIEntry {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
