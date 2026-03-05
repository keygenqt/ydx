import * as wasm from "../wasm/ydx_wasm.js";

export const HttpClient = {
  /**
   * Initialize HTTP client
   */
  init: function () {
    // @todo - create unique deviceId
    wasm.ydx_wasm_request_init("deviceId");
  },

  /**
   * Checks the authorization status
   * @returns {Promise<boolean>} - true if authorized, otherwise false
   */
  signcheck: async function () {
    try {
      const data = JSON.parse(await wasm.ydx_wasm_request_signcheck());
      return data.code === 200;
    } catch (error) {
      console.error("Signcheck failed:", error);
      return false;
    }
  },

  /**
   * Sign in user
   * @param {string} email - User email
   * @param {string} password - User password
   * @returns {Promise<Object>} - Server response (parsed JSON)
   */
  signin: async function (email, password) {
    const data = JSON.parse(
      await wasm.ydx_wasm_request_signin(email, password),
    );
    if (data.code !== 200) throw data;
    return data;
  },

  /**
   * Sign in user
   * @param {string} email - User email
   * @param {string} password - User password
   * @returns {Promise<Object>} - Server response (parsed JSON)
   */
  signup: async function (email, password) {
    const data = JSON.parse(
      await wasm.ydx_wasm_request_signup(email, password),
    );
    if (data.code !== 200) throw data;
    return data;
  },

  /**
   * Sign out user
   * @returns {Promise<void>}
   */
  signout: async function () {
    try {
      await wasm.ydx_wasm_request_signout();
    } catch (error) {
      console.warn("Signout failed (ignored):", error);
    }
  },
};
