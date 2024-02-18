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

use std::error::Error;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;

mod imp {
    use std::cell::{Cell, RefCell};
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/matrix_grid.ui")]
    pub struct UIMatrixGrid {
        pub elements: RefCell<Vec<gtk::Entry>>,
        pub rows: Cell<usize>,
        pub cols: Cell<usize>

    }

    #[glib::object_subclass]
    impl ObjectSubclass for UIMatrixGrid {
        const NAME: &'static str = "UIMatrixGrid";
        type Type = super::UIMatrixGrid;
        type ParentType = gtk::Grid;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UIMatrixGrid {}
    impl WidgetImpl for UIMatrixGrid {}
    impl GridImpl for UIMatrixGrid {}
}

glib::wrapper! {
    pub struct UIMatrixGrid(ObjectSubclass<imp::UIMatrixGrid>)
        @extends gtk::Grid, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl UIMatrixGrid {
    pub fn set_collection(&self, n: i32) {
        self.imp().rows.replace(n as usize);
        self.imp().cols.replace(n as usize);
        self.imp().elements.replace(Vec::new());
        let mut elements = self.imp().elements.borrow_mut();
        for i in 0..n {
            for j in 0..n {
                elements.push(gtk::Entry::new());
                self.attach(elements.get((i*n+j) as usize).unwrap(), j, i, 1, 1);
            }
        }
    }

    pub fn get_elements(&self) -> Result<Vec<Vec<isize>>, Box<dyn Error>> {
        let elements = self.imp().elements.borrow();
        let mut result = Vec::new();
        let mut iter = elements.iter();
        for _ in 0..self.imp().rows.get() {
            let mut buffer = Vec::new();
            for _ in 0..self.imp().cols.get() {
                if let Some(elem) = iter.next() {
                    buffer.push(elem.text().parse()?);
                } else {
                    Err("Неправильно инициализированный массив")?;
                }
            }
            result.push(buffer);
        }
        Ok(result)
    }

    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
