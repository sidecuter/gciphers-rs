/* ctr_magma.rs
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

    use encryption::magma::ctr_magma;
    use encryption::methods::{bytes_to_hex, bytes_to_string, hex_to_bytes, str_to_bytes};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/ctr_magma.ui")]
    pub struct GCiphersRsCtrMagma {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub key: TemplateChild<UIEntry>,
        #[template_child]
        pub iv: TemplateChild<UIEntry>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsCtrMagma {
        const NAME: &'static str = "GCiphersRsCtrMagma";
        type Type = super::GCiphersRsCtrMagma;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsCtrMagma {}
    impl WidgetImpl for GCiphersRsCtrMagma {}
    impl BinImpl for GCiphersRsCtrMagma {}

    #[template_callbacks]
    impl GCiphersRsCtrMagma {
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
            let key = self.key.get().text().to_string();
            let iv = self.iv.get().text().to_string();
            let result = action(window, &text, &key, &iv);
            if let Some(result) = result {
                self.text_view.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, key, iv| {
                if key.chars().count() != 64 {
                    window.show_message("Ключ должен состоять из 32 байт");
                    return None;
                }
                let text = if window.get_prettify_state() {
                    bytes_to_hex(&match str_to_bytes(text, 8) {
                        Ok(buffer) => buffer,
                        Err(e) => {
                            window.show_message(&e.to_string());
                            return None;
                        }
                    })
                } else { text.to_owned() };
                Some(match ctr_magma(&text, iv, key) {
                    Ok(result) => result,
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return None;
                    }
                })
            })
        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, key, iv| {
                if key.chars().count() != 64 {
                    window.show_message("Ключ должен состоять из 32 байт");
                    return None;
                }
                let result = match ctr_magma(text, iv, key) {
                    Ok(result) => result,
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return None;
                    }
                };
                let result = if window.get_prettify_state() {
                    match bytes_to_string(&hex_to_bytes(&result, 4).unwrap()) {
                        Ok(result) => result,
                        Err(e) => {
                            window.show_message(&e.to_string());
                            return None;
                        }
                    }
                } else {
                    result
                };
                Some(result)
            })
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsCtrMagma(ObjectSubclass<imp::GCiphersRsCtrMagma>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsCtrMagma {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}