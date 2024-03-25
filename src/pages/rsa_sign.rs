/* rsa_sign.rs
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
    use crate::ui::entry::UIEntry;
    use gtk::prelude::WidgetExt;
    use gtk::{template_callbacks, Button};

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::rsa_sign::*;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/rsa_sign.ui")]
    pub struct GCiphersRsRSASign {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub q: TemplateChild<UIEntry>,
        #[template_child]
        pub p: TemplateChild<UIEntry>,
        #[template_child]
        pub e: TemplateChild<UIEntry>,
        #[template_child]
        pub n: TemplateChild<UIEntry>,
        #[template_child]
        pub d: TemplateChild<UIEntry>,
        #[template_child]
        pub modula: TemplateChild<UIEntry>,
        #[template_child]
        pub sign_val: TemplateChild<UIEntry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsRSASign {
        const NAME: &'static str = "GCiphersRsRSASign";
        type Type = super::GCiphersRsRSASign;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsRSASign {}
    impl WidgetImpl for GCiphersRsRSASign {}
    impl BinImpl for GCiphersRsRSASign {}

    #[template_callbacks]
    impl GCiphersRsRSASign {
        fn call_p<T>(&self, action: T)
        where
            T: Fn(&GCiphersRsWindow, &str, usize, usize, usize, usize, usize) -> Option<String>,
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text().to_lowercase();
            let n = self.n.get().text().to_string().parse::<usize>();
            let e = self.e.get().text().to_string().parse::<usize>();
            let d = self.d.get().text().to_string().parse::<usize>();
            let modula = self.modula.get().text().to_string().parse::<usize>();
            let s = if self.sign_val.get().text().is_empty() {
                Ok(0)
            } else {
                self.sign_val.get().text().to_string().parse::<usize>()
            };
            let mut args = Vec::new();
            for elem in [e, n, d, modula, s].iter() {
                match elem {
                    Ok(elem) => args.push(*elem),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return;
                    }
                }
            }
            let result = action(window, &text, args[0], args[1], args[2], args[3], args[4]);
            if let Some(result) = result {
                self.sign_val.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_gen_click(&self, _button: &Button) {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let p = self.p.get().text().to_string().parse::<usize>();
            let q = self.q.get().text().to_string().parse::<usize>();
            let p = match p {
                Ok(p) => p,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            let q = match q {
                Ok(p) => p,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            let (e, d, n) = gen_keys(p, q);
            self.n.get().set_text(&n.to_string());
            self.e.get().set_text(&e.to_string());
            self.d.get().set_text(&d.to_string());
        }

        #[template_callback]
        fn on_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, _, n, d, modula, _| {
                match sign(&window.mask_text(text), n, d, modula) {
                    Ok(res) => Some(res.to_string()),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        None
                    }
                }
            })
        }

        #[template_callback]
        fn on_check_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, e, n, _, modula, s| {
                match check_sign(&window.mask_text(text), n, e, s, modula) {
                    Ok(true) => {
                        window.show_message("Подпись верна");
                        None
                    }
                    Ok(false) => {
                        window.show_message("Подпись неверна");
                        None
                    }
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
    pub struct GCiphersRsRSASign(ObjectSubclass<imp::GCiphersRsRSASign>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsRSASign {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
