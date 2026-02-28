export const LanguageUtils = {
  get: function (i18n) {
    if (i18n.language.toLowerCase().includes("ru")) {
      return "ru";
    } else if (i18n.language.toLowerCase().includes("cn")) {
      return "cn";
    }
    return "en";
  },
};
