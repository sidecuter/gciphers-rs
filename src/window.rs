/* window.rs
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

use adw::Bin;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use crate::menu_entry::GCiphersMenuEntry;
use crate::pages::atbash::GCiphersRsAtbash;
use crate::pages::caesar::GCiphersRsCaesar;
use crate::pages::polybius::GCiphersRsPolybius;
use crate::pages::trithemium::GCiphersRsTrithemium;
use crate::pages::belazo::GCiphersRsBelazo;
use crate::pages::cardano::GCiphersRsCardano;
use crate::pages::feistel::GCiphersRsFeistel;
use crate::pages::matrix::GCiphersRsMatrix;
use crate::pages::playfair::GCiphersRsPlayfair;
use crate::pages::shenon::GCiphersRsShenon;
use crate::pages::stable::GCiphersRsStable;
use crate::pages::vetrical::GCiphersRsVertical;
use crate::pages::vigenere::GCiphersRsVigenere;

mod imp {
    use std::cell::RefCell;
    use gtk::{template_callbacks, ToggleButton};
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/window.ui")]
    pub struct GCiphersRsWindow {
        pub labels: RefCell<Option<Vec<String>>>,
        pub pages: RefCell<Option<gio::ListStore>>,
        #[template_child]
        pub toast: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,
        #[template_child]
        pub stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub list_rows: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub prettify: TemplateChild<gtk::Switch>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsWindow {
        const NAME: &'static str = "GCiphersRsWindow";
        type Type = super::GCiphersRsWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_labels();
            obj.setup_pages();
            obj.setup_stack();
            self.list_rows.get().select_row(self.list_rows.get().row_at_index(0).as_ref());
        }
    }

    impl WidgetImpl for GCiphersRsWindow {}
    impl WindowImpl for GCiphersRsWindow {}
    impl ApplicationWindowImpl for GCiphersRsWindow {}
    impl AdwApplicationWindowImpl for GCiphersRsWindow {}

    #[template_callbacks]
    impl GCiphersRsWindow {
        #[template_callback]
        fn on_row_selected(&self, row: Option<&gtk::ListBoxRow>) {
            if let Some(row) = row {
                let obj = self.obj();
                let index = row.index();
                let label_text = obj.imp().labels.borrow().as_ref().expect("Ну так получилось")
                    .get(index as usize)
                    .expect("Invalid index").clone();
                obj.set_title(Some(&label_text));
                let page = obj.pages().item(index as u32)
                    .expect("Index error")
                    .downcast_ref::<Bin>()
                    .expect("Must be Adw.Bin").clone();
                self.stack.set_visible_child_name(&page.widget_name().to_string());
            }
        }

        #[template_callback]
        fn on_sidebar_button_toggle(&self, _button: &ToggleButton) {
            self.split_view.set_show_sidebar(!self.split_view.shows_sidebar());
        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsWindow(ObjectSubclass<imp::GCiphersRsWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl GCiphersRsWindow {
    pub fn new<P>(application: &P) -> Self
        where P: IsA<gtk::Application>
    {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn pages(&self) -> gio::ListStore {
        self.imp().pages.borrow().clone().expect("Could not get pages")
    }

    fn setup_pages(&self) {
        let pages = gio::ListStore::new::<Bin>();
        pages.append(&GCiphersRsAtbash::new());
        pages.append(&GCiphersRsCaesar::new());
        pages.append(&GCiphersRsPolybius::new());
        pages.append(&GCiphersRsTrithemium::new());
        pages.append(&GCiphersRsBelazo::new());
        pages.append(&GCiphersRsVigenere::new());
        pages.append(&GCiphersRsStable::new());
        pages.append(&GCiphersRsMatrix::new());
        pages.append(&GCiphersRsPlayfair::new());
        pages.append(&GCiphersRsVertical::new());
        pages.append(&GCiphersRsCardano::new());
        pages.append(&GCiphersRsFeistel::new());
        pages.append(&GCiphersRsShenon::new());
        self.imp().pages.replace(Some(pages));
    }

    fn setup_labels(&self) {
        let labels = vec![
            String::from("Атбаш"),
            String::from("Цезарь"),
            String::from("Полибий"),
            String::from("Тритемий"),
            String::from("Белазо"),
            String::from("Виженер"),
            String::from("S таблица"),
            String::from("Матричный"),
            String::from("Плейфер"),
            String::from("Вертикальный"),
            String::from("Кардано"),
            String::from("Сеть Фейстеля"),
            String::from("Шеннон"),
        ];
        self.imp().labels.replace(Some(labels));
        self.setup_rows();
    }

    fn setup_rows(&self) {
        let imp = self.imp();
        for label in self.imp().labels.borrow().as_ref().expect("Пусто") {
            imp.list_rows.append(&GCiphersMenuEntry::new(&label.clone()));
        }
    }

    fn setup_stack(&self) {
        let pages = self.pages();
        for page in pages.into_iter() {
            let page = page.expect("Привет")
                .downcast_ref::<Bin>()
                .expect("Needs to be an Adw.Bin")
                .clone();
            let _name = page.widget_name().to_string();
            self.imp().stack.add_named(&page, Some(&page.widget_name().to_string()));
        }
    }

    pub fn get_prettify_state(&self) -> bool {
        self.imp().prettify.state()
    }

    pub fn mask_text(&self, text: &str) -> String {
        let mut result = String::from(text);
        result = result.replace(".", "тчк")
            .replace(",", "зпт")
            .replace("-", "тире");
        if !self.get_prettify_state() {
            result.replace(" ", "")
        } else {
            result.replace(" ", "прб")
        }
    }

    pub fn demask_text(&self, text: &str) -> String {
        let result = String::from(text);
        if self.get_prettify_state() {
            result.replace("тчк", ".")
                .replace("зпт", ",")
                .replace("тире", "-")
                .replace("прб", " ")
        } else {
            result
        }
    }

    pub fn show_message(&self, message: &str) {
        let toast_message = adw::Toast::new(message);
        toast_message.set_timeout(3);
        self.imp().toast.get().add_toast(toast_message);
    }
}
