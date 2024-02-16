/* atbash.rs
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

mod imp {
    use gtk::{Button, template_callbacks};
    use crate::ui::text_view::UITextView;
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/sidecuter/gciphers_rs/atbash.ui")]
    pub struct GCiphersRsAtbash {
        #[template_child]
        pub text_view: TemplateChild<UITextView>
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GCiphersRsAtbash {
        const NAME: &'static str = "GCiphersRsAtbash";
        type Type = super::GCiphersRsAtbash;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GCiphersRsAtbash {}
    impl WidgetImpl for GCiphersRsAtbash {}
    impl BinImpl for GCiphersRsAtbash {}

    #[template_callbacks]
    impl GCiphersRsAtbash {
        #[template_callback]
        fn on_encrypt_click(&self, _button: &Button) {

        }

        #[template_callback]
        fn on_decrypt_click(&self, _button: &Button) {

        }
    }
}

glib::wrapper! {
    pub struct GCiphersRsAtbash(ObjectSubclass<imp::GCiphersRsAtbash>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GCiphersRsAtbash {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
