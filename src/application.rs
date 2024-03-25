/* application.rs
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

use crate::config::VERSION;
use crate::GCiphersRsWindow;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GciphersRsApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for GciphersRsApplication {
        const NAME: &'static str = "GciphersRsApplication";
        type Type = super::GciphersRsApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for GciphersRsApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for GciphersRsApplication {
        fn activate(&self) {
            let application = self.obj();
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = GCiphersRsWindow::new(&*application);
                window.upcast()
            };
            window.present();
            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/com/github/sidecuter/gciphers_rs/style.css");
            gtk::style_context_add_provider_for_display(&window.style_context().display(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
    }

    impl GtkApplicationImpl for GciphersRsApplication {}
    impl AdwApplicationImpl for GciphersRsApplication {}
}

glib::wrapper! {
    pub struct GciphersRsApplication(ObjectSubclass<imp::GciphersRsApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GciphersRsApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("gciphers-rs")
            .application_icon("com.github.sidecuter.gciphers_rs")
            .developer_name("Alexander Svobodov")
            .version(VERSION)
            .developers(vec!["Alexander Svobodov"])
            .copyright("© 2024 Alexander Svobodov")
            .license_type(gtk::License::Gpl30)
            .build();
        about.present();
    }
}
