/* main.rs
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

mod application;
mod config;
mod window;
mod pages;
mod ui;
mod menu_entry;

use self::application::GciphersRsApplication;
use self::window::GCiphersRsWindow;

use config::PKGDATADIR;
use gtk::{gio, glib};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/gciphers-rs.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);
    let app = GciphersRsApplication::new("com.github.sidecuter.gciphers_rs", &gio::ApplicationFlags::empty());
    app.run()
}
