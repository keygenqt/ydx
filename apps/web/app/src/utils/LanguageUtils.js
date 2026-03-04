export const LanguageUtils = {
  /**
   * Gets the base language code from i18n instance
   * @param {Object} i18n - i18n instance with language property
   * @returns {string} - 'ru' for Russian, 'cn' for Chinese, 'en' for all others
   */
  get: function (i18n) {
    if (i18n.language.toLowerCase().includes("ru")) {
      return "ru";
    } else if (i18n.language.toLowerCase().includes("cn")) {
      return "cn";
    }
    return "en";
  },
};
