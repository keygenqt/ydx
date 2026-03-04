import { MD5 } from "crypto-js";

export const CacheUtils = {
  /**
   * Gets a value from localStorage
   *
   * @param {string} key - Storage key
   * @returns {string|null} - Stored value or null if not exists
   *
   * @example
   * const isLogin = CacheUtils.get(AppCache.isLogin);
   */
  get: function (key) {
    return localStorage.getItem(key);
  },

  /**
   * Sets a localStorage item and updates the cache checksum
   *
   * @param {string} key - Storage key
   * @param {string} value - Value to store
   *
   * @description
   * After setting the item, generates an MD5 hash of all localStorage
   * values and stores it in #root element's data-cache attribute.
   * This hash can be used to monitor cache changes.
   */
  set: function (key, value) {
    localStorage.setItem(key, value);
    let values = [],
      keys = Object.keys(localStorage),
      i = keys.length,
      el = document.querySelector("#root");
    while (i--) {
      values.push(localStorage.getItem(keys[i]));
    }
    el.dataset.cache = MD5(values.toString()).toString();
  },
};
