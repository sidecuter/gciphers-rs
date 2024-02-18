/* matrix.rs
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
    use std::cell::Cell;
    use adw::Bin;
    use adw::prelude::BinExt;
    use gtk::{Button, template_callbacks};
    use gtk::prelude::WidgetExt;
    use crate::ui::entry::UIEntry;

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::matrix::*;
    use crate::ui::matrix_grid::UIMatrixGrid;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/matrix.ui")]
    pub struct GCiphersRsMatrix {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub placeholder: TemplateChild<Bin>,
        #[template_child]
        pub n: TemplateChild<UIEntry>,
        pub initialized_matrixes: Cell<bool>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsMatrix {
        const NAME: &'static str = "GCiphersRsMatrix";
        type Type = super::GCiphersRsMatrix;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsMatrix {}
    impl WidgetImpl for GCiphersRsMatrix {}
    impl BinImpl for GCiphersRsMatrix {}

    #[template_callbacks]
    impl GCiphersRsMatrix {
        fn call_p<T>(&self, action: T)
            where T: Fn(&GCiphersRsWindow, &str, Vec<Vec<isize>>) -> Option<String>
        {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            let _binding = match self.placeholder.get().child() {
                Some(child) => child,
                None => {
                    window.show_message("Матрица не указана");
                    return;
                }
            } ;
            let matrix = _binding
                .downcast_ref::<UIMatrixGrid>()
                .expect("Не является подклассом UIMatrixGrid");
            let text = self.text_view.get().get_text().to_lowercase();
            let key = match matrix.get_elements() {
                Ok(key) => key,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            let result = action(window, &text, key);
            if let Some(result) = result {
                self.text_view.get().set_text(&result);
            }
        }

        #[template_callback]
        fn on_get_click(&self, _button: &Button) {
            let root = self.obj().root().expect("Не удалось получить окно");
            let window = root
                .downcast_ref::<gtk::Window>()
                .expect("Приведение не удалось")
                .downcast_ref::<GCiphersRsWindow>()
                .expect("Приведение не удалось");
            if !self.initialized_matrixes.get() {
                self.placeholder.get().set_child(Some(&UIMatrixGrid::new()));
                self.initialized_matrixes.replace(true);
            }
            let n: usize = match self.n.get().text().parse() {
                Ok(n) => n,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            if 2 > n || n > 10 {
                window.show_message("N не может быть меньше 2 и больше 10");
                return;
            }
            self.placeholder.get().child()
                .expect("Потомок не задан")
                .downcast_ref::<UIMatrixGrid>()
                .expect("Не является подклассом UIMatrixGrid")
                .set_collection(n as i32);
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, key| {
                match encrypt(&window.mask_text(text), key) {
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
    pub struct GCiphersRsMatrix(ObjectSubclass<imp::GCiphersRsMatrix>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsMatrix {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
