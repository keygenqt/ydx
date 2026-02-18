/* main.js
 *
 * Copyright 2026 keygenqt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * 	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

import GObject from 'gi://GObject';
import Gdk from 'gi://Gdk';
import Gio from 'gi://Gio';
import Gtk from 'gi://Gtk?version=4.0';
import Adw from 'gi://Adw?version=1';

import { YdxWindow } from './window.js';

pkg.initGettext();
pkg.initFormat();

export const YdxApplication = GObject.registerClass(
    class YdxApplication extends Adw.Application {
        constructor() {
            super({
                application_id: 'com.keygenqt.ydx',
                flags: Gio.ApplicationFlags.DEFAULT_FLAGS,
                resource_base_path: '/com/keygenqt/ydx'
            });

            const quit_action = new Gio.SimpleAction({name: 'quit'});
                quit_action.connect('activate', action => {
                this.quit();
            });
            this.add_action(quit_action);
            this.set_accels_for_action('app.quit', ['<control>q']);

            const show_about_action = new Gio.SimpleAction({name: 'about'});
            show_about_action.connect('activate', action => {
                const aboutParams = {
                    application_name: 'demo-rust',
                    application_icon: 'com.keygenqt.ydx',
                    developer_name: 'keygenqt',
                    version: '0.1.0',
                    developers: [
                        'keygenqt'
                    ],
                    translator_credits: _("translator-credits"),
                    copyright: '© 2026 keygenqt'
                };
                const aboutDialog = new Adw.AboutDialog(aboutParams);
                aboutDialog.present(this.active_window);
            });
            this.add_action(show_about_action);
        }

        vfunc_startup() {
            super.vfunc_startup();
            this.#loadStylesheet();
        }

        vfunc_activate() {
            let {active_window} = this;
            if (!active_window)
                active_window = new YdxWindow(this);
            active_window.present();
        }

        #loadStylesheet() {
            const provider = new Gtk.CssProvider();
            provider.load_from_resource('/com/keygenqt/ydx/style.css');
            Gtk.StyleContext.add_provider_for_display(
                Gdk.Display.get_default(),
                provider,
                Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
            );
        }
    }
);

export function main(argv) {
    const application = new YdxApplication();
    return application.runAsync(argv);
}
