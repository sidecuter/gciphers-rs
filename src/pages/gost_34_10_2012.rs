/* gost_34_10_2012.rs
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

    use encryption::gost_r_34_10_2012::*;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/gost_34_10_2012.ui")]
    pub struct GCiphersRsGOST2012 {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub a: TemplateChild<UIEntry>,
        #[template_child]
        pub b: TemplateChild<UIEntry>,
        #[template_child]
        pub p: TemplateChild<UIEntry>,
        #[template_child]
        pub cb: TemplateChild<UIEntry>,
        #[template_child]
        pub q: TemplateChild<UIEntry>,
        #[template_child]
        pub gx: TemplateChild<UIEntry>,
        #[template_child]
        pub gy: TemplateChild<UIEntry>,
        #[template_child]
        pub dbx: TemplateChild<UIEntry>,
        #[template_child]
        pub dby: TemplateChild<UIEntry>,
        #[template_child]
        pub sign: TemplateChild<UIEntry>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsGOST2012 {
        const NAME: &'static str = "GCiphersRsGOST2012";
        type Type = super::GCiphersRsGOST2012;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsGOST2012 {}
    impl WidgetImpl for GCiphersRsGOST2012 {}
    impl BinImpl for GCiphersRsGOST2012 {}

    #[template_callbacks]
    impl GCiphersRsGOST2012 {
        #[template_callback]
        fn on_gen_click(&self, _button: &Button) {
            let (g, q, cb, db) = get_keys();
            self.a.get().set_text(&g.a.to_string());
            self.b.get().set_text(&g.b.to_string());
            self.p.get().set_text(&g.modula.to_string());
            self.q.get().set_text(&q.to_string());
            self.cb.get().set_text(&cb.to_string());
            let (gx, gy) = g.get_x_y();
            let (dbx, dby) = db.get_x_y();
            self.gx.get().set_text(&gx.to_string());
            self.gy.get().set_text(&gy.to_string());
            self.dbx.get().set_text(&dbx.to_string());
            self.dby.get().set_text(&dby.to_string());
        }

        fn call_p<T>(&self, action: T)
            where T: Fn(&GCiphersRsWindow, &str, Point, Point, usize, usize, &str) -> Option<String>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let text = self.text_view.get().get_text().to_lowercase();
            let p = self.p.get().text().to_string().parse::<isize>();
            let a = self.a.get().text().to_string().parse::<isize>();
            let b = self.b.get().text().to_string().parse::<isize>();
            let dbx = self.dbx.get().text().to_string().parse::<isize>();
            let dby = self.dby.get().text().to_string().parse::<isize>();
            let gx = self.gx.get().text().to_string().parse::<isize>();
            let gy = self.gy.get().text().to_string().parse::<isize>();
            let cb = self.cb.get().text().to_string().parse::<isize>();
            let q = self.q.get().text().to_string().parse::<isize>();
            let sign = self.sign.get().text().to_string();
            let mut args = Vec::new();
            for elem in [p, a, b, dbx, dby, gx, gy, cb, q].iter() {
                match elem {
                    Ok(elem) => args.push(*elem),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        return;
                    }
                }
            }
            let [p, a, b, dbx, dby, gx, gy, cb, q] = args[..]
                else { panic!("Неожиданное поведение") };
            let g = Point::new(
                a, b, gx as usize,
                gy as usize, p as usize);
            let db = Point::new(
                a, b, dbx as usize,
                dby as usize, p as usize);
            let result = action(window, &text, db, g, q as usize, cb as usize, &sign);
            if let Some(result) = result {
                self.sign.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_sing_click(&self, _button: &Button) {
            self.call_p(|window, text, _, g, q, cb, _| {
                match sign(&window.mask_text(text), cb, g, q, 29) {
                    Ok(res) => Some(res),
                    Err(e) => {
                        window.show_message(&e.to_string());
                        None
                    }
                }
            })
        }

        #[template_callback]
        fn on_check_sign_click(&self, _button: &Button) {
            self.call_p(|window, text, db, g, q, _, sign| {
                match check_sign(&window.mask_text(text), db, g, q, sign,29) {
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
    pub struct GCiphersRsGOST2012(ObjectSubclass<imp::GCiphersRsGOST2012>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsGOST2012 {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
