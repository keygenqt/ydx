import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';

import './elements/WelcomeAuth.js';
import './elements/WelcomeLoading.js';

export const WelcomePage = GObject.registerClass({
    GTypeName: 'YdxWelcomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/welcome/WelcomePage.ui',
    InternalChildren: [
        'IdWelcomeStack',
    ],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.Welcome;
    }

    vfunc_realize() {
        super.vfunc_realize();
        this.#window = this.get_native();
        this.#checkAuth();
    }

    #checkAuth() {
        this.utils.secret.get(this.utils.constants.SecretKeys.AccessToken).then(token => {
            GLib.timeout_add(GLib.PRIORITY_DEFAULT, 1000, () => {
                if (token) {
                    const response = JSON.parse(this.utils.core.request_signcheck(token));
                    if (response.code === 200) {
                        this.#window.navigation().switchNavigation(true);
                    } else {
                        this.#showAuth();
                    }
                } else {
                    this.#showAuth();
                }
                return GLib.SOURCE_REMOVE;
            });
        }).catch(e => this.utils.log.error('Auth check failed:', e));
    }

    #showAuth() {
        this._IdWelcomeStack.set_visible_child_name('auth');
    }
});
