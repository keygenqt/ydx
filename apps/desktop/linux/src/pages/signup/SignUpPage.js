import GObject from 'gi://GObject';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';

export const SignUpPage = GObject.registerClass({
    GTypeName: 'YdxSignUpPage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/signup/SignUpPage.ui',
    InternalChildren: [
        'IdEmailEntry',
        'IdPasswordEntry',
        'IdSubmitButton',
        'IdSignInLink'
    ],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.SignUp;
        this.#setupActions();
        this.#setupChangeValidation();
    }

    vfunc_realize() {
        super.vfunc_realize();
        this.#window = this.get_native();
    }

    vfunc_map() {
        super.vfunc_map();
        // clear form after start
        this.fieldClearFull('IdEmailEntry', 'IdPasswordEntry');
        // enable button
        this._IdSubmitButton.sensitive = true;
        // set focus to submit button
        GLib.idle_add(GLib.PRIORITY_DEFAULT, () => {
            this._IdSubmitButton.grab_focus();
            return GLib.SOURCE_REMOVE;
        });
    }

    #setupActions() {
        this._IdSubmitButton.connect('clicked', () => {
            this.#handleSubmit();
        });
        this._IdEmailEntry.connect('activate', () => {
            this.#handleSubmit();
        });
        this._IdPasswordEntry.connect('activate', () => {
            this.#handleSubmit();
        });
        this._IdSignInLink.connect('clicked', () => {
            this.#window.navigation().replace(this.utils.constants.Pages.SignIn);
        });
    }

    #setupChangeValidation() {
        this.fieldChangedValidate('IdEmailEntry', this.#isValidEmail);
        this.fieldChangedValidate('IdPasswordEntry', this.#isValidPassword);
    }

    #isValidEmail(email) {
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
    }

    #isValidPassword(password) {
        if (password.length < 8 || password.length > 16) {
            return false;
        }
        return /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d).{8,16}$/.test(password);
    }

    #handleSubmit() {
        // block button
        this._IdSubmitButton.sensitive = false;
        // clear validate from fields
        this.fieldClearValidate('IdEmailEntry', 'IdPasswordEntry');
        // get value
        const email = this._IdEmailEntry.get_text().trim();
        const password = this._IdPasswordEntry.get_text();
        // check is not empty
        if (!email && !password) {
            this.showToastError(_('Please fill in all fields.'), 'IdEmailEntry', 'IdPasswordEntry');
            return;
        }
        // check email
        if (!this.#isValidEmail(email)) {
            this.showToastError(_('Please enter a valid email address.'), 'IdEmailEntry');
            return;
        }
        // check password
        if (!this.#isValidPassword(password)) {
            this.showToastError(_('Password needs uppercase, lowercase and number.'), 'IdPasswordEntry');
            return;
        }
        // send query
        GLib.timeout_add(GLib.PRIORITY_DEFAULT, 50, () => {
            const response = JSON.parse(this.utils.core.request_signup(email, password));
            if (response.code === 200) {
                // save secure data
                this.utils.secret.set(this.utils.constants.SecretKeys.UserID, response.data.user_id);
                this.utils.secret.set(this.utils.constants.SecretKeys.AccessToken, response.data.access_token);
                this.utils.secret.set(this.utils.constants.SecretKeys.RefreshToken, response.data.refresh_token);
                // change navigation stack for user
                this.#window.navigation().switchNavigation(true);
            } else {
                // show error
                this.handleServerError(response, {
                    'email': 'IdEmailEntry',
                    'password': 'IdPasswordEntry'
                }, (obj) => {
                    if (obj.code === 409) {
                        this.showToastError(obj.message, 'IdEmailEntry');
                        return true;
                    }
                    return false;
                })
            }
            return GLib.SOURCE_REMOVE;
        });
    }
});
