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
    use gtk::{Button, template_callbacks};
    use gtk::prelude::WidgetExt;
    use crate::ui::entry::UIEntry;

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::egsa::*;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/egsa.ui")]
    pub struct GCiphersRsEgsa {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub p: TemplateChild<UIEntry>,
        #[template_child]
        pub g: TemplateChild<UIEntry>,
        #[template_child]
        pub y: TemplateChild<UIEntry>,
        #[template_child]
        pub x: TemplateChild<UIEntry>,
        #[template_child]
        pub modula: TemplateChild<UIEntry>,
        #[template_child]
        pub sign_val: TemplateChild<UIEntry>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsEgsa {
        const NAME: &'static str = "GCiphersRsEgsa";
        type Type = super::GCiphersRsEgsa;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsEgsa {}
    impl WidgetImpl for GCiphersRsEgsa {}
    impl BinImpl for GCiphersRsEgsa {}

    #[template_callbacks]
    impl GCiphersRsEgsa {
        fn call_p<T>(&self, action: T)
            where T: Fn(&GCiphersRsWindow, &str, usize, usize, usize, usize, usize, (usize, usize)) -> Option<String>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text().to_lowercase();
            let p = self.p.get().text().to_string().parse::<usize>();
            let x = self.x.get().text().to_string().parse::<usize>();
            let g = self.g.get().text().to_string().parse::<usize>();
            let y = self.y.get().text().to_string().parse::<usize>();
            let modula = self.modula.get().text().to_string().parse::<usize>();
            let s = if self.sign_val.get().text().is_empty() {
                (Ok(0), Ok(0))
            } else {
                let [ref a, ref b] = self.sign_val.get().text().to_string().split(',').map(|x| x.parse::<usize>()).collect::<Vec<Result<usize, _>>>()[..] else {
                    window.show_message("ЭЦП введена неверно");
                    return;
                };
                (a.clone(), b.clone())
            };
            let mut args = Vec::new();
            for elem in [p, x, g, y, modula, s.0, s.1].iter() {
                match elem {
                    Ok(elem) => args.push(*elem),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return;
                    }
                }
            }
            let result = action(window, &text, args[0], args[1], args[2], args[3], args[4], (args[5], args[6]));
            if let Some(result) = result {
                self.sign_val.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_gen_click(&self, _button: &Button) {
            let (p, x, g, y) = gen_keys();
            self.p.get().set_text(&p.to_string());
            self.x.get().set_text(&x.to_string());
            self.g.get().set_text(&g.to_string());
            self.y.get().set_text(&y.to_string());
        }

        #[template_callback]
        fn on_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, p, x, g, _, modula, _| {
                match sign(&window.mask_text(text), p, g, x, modula, if window.get_prettify_state() { None } else { Some(5) }) {
                    Ok(res) => Some(format!("{},{}", res.0, res.1)),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        None
                    }
                }
            })
        }

        #[template_callback]
        fn on_check_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, p, _, g, y, modula, s| {
                match check_sign(&window.mask_text(text), p, g, y, modula, s) {
                    Ok(true) => {
                        window.show_message("Подпись верна");
                        None
                    },
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
    pub struct GCiphersRsEgsa(ObjectSubclass<imp::GCiphersRsEgsa>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsEgsa {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
