/* polybius.rs
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

use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use gtk::{Button, template_callbacks};
    use gtk::prelude::WidgetExt;
    use crate::ui::entry::UIEntry;

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::polybius::*;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/polybius.ui")]
    pub struct GCiphersRsPolybius {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub rows: TemplateChild<UIEntry>,
        #[template_child]
        pub columns: TemplateChild<UIEntry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsPolybius {
        const NAME: &'static str = "GCiphersRsPolybius";
        type Type = super::GCiphersRsPolybius;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsPolybius {}
    impl WidgetImpl for GCiphersRsPolybius {}
    impl BinImpl for GCiphersRsPolybius {}

    #[template_callbacks]
    impl GCiphersRsPolybius {
        fn call_p<T>(&self, action: T)
            where T: Fn(&GCiphersRsWindow, &str, &str, &str) -> Option<String>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text();
            let rows = self.rows.get().text().to_string();
            let cols = self.columns.get().text().to_string();
            let result = action(window, &text, &rows, &cols);
            if let Some(result) = result {
                self.text_view.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, rows, cols| {
                match encrypt(&window.mask_text(text), rows, cols) {
                    Ok(res) => Some(res),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        None
                    }
                }
            })
        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, rows, cols| {
                match decrypt(text, rows, cols) {
                    Ok(res) => Some(window.demask_text(&res)),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        None
                    }
                }
            })
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsPolybius(ObjectSubclass<imp::GCiphersRsPolybius>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsPolybius {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
