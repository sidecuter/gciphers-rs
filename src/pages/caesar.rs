/* caesar.rs
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

    use encryption::caesar::*;
    use encryption::methods::transform;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/caesar.ui")]
    pub struct GCiphersRsCaesar {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub key: TemplateChild<UIEntry>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsCaesar {
        const NAME: &'static str = "GCiphersRsCaesar";
        type Type = super::GCiphersRsCaesar;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsCaesar {}
    impl WidgetImpl for GCiphersRsCaesar {}
    impl BinImpl for GCiphersRsCaesar {}

    #[template_callbacks]
    impl GCiphersRsCaesar {
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

        fn check_key(&self, window: &GCiphersRsWindow, key: &str) -> Option<isize> {
            let key = match transform(key, "Ключ") {
                Ok(val) => val,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return None;
                }
            };
            if key < 0 {
                window.show_message("Сдвиг не может быть отрицательным");
                return None;
            }
            Some(key)
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, key| {
                match encrypt(&window.mask_text(text), self.check_key(window, key)?) {
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
            self.call_p(|window, text, key| {
                let key = match transform(key, "Ключ") {
                    Ok(val) => val,
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return None;
                    }
                };
                if key < 0 {
                    window.show_message("Сдвиг не может быть отрицательным");
                    return None;
                }
                match decrypt(text, key) {
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
    pub struct GCiphersRsCaesar(ObjectSubclass<imp::GCiphersRsCaesar>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsCaesar {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
