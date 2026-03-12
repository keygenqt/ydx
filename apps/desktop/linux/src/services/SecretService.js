import Secret from 'gi://Secret';
import { AppConstants } from '../utils/AppConstants.js';

export const SecretService = class {
    #secret = Secret;
    #schema = null;

    constructor() {
        if (!this.#secret) throw new Error('libsecret is required for secure token storage');
        this.#schema = new Secret.Schema(`${AppConstants.Gtk.schema}.Data`, Secret.SchemaFlags.NONE, {
            'app': Secret.SchemaAttributeType.STRING,
            'type': Secret.SchemaAttributeType.STRING,
        });
    }

    /**
     * Retrieve a stored value by key
     *
     * @param {string} key - The key to retrieve (e.g., 'access_token', 'refresh_token')
     * @returns {Promise<string|null>} The stored value or null if not found
     */
    get(key) {
        return new Promise(resolve => {
            this.#secret.password_lookup(
                this.#schema,
                { app: AppConstants.Gtk.schema, type: key },
                null,
                (_, res) => resolve(this.#secret.password_lookup_finish(res))
            );
        });
    }

    /**
     * Store a value by key
     *
     * @param {string} key - The key to store under (e.g., 'access_token', 'refresh_token')
     * @param {string} value - The value to store
     */
    set(key, value) {
        this.#secret.password_store(
            this.#schema,
            { app: AppConstants.Gtk.schema, type: key },
            this.#secret.COLLECTION_DEFAULT,
            `YDX secret data: ${key}`,
            value,
            null,
            () => { }
        );
    }

    /**
     * Remove a stored value by key
     *
     * @param {string} key - The key to remove (e.g., 'access_token', 'refresh_token')
     */
    reset(key) {
        this.set(key, '');
    }

    /**
     * Clear all stored secrets
     * Removes all tokens from AppConstants.SecretKeys
     * Useful for logout or account switching
     */
    clearAll() {
        const keys = Object.values(AppConstants.SecretKeys);
        for (const key of keys) {
            this.reset(key);
        }
    }
};
