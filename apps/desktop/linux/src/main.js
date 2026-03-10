import GObject from 'gi://GObject';
import Gdk from 'gi://Gdk';
import Gio from 'gi://Gio';
import Gtk from 'gi://Gtk?version=4.0';
import Adw from 'gi://Adw?version=1';

import './widgets/GtkWidget.js';

import { WindowWidget } from './widgets/WindowWidget.js';

pkg.initGettext();
pkg.initFormat();

export const Application = GObject.registerClass(
    class Application extends Adw.Application {
        constructor() {
            super({
                application_id: 'com.keygenqt.ydx',
                flags: Gio.ApplicationFlags.DEFAULT_FLAGS,
                resource_base_path: '/com/keygenqt/ydx'
            });
        }

        vfunc_startup() {
            super.vfunc_startup();
            this.#loadStylesheet();
        }

        vfunc_activate() {
            new WindowWidget({ application: this }).present();
        }

        #loadStylesheet() {
            const provider = new Gtk.CssProvider();
            provider.load_from_resource('/com/keygenqt/ydx/css/style.css');
            Gtk.StyleContext.add_provider_for_display(
                Gdk.Display.get_default(),
                provider,
                Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
            );
        }
    }
);

export function main(argv) {
    return new Application().runAsync(argv);
}
