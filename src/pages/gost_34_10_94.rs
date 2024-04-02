/* gost_34_10_94.rs
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

    use encryption::gost_r_34_10_94::*;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/gost_34_10_94.ui")]
    pub struct GCiphersRsGOST94 {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub q: TemplateChild<UIEntry>,
        #[template_child]
        pub p: TemplateChild<UIEntry>,
        #[template_child]
        pub a: TemplateChild<UIEntry>,
        #[template_child]
        pub x: TemplateChild<UIEntry>,
        #[template_child]
        pub y: TemplateChild<UIEntry>,
        #[template_child]
        pub modula: TemplateChild<UIEntry>,
        #[template_child]
        pub sign_val: TemplateChild<UIEntry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsGOST94 {
        const NAME: &'static str = "GCiphersRsGOST94";
        type Type = super::GCiphersRsGOST94;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsGOST94 {}
    impl WidgetImpl for GCiphersRsGOST94 {}
    impl BinImpl for GCiphersRsGOST94 {}

    #[template_callbacks]
    impl GCiphersRsGOST94 {
        fn call_p<T>(&self, action: T)
            where
                T: Fn(&GCiphersRsWindow, &str, usize, usize, usize, usize, usize, usize, (u128, u128)) -> Option<String>,
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text().to_lowercase();
            let p = self.p.get().text().to_string().parse::<usize>();
            let q = self.q.get().text().to_string().parse::<usize>();
            let a = self.a.get().text().to_string().parse::<usize>();
            let x = self.x.get().text().to_string().parse::<usize>();
            let y = self.y.get().text().to_string().parse::<usize>();
            let modula = self.modula.get().text().to_string().parse::<usize>();
            let (r, s) = if self.sign_val.get().text().is_empty() {
                (Ok(0), Ok(0))
            } else {
                let s = self.sign_val.get().text().to_string().split(',').map(|x| String::from(x)).collect::<Vec<String>>();
                (s[0].parse::<usize>(), s[1].parse::<usize>())
            };
            let mut args = Vec::new();
            for elem in [p, q, a, x, y, modula, r, s].iter() {
                match elem {
                    Ok(elem) => args.push(*elem),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return;
                    }
                }
            }
            let result = action(window, &text, args[2], args[0], args[1], args[3], args[4], args[5], (args[6] as u128, args[7] as u128));
            if let Some(result) = result {
                self.sign_val.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, a, p, q, x, _y, m, _| {
                let result = sign(&window.mask_text(text), a as u128, p as u128, x as u128, q as u128, m as u128);
                Some(format!("{},{}", result.0, result.1))
            })
        }

        #[template_callback]
        fn on_check_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, a, p, q, _x, y, m, rss| {
                match check_sign(&window.mask_text(text), p as u128, q as u128, a as u128, y as u128, m as u128, rss) {
                    true => {
                        window.show_message("Подпись верна");
                        None
                    }
                    false => {
                        window.show_message("Подпись неверна");
                        None
                    }
                }
            })
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsGOST94(ObjectSubclass<imp::GCiphersRsGOST94>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsGOST94 {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
