/* a5_2.rs
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
    use std::error::Error;
    use gtk::{Button, template_callbacks};
    use gtk::prelude::WidgetExt;
    use crate::ui::entry::UIEntry;

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::a5_2::*;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/a5_2.ui")]
    pub struct GCiphersRsA52 {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub key: TemplateChild<UIEntry>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsA52 {
        const NAME: &'static str = "GCiphersRsA52";
        type Type = super::GCiphersRsA52;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsA52 {}
    impl WidgetImpl for GCiphersRsA52 {}
    impl BinImpl for GCiphersRsA52 {}

    #[template_callbacks]
    impl GCiphersRsA52 {
        fn call_p<T>(&self, action: T)
            where T: Fn(&str, &str) -> Result<String, Box<dyn Error>>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text();
            let key = self.key.get().text().to_string();
            let result = match action(&text, &key) {
                Ok(res) => Some(res),
                Err(e) => {
                    window.show_message(&e.to_string());
                    None
                }
            };
            if let Some(result) = result {
                self.text_view.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(encrypt)
        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {
            self.call_p(decrypt)
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsA52(ObjectSubclass<imp::GCiphersRsA52>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsA52 {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
