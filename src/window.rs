/* window.rs
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
use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/window.ui")]
    pub struct GciphersRsWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GciphersRsWindow {
        const NAME: &'static str = "GciphersRsWindow";
        type Type = super::GciphersRsWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GciphersRsWindow {}
    impl WidgetImpl for GciphersRsWindow {}
    impl WindowImpl for GciphersRsWindow {}
    impl ApplicationWindowImpl for GciphersRsWindow {}
    impl AdwApplicationWindowImpl for GciphersRsWindow {}
}

glib::wrapper! {
    pub struct GciphersRsWindow(ObjectSubclass<imp::GciphersRsWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl GciphersRsWindow {
    pub fn new<P>(application: &P) -> Self
        where P: IsA<gtk::Application>
    {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
