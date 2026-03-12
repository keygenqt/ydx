import Gio from 'gi://Gio';
import { AppConstants } from '../utils/AppConstants.js';

export const SettingsService = class {
    #settings = new Gio.Settings({ schema: AppConstants.Gtk.schema });

    /**
     * Get a setting value by key
     *
     * @param {string} key - The setting key
     * @returns {string|number|boolean} The setting value
     */
    get(key) {
        const val = this.#settings.get_value(key).recursiveUnpack();
        if (typeof val === 'string') return this.#settings.get_string(key);
        if (typeof val === 'number') return this.#settings.get_int(key);
        if (typeof val === 'boolean') return this.#settings.get_boolean(key);
        return val;
    }

    /**
     * Set a setting value by key
     *
     * @param {string} key - The setting key
     * @param {string|number|boolean} value - The value to set
     * @throws {Error} If value type is not supported
     */
    set(key, value) {
        const type = typeof value;
        if (type === 'string') this.#settings.set_string(key, value);
        else if (type === 'number') this.#settings.set_int(key, value);
        else if (type === 'boolean') this.#settings.set_boolean(key, value);
        else throw new Error(`Unsupported type for ${key}: ${type}`);
        this.#settings.sync();
    }

    /**
     * Reset a setting to its default value
     *
     * @param {string} key - The setting key to reset
     */
    reset(key) {
        this.#settings.reset(key);
    }

    /**
     * Bind a setting to an object property
     *
     * @param {string} key - The setting key
     * @param {GObject.Object} object - Target object
     * @param {string} property - Target property name
     * @param {Gio.SettingsBindFlags} flags - Binding flags
     */
    bind(key, object, property, flags) {
        this.#settings.bind(key, object, property, flags);
    }

    /**
     * Reset all settings to their default values
     */
    resetAll() {
        const keys = Object.values(AppConstants.SettingsKeys);
        for (const key of keys) {
            this.reset(key);
        }
    }
};
