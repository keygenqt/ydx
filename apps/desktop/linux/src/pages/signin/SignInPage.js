import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';

export const SignInPage = GObject.registerClass({
    GTypeName: 'YdxSignInPage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/signin/SignInPage.ui',
    InternalChildren: [
        'IdEmailEntry',
        'IdPasswordEntry',
        'IdSubmitButton',
        'IdSignUpLink'
    ],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.SignIn;
        this.#setupActions();
    }

    vfunc_realize() {
        super.vfunc_realize();
        this.#window = this.get_native();
    }

    vfunc_map() {
        super.vfunc_map();
        this.#clearFields();
        GLib.idle_add(GLib.PRIORITY_DEFAULT, () => {
            this._IdSubmitButton.grab_focus();
            return GLib.SOURCE_REMOVE;
        });
    }

    #clearFields() {
        this._IdEmailEntry.set_text('');
        this._IdPasswordEntry.set_text('');
    }

    #setupActions() {
        this._IdSubmitButton.connect('clicked', () => {
            this.#handleSignIn();
        });
        this._IdEmailEntry.connect('activate', () => {
            this.#handleSignIn();
        });
        this._IdPasswordEntry.connect('activate', () => {
            this.#handleSignIn();
        });
        this._IdSignUpLink.connect('clicked', () => {
            this.#window.navigation().replace(this.utils.constants.Pages.SignUp);
        });
    }

    #handleSignIn() {
        const email = this._IdEmailEntry.get_text();
        const password = this._IdPasswordEntry.get_text();

        if (!email || !password) {
            this.#showError('Please fill in all fields');
            return;
        }
        this._IdSubmitButton.sensitive = false;
        this.utils.log.debug('Sign in attempt', { email });
        GLib.timeout_add(GLib.PRIORITY_DEFAULT, 1000, () => {
            this._IdSubmitButton.sensitive = true;
            this.#window.navigation().push(this.utils.constants.Pages.Home);
            return GLib.SOURCE_REMOVE;
        });
    }

    #showError(message) {
        this.utils.log.error(message);
    }
});
