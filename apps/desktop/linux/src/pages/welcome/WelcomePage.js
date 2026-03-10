import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

import { AboutDialog } from '../../dialogs/AboutDialog.js';

export const WelcomePage = GObject.registerClass({
    GTypeName: 'YdxWelcomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/WelcomePage.ui',
    InternalChildren: ['IdAbout'],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.WelcomePage;
        this.#actionsConnect();
    }

    vfunc_realize() {
		super.vfunc_realize();
		this.#window = this.get_native();
        this.#checkAuth();
	}

    #checkAuth() {
        this.utils.secret.get(this.utils.constants.SecretKeys.AccessToken).then(token => {
            if (!token) return;
            const response = this.utils.core.request_signcheck(token);
            if (JSON.parse(response).code === 200) {
                this.#window.navigation().push(this.utils.constants.Pages.HomePage);
            }
        }).catch(e => this.utils.log.error('Auth check failed:', e));
    }

    #actionsConnect() {
        this._IdAbout.connect('clicked', () => {
            new AboutDialog().present(this);
        });
    }
});
