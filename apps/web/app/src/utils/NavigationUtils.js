export const NavigationUtils = {
  /**
   * Open root page
   */
  toHome: function (navigate) {
    navigate("/");
  },

  /**
   * Back by route
   */
  back: function (navigate) {
    navigate(-1);
  },

  /**
   * Open page route
   *
   * @param navigate `const navigate = useNavigate();`
   * @param key name in `AppRoute`
   * @param arg data in `let { state } = useLocation();`
   */
  openPage: function (navigate, key, arg = {}) {
    navigate(key, arg);
  },

  /**
   * Open email
   *
   * @param email
   */
  openEmail(email) {
    window.location.href = `mailto:${email}`;
  },

  /**
   * Open phone
   *
   * @param phone
   */
  openPhone(phone) {
    window.location.href = `tel:${phone}`;
  },

  /**
   * Open outer url
   *
   * @param url
   */
  openUrl(url) {
    window.location.href = url;
  },

  /**
   * Open outer url
   *
   * @param url
   */
  openUrlNewTab(url) {
    window.open(url, "_blank", "noopener,noreferrer");
  },
};
