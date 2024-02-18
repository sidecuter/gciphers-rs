/* matrix_grid.rs
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

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;

mod imp {
    use std::cell::{Cell, RefCell};
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/cardano_grid.ui")]
    pub struct UICardanoGrid {
        pub elements: RefCell<Vec<gtk::ToggleButton>>,
        pub rows: Cell<usize>,
        pub cols: Cell<usize>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UICardanoGrid {
        const NAME: &'static str = "UICardanoGrid";
        type Type = super::UICardanoGrid;
        type ParentType = gtk::Grid;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UICardanoGrid {}
    impl WidgetImpl for UICardanoGrid {}
    impl GridImpl for UICardanoGrid {}
}

glib::wrapper! {
    pub struct UICardanoGrid(ObjectSubclass<imp::UICardanoGrid>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl UICardanoGrid {
    pub fn set_collection(&self, rows: usize, cols: usize) {
        self.imp().rows.replace(rows);
        self.imp().cols.replace(cols);
        self.imp().elements.replace(Vec::new());
        let mut elements = self.imp().elements.borrow_mut();
        for i in 0..rows {
            for j in 0..cols {
                elements.push(gtk::ToggleButton::new());
                self.attach(elements.get(i*cols+j).unwrap(), j as i32, i as i32, 1, 1);
            }
        }
        if rows == 6 && cols == 10 {
            let grid = vec![
                false, true, false, false, false, false, false, false, false, false,
                true, false, false, false, true, false, true, true, false, false,
                false, true, false, false, false, true, false, false, false, true,
                false, false, false, true, false, false, false, true, false, false,
                false, true, false, false, false, false, false, false, false, false,
                false, false, true, false, false, true, true, false, false, true
            ];
            for (elem, state) in elements.iter().zip(grid.into_iter()) {
                elem.set_active(state);
            }
        }
    }

    pub fn get_elements(&self) -> (Vec<bool>, usize, usize) {
        let elements = self.imp().elements.borrow();
        let mut result = Vec::new();
        for element in elements.iter() {
            result.push(element.is_active())
        }
        (result, self.imp().rows.get(), self.imp().cols.get())
    }

    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
