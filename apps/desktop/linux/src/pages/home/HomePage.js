import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';

const YdxLinux = imports.gi.YdxLinux;

export const HomePage = GObject.registerClass({
    GTypeName: 'HomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/HomePage.ui',
    InternalChildren: ['label', 'IdBtn'],
}, class HomePage extends Adw.ApplicationWindow {

    constructor(application) {
        super({ application });
        this.#actionsConnect();
    }

    #actionsConnect() {
        this._IdBtn.connect('clicked', () => {
            // Run function
            GLib.timeout_add(GLib.PRIORITY_DEFAULT, 50, () => {
                // const value = YdxLinux.performance_test();
                this._IdBtn.label = `The test was performed: 000ms`;
                this._IdBtn.sensitive = true;
                return GLib.SOURCE_REMOVE;
            });
            // Start
            this._IdBtn.label = 'Loading...';
            this._IdBtn.sensitive = false;
        });
    }
});
