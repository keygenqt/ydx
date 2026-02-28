import { MD5 } from "crypto-js";

export const CacheUtils = {
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
