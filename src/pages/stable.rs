/* stable.rs
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

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::magma::{t, t_reverse};
    use encryption::methods::{bytes_to_hex, bytes_to_string, hex_to_bytes, str_to_bytes};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/stable.ui")]
    pub struct GCiphersRsStable {
        #[template_child]
        pub text_view: TemplateChild<UITextView>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsStable {
        const NAME: &'static str = "GCiphersRsStable";
        type Type = super::GCiphersRsStable;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsStable {}
    impl WidgetImpl for GCiphersRsStable {}
    impl BinImpl for GCiphersRsStable {}

    #[template_callbacks]
    impl GCiphersRsStable {
        fn call_p<T>(&self, action: T)
            where T: FnOnce(&GCiphersRsWindow, &str) -> Option<String>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text();
            let result = action(window, &text);
            if let Some(result) = result {
                self.text_view.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text| {
                let mut result = String::new();
                let text_r = if window.get_prettify_state() {
                    str_to_bytes(text, 4)
                } else {
                    hex_to_bytes(text, 4)
                };
                let text = match text_r {
                    Ok(text) => text,
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return None;
                    }
                };
                for text_slice in text.windows(4).step_by(4) {
                    result.push_str(&bytes_to_hex(&t(text_slice)));
                }
                Some(result)
            })
        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {
            self.call_p(|window, text| {
                let text = match hex_to_bytes(text, 4) {
                    Ok(text) => text,
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return None;
                    }
                };
                let mut buffer = Vec::new();
                for text_slice in text.windows(4).step_by(4) {
                    buffer.extend(t_reverse(text_slice).into_iter())
                }
                if window.get_prettify_state() {
                    Some(match bytes_to_string(&buffer) {
                        Ok(text) => text,
                        Err(e) => {
                            window.show_message(&e.to_string());
                            return None;
                        }
                    })
                } else { Some(bytes_to_hex(&buffer)) }
            })
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsStable(ObjectSubclass<imp::GCiphersRsStable>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsStable {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
