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
Gtk.Widget.prototype.connectWithEmit = function (action, callback = function(value) {}) {
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
