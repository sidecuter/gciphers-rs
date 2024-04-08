/* diffi_hellman.rs
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

    use crate::window::GCiphersRsWindow;

    use encryption::diffie_hellman::*;
    use crate::ui::entry::UIEntry;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/diffi_hellman.ui")]
    pub struct GCiphersRsDiffi {
        #[template_child]
        pub ka: TemplateChild<UIEntry>,
        #[template_child]
        pub kb: TemplateChild<UIEntry>,
        #[template_child]
        pub ya: TemplateChild<UIEntry>,
        #[template_child]
        pub yb: TemplateChild<UIEntry>,
        #[template_child]
        pub a: TemplateChild<UIEntry>,
        #[template_child]
        pub n: TemplateChild<UIEntry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsDiffi {
        const NAME: &'static str = "GCiphersRsDiffi";
        type Type = super::GCiphersRsDiffi;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsDiffi {}
    impl WidgetImpl for GCiphersRsDiffi {}
    impl BinImpl for GCiphersRsDiffi {}

    #[template_callbacks]
    impl GCiphersRsDiffi {

        fn get_window(&self) -> GCiphersRsWindow {
            let root = self.obj().root().expect("Не удалось получить окно");
            root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось").clone()
        }
        #[template_callback]
        fn on_key_count_click(&self, _button: &Button) {
            let window = self.get_window();
            let n = self.n.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let a = self.a.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let ka = self.ka.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let (kb, yb) = match gen_keys(a, n) {
                Ok(val) => val,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            let ya = get_y(a, n, ka);
            self.ya.set_text(&ya.to_string());
            self.kb.set_text(&kb.to_string());
            self.yb.set_text(&yb.to_string());
        }

        #[template_callback]
        fn on_key_exchange_click(&self, _button: &Button) {
            let window = self.get_window();
            let ka = self.ka.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let kb = self.kb.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let ya = self.ya.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let yb = self.yb.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let n = self.n.get().text().to_string().parse::<usize>().expect("Нежданчик");
            let k1 = get_k(n, ka, yb);
            let k2 = get_k(n, kb, ya);
            if k1 == k2 {
                window.show_message("Ключи обменены верно");
            } else {
                window.show_message("Ключи обменены неверно");
            }
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsDiffi(ObjectSubclass<imp::GCiphersRsDiffi>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsDiffi {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
