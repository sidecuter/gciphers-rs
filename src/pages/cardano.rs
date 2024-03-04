/* cardano.rs
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
    use adw::prelude::BinExt;
    use gtk::{Button, template_callbacks};
    use gtk::prelude::WidgetExt;
    use crate::ui::entry::UIEntry;

    use crate::ui::text_view::UITextView;
    use crate::window::GCiphersRsWindow;

    use encryption::cardano::*;
    use crate::ui::cardano_grid::UICardanoGrid;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/cardano.ui")]
    pub struct GCiphersRsCardano {
        #[template_child]
        pub text_view: TemplateChild<UITextView>,
        #[template_child]
        pub placeholder: TemplateChild<adw::Bin>,
        #[template_child]
        pub rows: TemplateChild<UIEntry>,
        #[template_child]
        pub cols: TemplateChild<UIEntry>,
        pub initialized_grid: Cell<bool>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsCardano {
        const NAME: &'static str = "GCiphersRsCardano";
        type Type = super::GCiphersRsCardano;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsCardano {}
    impl WidgetImpl for GCiphersRsCardano {}
    impl BinImpl for GCiphersRsCardano {}

    #[template_callbacks]
    impl GCiphersRsCardano {
        fn call_p<T>(&self, action: T)
            where T: Fn(&GCiphersRsWindow, &str, (Vec<bool>, usize, usize)) -> Option<String>
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
            };
            let grid = _binding
                .downcast_ref::<UICardanoGrid>()
                .expect("Не является подклассом UIMatrixGrid");
            let text = self.text_view.get().get_text().to_lowercase();
            let keys = grid.get_elements();
            let result = action(window, &text, keys);
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
            if !self.initialized_grid.get() {
                self.placeholder.get().set_child(Some(&UICardanoGrid::new()));
                self.initialized_grid.replace(true);
            }
            let rows: usize = match self.rows.get().text().parse() {
                Ok(n) => n,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            let cols: usize = match self.cols.get().text().parse() {
                Ok(n) => n,
                Err(e) => {
                    window.show_message(&e.to_string());
                    return;
                }
            };
            if !(2..=10).contains(&rows) {
                window.show_message("Количество рядов не может быть меньше 2 и больше 10");
                return;
            }
            if !(2..=10).contains(&cols) {
                window.show_message("Количество столбцов не может быть меньше 2 и больше 10");
                return;
            }
            self.placeholder.get().child()
                .expect("Потомок не задан")
                .downcast_ref::<UICardanoGrid>()
                .expect("Не является подклассом UIMatrixGrid")
                .set_collection(rows, cols);
        }

        fn get_encrypted_text(
            &self, window: &GCiphersRsWindow,text: &str, grid: Vec<bool>, rows: usize, cols: usize
        ) -> Option<String> {
            match encrypt(text, grid, rows, cols, vec![true, false, true]) {
                Ok(res) => Some(res),
                Err(e) => {
                    window.show_message(&e.to_string());
                    None
                }
            }
        }

        fn get_decrypted_text(
            &self, window: &GCiphersRsWindow,text: &str, grid: Vec<bool>, rows: usize, cols: usize
        ) -> Option<String> {
            match decrypt(text, grid, rows, cols, vec![true, false, true]) {
                Ok(res) => Some(res),
                Err(e) => {
                    window.show_message(&e.to_string());
                    None
                }
            }
        }

        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, (grid, rows, cols)| {
                let mut text = window.mask_text(text);
                let mut result = String::new();
                if !window.get_prettify_state() {
                    if text.chars().count() == 42 {
                        text.push_str("пожпскдсвгасярдсаю");
                    }
                    result.push_str(&self.get_encrypted_text(window, &text, grid, rows, cols)?);
                } else {
                    let mut chars: Vec<char> = text.chars().collect();
                    if chars.len() % rows * cols != 0 {
                        chars.extend(vec!['\u{0444}'; rows * cols - chars.len() % (rows * cols)]);
                    }
                    for part in chars.windows(rows*cols).step_by(rows*cols) {
                        let part: String = part.iter().copied().collect();
                        result.push_str(&self.get_encrypted_text(
                            window, &part, grid.clone(), rows, cols)?
                        );
                    }
                }
                Some(result)
            })
        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {
            self.call_p(|window, text, (grid, rows, cols)| {
                let text = String::from(text);
                let mut result = String::new();
                if !window.get_prettify_state() {
                    result.push_str(&self.get_decrypted_text(window, &text, grid, rows, cols)?);
                } else {
                    let chars: Vec<char> = text.chars().collect();
                    for part in chars.windows(rows*cols).step_by(rows*cols) {
                        let part: String = part.iter().copied().collect();
                        result.push_str(&self.get_decrypted_text(
                            window, &part, grid.clone(), rows, cols)?
                        );
                    }

                }
                Some(window.demask_text(&result))
            })
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsCardano(ObjectSubclass<imp::GCiphersRsCardano>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsCardano {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
