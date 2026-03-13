import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';
import Gtk from 'gi://Gtk';

import './elements/WelcomeAuth.js';
import './elements/WelcomeLoading.js';

import { AboutDialog } from '../../dialogs/AboutDialog.js';

export const WelcomePage = GObject.registerClass({
    GTypeName: 'YdxWelcomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/welcome/WelcomePage.ui',
    InternalChildren: [
        'IdWelcomeStack',
        'IdMenuButton',
    ],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.Welcome;
        this.#setupPageActions();
    }

    vfunc_realize() {
        super.vfunc_realize();
        this.#window = this.get_native();
        this.#showLoading();
    }

    vfunc_map() {
        super.vfunc_map();
        this.#checkAuth();
    }

    #setupPageActions() {
        this.connectGroup('welcomePage', {
            'about': () => {
                new AboutDialog().present(this);
            },
        });
    }

    #checkAuth() {
        this.utils.secret.get(this.utils.constants.SecretKeys.AccessToken).then(token => {
            if (token) {
                GLib.timeout_add(GLib.PRIORITY_DEFAULT, 900, () => {
                    const response = JSON.parse(this.utils.core.request_signcheck(token));
                    if (response.code === 200) {
                        // change navigation stack for user
                        this.#window.navigation().switchNavigation(true);
                    } else {
                        this.#showAuth(true);
                    }
                    return GLib.SOURCE_REMOVE;
                });
            } else {
                this.#showAuth(false);
            }
        }).catch(e => this.utils.log.error('Auth check failed:', e));
    }

    #showLoading() {
        this._IdMenuButton.visible = false;
        this._IdWelcomeStack.set_transition_type(Gtk.StackTransitionType.NONE);
        this._IdWelcomeStack.set_visible_child_name('loading');
        this._IdWelcomeStack.set_transition_type(Gtk.StackTransitionType.OVER_LEFT);
    }

    #showAuth(isAnimate) {
        this._IdMenuButton.visible = true;
        if (isAnimate) {
            this._IdWelcomeStack.set_visible_child_name('auth');
        } else {
            this._IdWelcomeStack.set_transition_type(Gtk.StackTransitionType.NONE);
            this._IdWelcomeStack.set_visible_child_name('auth');
            this._IdWelcomeStack.set_transition_type(Gtk.StackTransitionType.OVER_LEFT);
        }
    }
});
