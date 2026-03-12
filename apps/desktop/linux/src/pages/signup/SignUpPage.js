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
    #errorToast = null

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.SignUp;
        this.#setupActions();
        this.#setupValidation();
    }

    vfunc_realize() {
        super.vfunc_realize();
        this.#window = this.get_native();
    }

    vfunc_map() {
        super.vfunc_map();
        this.#clearFields();
        this.#removeErrorStyles();
        GLib.idle_add(GLib.PRIORITY_DEFAULT, () => {
            this._IdSubmitButton.grab_focus();
            return GLib.SOURCE_REMOVE;
        });
    }

    #clearFields() {
        this._IdEmailEntry.set_text('');
        this._IdPasswordEntry.set_text('');
    }

    #removeErrorStyles() {
        this._IdEmailEntry.remove_css_class('error');
        this._IdPasswordEntry.remove_css_class('error');
    }

    #setupValidation() {
        this._IdEmailEntry.connect('changed', () => {
            const email = this._IdEmailEntry.get_text();
            if (email && !this.#isValidEmail(email)) {
                this._IdEmailEntry.add_css_class('error');
            } else {
                this._IdEmailEntry.remove_css_class('error');
            }
        });
        this._IdPasswordEntry.connect('changed', () => {
            const password = this._IdPasswordEntry.get_text();
            if (password && !this.#isValidPassword(password)) {
                this._IdPasswordEntry.add_css_class('error');
            } else {
                this._IdPasswordEntry.remove_css_class('error');
            }
        });
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

    #showError(message, ...entries) {
        this.#window.navigation().showToast(message);
        if (entries) {
            entries.forEach((entry, index) => {
                if (entry) {
                    entry.add_css_class('error');
                    if (index === 0) {
                        entry.grab_focus();
                    }
                }
            });
        }
    }

    #setupActions() {
        this._IdSubmitButton.connect('clicked', () => {
            this.#handleSignUp();
        });
        this._IdEmailEntry.connect('activate', () => {
            this.#handleSignUp();
        });
        this._IdPasswordEntry.connect('activate', () => {
            this.#handleSignUp();
        });
        this._IdSignInLink.connect('clicked', () => {
            this.#window.navigation().replace(this.utils.constants.Pages.SignIn);
        });
    }

    #handleSignUp() {
        this.#removeErrorStyles();

        const email = this._IdEmailEntry.get_text().trim();
        const password = this._IdPasswordEntry.get_text();

        if (!email && !password) {
            this.#showError(
                _('Please fill in all fields.'),
                this._IdEmailEntry, this._IdPasswordEntry
            );
            return;
        }

        if (!this.#isValidEmail(email)) {
            this.#showError(
                _('Please enter a valid email address.'),
                this._IdEmailEntry
            );
            return;
        }

        if (!this.#isValidPassword(password)) {
            this.#showError(
                _('Password must contain at least one uppercase letter, one lowercase letter, and one number.'),
                this._IdPasswordEntry
            );
            return;
        }

        this._IdSubmitButton.sensitive = false;

        GLib.timeout_add(GLib.PRIORITY_DEFAULT, 50, () => {
            this._IdSubmitButton.sensitive = true;
            const response = JSON.parse(this.utils.core.request_signup(email, password));
            if (response.code === 200) {
                this.utils.secret.set(this.utils.constants.SecretKeys.AccessToken, response.data.access_token);
                this.utils.secret.set(this.utils.constants.SecretKeys.RefreshToken, response.data.refresh_token);
                this.#window.navigation().switchNavigation(true);
            } else {
                this.#handleServerError(response, {
                    'email': this._IdEmailEntry,
                    'password': this._IdPasswordEntry
                }, (obj) => {
                    if (obj.code === 409) {
                        this.#showError(obj.message, this._IdEmailEntry);
                        return true;
                    }
                    return false;
                })
            }
            return GLib.SOURCE_REMOVE;
        });
    }

    #handleServerError(obj, fieldMap, customHandler = null) {
        if (customHandler && customHandler(obj)) {
            return;
        }
        const errors = obj.data?.errors;
        if (!errors) {
            this.#showError(obj.message || _('Registration failed'));
            return;
        }
        let firstError = null;
        Object.entries(errors).forEach(([field, fieldErrors]) => {
            const uiField = fieldMap[field];
            if (uiField && fieldErrors.length > 0) {
                uiField.add_css_class('error');
            }
            if (!firstError && fieldErrors[0]?.message) {
                firstError = fieldErrors[0].message;
            }
        });
        if (firstError) {
            this.#showError(firstError);
        }
    }
});
