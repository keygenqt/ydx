import Gtk from 'gi://Gtk';
import Gio from 'gi://Gio';

import { SecretService } from '../services/SecretService.js';
import { SettingsService } from '../services/SettingsService.js';
import { AppConstants } from '../utils/AppConstants.js';
import { AppLog } from '../utils/AppLog.js';

// Core library
const YdxLinux = imports.gi.YdxLinux;

// Init Http client
YdxLinux.request_init(AppConstants.Api.base_url, AppConstants.Api.uuid)

// Quick access
Gtk.Widget.prototype.utils = {
    constants: AppConstants,
    secret: new SecretService(),
    settings: new SettingsService(),
    core: YdxLinux,
    log: AppLog,
};

/**
 * Simple way connect to actions
 *
 * @param {String} group Name group
 * @param {Object} actions Actions object name: callback
 */
Gtk.Widget.prototype.connectGroup = function (group, actions = {}) {
    const simpleActionGroup = new Gio.SimpleActionGroup();
    for (const [name, func] of Object.entries(actions)) {
        const action = new Gio.SimpleAction({ name: name });
        action.connect('activate', func);
        simpleActionGroup.insert(action);
    }
    this.insert_action_group(group, simpleActionGroup);
    return simpleActionGroup;
};

/**
 * Simple way create action
 *
 * @param {String} action Name action
 */
Gtk.Widget.prototype.createAction = function (name, func) {
    const action = new Gio.SimpleAction({ name: name });
    action.connect('activate', func);
    return action;
};

/**
 * Connect with emit value
 *
 * @param {String} action - signal name
 * @param {Function} callback - callback value
 */
Gtk.Widget.prototype.connectWithEmit = function (action, callback = function (value) { }) {
    this.connect(action, (_, value) => {
        callback(value)
    });
    if (this[action] !== undefined) {
        this.emit(action, this[action]);
    }
};

/**
 * Hide list children
 *
 * @param {Array} children - keys children
 */
Gtk.Widget.prototype.childrenHide = function (...children) {
    children.forEach((key) => {
        if (this[`_${key}`]) {
            this[`_${key}`].visible = false;
        }
    })
};

/**
 * Show list children
 *
 * @param {Array} children - keys children
 */
Gtk.Widget.prototype.childrenShow = function (...children) {
    children.forEach((key) => {
        if (this[`_${key}`]) {
            this[`_${key}`].visible = true;
        }
    })
};

/**
 * Connects to the 'changed' signal of a field to validate its content in real-time.
 * Adds or removes an 'error' CSS class based on the validation result.
 *
 * @param {string} key - The key name of the field's internal child widget (used to access it as this[`_${key}`]).
 * @param {Function} isValid - A validation function that takes the field's current text and returns true if valid, false otherwise.
 */
Gtk.Widget.prototype.fieldChangedValidate = function (key, isValid) {
    this[`_${key}`].connect('changed', () => {
        const email = this[`_${key}`].get_text();
        if (email && !isValid(email)) {
            this[`_${key}`].add_css_class('error');
        } else {
            this[`_${key}`].remove_css_class('error');
        }
    });
};

/**
 * Clears the text content and removes the 'error' CSS class from the specified form fields.
 * Useful for resetting form fields to their initial state.
 *
 * @param {...string} children - One or more key names of the internal child widgets (form fields) to clear.
 */
Gtk.Widget.prototype.fieldClearValidate = function (...children) {
    children.forEach((key) => {
        if (this[`_${key}`]) {
            this[`_${key}`].remove_css_class('error');
        }
    })
};

/**
 * Clears the text content and removes the 'error' CSS class from the specified form fields.
 *
 * @param {...string} children - One or more key names of the internal child widgets (form fields) to clear.
 */
Gtk.Widget.prototype.fieldClearFull = function (...children) {
    children.forEach((key) => {
        if (this[`_${key}`]) {
            this[`_${key}`].set_text('');
            this[`_${key}`].remove_css_class('error');
        }
    })
};

/**
 * Displays an error toast, marks specified fields with an error style, and sets focus to the first field.
 *
 * @param {string} message - The error message to display in the toast.
 * @param {...string} children - Field keys to mark as error (first gets focus).
 */
Gtk.Widget.prototype.showToastError = function (message, ...children) {
    this.get_native().navigation().showToast(message);
    if (children) {
        children.forEach((key, index) => {
            if (this[`_${key}`]) {
                this[`_${key}`].add_css_class('error');
                if (index === 0) {
                    this[`_${key}`].grab_focus();
                }
            }
        });
    }
};

/**
 * Processes a server error response, applies error styles to mapped fields, and shows the first error message.
 * Allows a custom handler to override default behavior for specific status codes.
 *
 * @param {Object} obj - The server error response object.
 * @param {Object<string, Gtk.Widget>} fieldMap - Mapping of field names to their UI widgets.
 * @param {Function} [customHandler] - Optional function to handle specific error cases; returns true if handled.
 */
Gtk.Widget.prototype.handleServerError = function (obj, fieldMap, customHandler = null) {
    if (customHandler && customHandler(obj)) {
        return;
    }
    const errors = obj.data?.errors;
    if (!errors) {
        this.showToastError(obj.message || _('Validate form failed'));
        return;
    }
    let firstError = null;
    Object.entries(errors).forEach(([field, fieldErrors]) => {
        const uiField = this[`_${fieldMap[field]}`];
        if (uiField && fieldErrors.length > 0) {
            uiField.add_css_class('error');
        }
        if (!firstError && fieldErrors[0]?.message) {
            firstError = fieldErrors[0].message;
        }
    });
    if (firstError) {
        this.showToastError(firstError);
    }
};
