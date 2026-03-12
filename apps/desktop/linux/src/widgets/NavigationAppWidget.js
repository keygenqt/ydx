import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';

import { NavigationGuestWidget } from './NavigationGuestWidget.js';
import { NavigationUserWidget } from './NavigationUserWidget.js';

export const NavigationAppWidget = GObject.registerClass({
    GTypeName: 'YdxNavigationAppWidget',
    Template: 'resource:///com/keygenqt/ydx/ui/widgets/NavigationAppWidget.ui',
    InternalChildren: [
        'IdStack',
    ],
}, class extends Gtk.Box {
    #guestNav = null;
    #userNav = null;
    #current = null;

    constructor(params) {
        super(params);
        this.#init();
    }

    #init() {
        this.#userNav = new NavigationUserWidget();
        this.#guestNav = new NavigationGuestWidget();
        this._IdStack.add_named(this.#userNav, 'user');
        this._IdStack.add_named(this.#guestNav, 'guest');
        this._IdStack.set_visible_child_name('guest');
        this.#current = this.#guestNav;
    }

    /**
     * Switch between guest and user navigation
     *
     * @param {boolean} isAuth - true for user, false for guest
     */
    switchNavigation(isAuth) {
        const target = isAuth ? this.#userNav : this.#guestNav;
        const targetName = isAuth ? 'user' : 'guest';

        if (this.#current !== target) {
            this._IdStack.set_visible_child_name(targetName);
            this.#current = target;
        }
    }

    /**
     * Display a temporary notification message to the user
     * Toast appears at the bottom of the window and auto-hides after timeout
     *
     * @param {string} message - The text to display in the toast
     * @param {number} timeout - Duration in seconds before auto-hide (default: 3)
     */
    showToast(message, timeout = 3) {
        this.#current.showToast(message, timeout);
    }

    /**
     * Get parameters passed to a specific page
     * Parameters are set when pushing/replacing a page and can be retrieved by the page itself
     *
     * @param {string} key - The page tag to get parameters for
     * @returns {Object} Parameters object (empty object if none)
     */
    params(key) {
        return this.#current.params(key);
    }

    /**
     * Get and remove one-time data passed back from a previous page
     * Used when popping a page to return data to the previous page
     * The data is automatically deleted after first access
     *
     * @param {string} key - The page tag to get disposable data for
     * @returns {*} The disposable data or undefined if not found
     */
    disposable(key) {
        return this.#current.disposable(key);
    }

    /**
     * Push a new page onto the navigation stack
     * The new page becomes visible with animation
     *
     * @param {string} key - The page tag to push (must match registered page)
     * @param {Object} params - Optional parameters to pass to the page
     */
    push(key, params = undefined) {
        this.#current.push(key, params);
    }

    /**
     * Replace the current page with a new one
     * Removes the current page from history and shows the new one
     * Useful for redirects (e.g., after login, replace signin with home)
     *
     * @param {string} key - The page tag to replace with
     * @param {Object} params - Optional parameters for the new page
     */
    replace(key, params = undefined) {
        this.#current.replace(key, params);
    }

    /**
     * Pop (go back) to the previous page
     * Optionally pass disposable data back to the previous page
     *
     * @param {string} key - Page tag to pass data to (usually the previous page)
     * @param {*} disposable - One-time data to pass back
     */
    pop(key = undefined, disposable = undefined) {
        this.#current.pop(key, disposable);
    }
});
