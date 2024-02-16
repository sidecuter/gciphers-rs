/* text_view.rs
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

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/text_view.ui")]
    pub struct UITextView {
        #[template_child]
        pub text: TemplateChild<gtk::TextBuffer>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UITextView {
        const NAME: &'static str = "UITextView";
        type Type = super::UITextView;
        type ParentType = gtk::TextView;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UITextView {}
    impl WidgetImpl for UITextView {}
    impl TextViewImpl for UITextView {}
}

glib::wrapper! {
    pub struct UITextView(ObjectSubclass<imp::UITextView>)
        @extends gtk::TextView, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}

impl UITextView {
    pub fn get_text(&self) -> String {
        self.imp().text.get().text(
            &self.imp().text.get().start_iter(),
            &self.imp().text.get().end_iter(),
            false
        ).to_string()
    }

    pub fn set_text(&self, text: &str) {
        self.imp().text.set_text(text);
    }

    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
