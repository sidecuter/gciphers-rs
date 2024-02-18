/* feistel.rs
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

    use encryption::magma::{feistel_net_node, g};
    use encryption::methods::{bytes_to_hex, bytes_to_string, hex_to_bytes, str_to_bytes};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/feistel.ui")]
    pub struct GCiphersRsFeistel {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub key: TemplateChild<UIEntry>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsFeistel {
        const NAME: &'static str = "GCiphersRsFeistel";
        type Type = super::GCiphersRsFeistel;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsFeistel {}
    impl WidgetImpl for GCiphersRsFeistel {}
    impl BinImpl for GCiphersRsFeistel {}

    #[template_callbacks]
    impl GCiphersRsFeistel {
        fn call_p<T>(&self, action: T)
            where T: Fn(&GCiphersRsWindow, &str, &str) -> Option<String>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text();
            let key = self.key.get().text().to_string();
            let result = action(window, &text, &key);
            if let Some(result) = result {
                self.text_view.get().set_text(&result);
            }
        }

        fn get_bytes(&self, window: &GCiphersRsWindow, text: &str) -> Option<Vec<u8>> {
            match hex_to_bytes(text, 4) {
                Ok(key) => Some(key),
                Err(e) => {
                    window.show_message(&e.to_string());
                    return None;
                }
            }
        }

        fn get_bytes_str(&self, window: &GCiphersRsWindow, text: &str) -> Option<Vec<u8>> {
            match str_to_bytes(text, 4) {
                Ok(key) => Some(key),
                Err(e) => {
                    window.show_message(&e.to_string());
                    return None;
                }
            }
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, key| {
                if key.chars().count() != 8 {
                    window.show_message("Ключ должен состоять из 4 байт");
                    return None;
                }
                let key = self.get_bytes(window, key)?;
                let mut result = String::new();
                if window.get_prettify_state() {
                    let right: Vec<u8> = vec![ 0xfd, 0xcb, 0xc2, 0x0c ];
                    let left = self.get_bytes_str(window, text)?;
                    for sector in left.windows(4).step_by(4) {
                        let (_, right_result) = feistel_net_node(sector, &right, &key);
                        result.push_str(&bytes_to_hex(&right_result));
                    }
                } else {
                    let left = self.get_bytes(window, text)?;
                    result.push_str(&bytes_to_hex(&g(&left, &key)));
                }
                Some(result)
            })
        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, key| {
                if key.chars().count() != 8 {
                    window.show_message("Ключ должен состоять из 4 байт");
                    return None;
                }
                let key = self.get_bytes(window, key)?;
                let mut result: Vec<u8> = Vec::new();
                if window.get_prettify_state() {
                    let right: Vec<u8> = vec![ 0xfd, 0xcb, 0xc2, 0x0c ];
                    let left = self.get_bytes(window, text)?;
                    for sector in left.windows(4).step_by(4) {
                        let (_, right_result) = feistel_net_node(sector, &right, &key);
                        result.extend(right_result);
                    }
                } else {
                    return None;
                }
                Some(match bytes_to_string(&result) {
                    Ok(str) => str,
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return None;
                    }
                })
                /*match decrypt(text, key) {
                    Ok(res) => Some(window.demask_text(&res)),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        None
                    }
                }*/
            })
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsFeistel(ObjectSubclass<imp::GCiphersRsFeistel>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsFeistel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
