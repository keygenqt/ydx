import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

import { AboutDialog } from '../../dialogs/AboutDialog.js';

// @todo
const YdxLinux = imports.gi.YdxLinux;

export const WelcomePage = GObject.registerClass({
    GTypeName: 'YdxWelcomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/WelcomePage.ui',
    InternalChildren: ['IdAbout'],
}, class extends Adw.NavigationPage {

    constructor(params) {
        super(params);
        this.#actionsConnect();
    }

    #actionsConnect() {
        this._IdAbout.connect('clicked', () => {
            new AboutDialog().present(this);
        });
    }
});
