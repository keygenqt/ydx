import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';
import Adw from 'gi://Adw';
import GLib from 'gi://GLib';

export const NavigationBaseWidget = GObject.registerClass({
    GTypeName: 'YdxNavigationBaseWidget',
}, class NavigationBaseWidget extends Gtk.Box {
    #data = {}
    #disposable = {}

    constructor(params) {
        super(params);
    }

    /**
     * Display a temporary notification message to the user
     * Creates an Adw.Toast widget and adds it to the toast overlay
     * Toast appears at the bottom of the window and auto-hides after timeout
     *
     * @param {string} message - The text to display in the toast
     * @param {number} timeout - Duration in seconds before auto-hide (default: 3)
     */
    showToast(message, timeout = 3) {
        const toast = new Adw.Toast({
            title: message,
            timeout: timeout
        });
        this._IdToastOverlay.add_toast(toast);
    }

    /**
     * Get parameters for a specific page
     *
     * @param {string} key - The page tag to get parameters for
     * @returns {Object} Parameters object (empty object if none)
     */
    params(key) {
        return this.#data[key] ?? {};
    }

    /**
     * Get and remove disposable data for a page
     * Used for one-time data that should be cleared after use
     *
     * @param {string} key - The page tag to get disposable data for
     * @returns {*} The disposable data or undefined if not found
     */
    disposable(key) {
        if (this.#disposable[key]) {
            const value = this.#disposable[key];
            this.#disposable[key] = undefined;
            return value;
        }
        return undefined;
    }

    /**
     * Push a new page onto the navigation stack
     *
     * @param {string} key - The page tag to push (must match registered page)
     * @param {Object} params - Optional parameters to pass to the page
     */
    push(key, params = undefined) {
        this.#data[key] = params;
        this._IdNavigation.push_by_tag(key);
    }

    /**
     * Replace the current page without changing navigation history
     * Useful for redirects or after authentication
     *
     * @param {string} key - The page tag to replace with
     * @param {Object} params - Optional parameters for the new page
     */
    replace(key, params = undefined) {
        this.#data[key] = params;
        const nav = this._IdNavigation;
        nav.push_by_tag(key);
        GLib.timeout_add(GLib.PRIORITY_DEFAULT, 250, () => {
            nav.set_animate_transitions(false);
            nav.pop();
            nav.pop();
            nav.push_by_tag(key);
            nav.set_animate_transitions(true);
            return GLib.SOURCE_REMOVE;
        });
    }

    /**
     * Pop the current page from the navigation stack
     * Returns to the previous page
     *
     * @param {string} key - Optional page tag to store disposable data for
     * @param {*} disposable - Optional data to pass back to the previous page
     */
    pop(key = undefined, disposable = undefined) {
        if (key && disposable) {
            this.#disposable[key] = disposable;
        }
        this._IdNavigation.pop();
    }

    /**
     * Reset the navigation stack to its root page.
     * Pops all pages until only the root remains.
     * The root page is the first page added to the navigation view.
     */
    to_root() {
        const nav = this._IdNavigation;
        GLib.timeout_add(GLib.PRIORITY_DEFAULT, 250, () => {
            nav.set_animate_transitions(false);
            for (let i = 0; i < 50; i++) {
                nav.pop();
            }
            nav.set_animate_transitions(true);
            return GLib.SOURCE_REMOVE;
        });
    }
});
