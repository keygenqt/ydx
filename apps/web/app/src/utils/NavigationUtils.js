export const NavigationUtils = {
  /**
   * Navigates to the home/root page
   *
   * @param {Function} navigate - React Router's navigate function from useNavigate()
   * @returns {void}
   */
  toHome: function (navigate) {
    navigate("/");
  },

  /**
   * Navigates back one step in history
   *
   * @param {Function} navigate - React Router's navigate function from useNavigate()
   * @returns {void}
   */
  back: function (navigate) {
    navigate(-1);
  },

  /**
   * Navigates to a specified route with optional state data
   *
   * @param {Function} navigate - React Router's navigate function from useNavigate()
   * @param {string} path - Route path or key from AppRoute configuration
   * @param {Object} [state={}] - State data to pass to the route (accessible via useLocation())
   * @returns {void}
   *
   * @example
   * // Navigate to profile with user data
   * openPage(navigate, '/profile', { userId: 123, name: 'John' })
   */
  openPage: function (navigate, path, state = {}) {
    navigate(path, { state });
  },

  /**
   * Replaces the current page in history with a specified route (cannot go back)
   *
   * @param {Function} navigate - React Router's navigate function from useNavigate()
   * @param {string} path - Route path or key from AppRoute configuration
   * @param {Object} [state={}] - State data to pass to the route (accessible via useLocation())
   * @returns {void}
   *
   * @example
   * // Replace current page with dashboard after login
   * replacePage(navigate, '/dashboard', { from: 'login' })
   *
   * @example
   * // Replace with 404 page
   * replacePage(navigate, '/404')
   */
  replacePage: function (navigate, path, state = {}) {
    navigate(path, { replace: true, state });
  },

  /**
   * Refreshes the current page by navigating to the same path with replace
   *
   * @param {Function} navigate - React Router's navigate function from useNavigate()
   * @returns {void}
   *
   * @example
   * // Refresh current page (e.g., after language change)
   * NavigationUtils.refreshPage(navigate);
   */
  refreshPage: function (navigate) {
    navigate(".", { replace: true });
  },

  /**
   * Opens default email client with pre-filled recipient
   *
   * @param {string} email - Recipient's email address
   * @returns {void}
   *
   * @example
   * openEmail('support@example.com')
   */
  openEmail(email) {
    window.location.href = `mailto:${email}`;
  },

  /**
   * Opens default phone app with the specified number
   *
   * @param {string} phone - Phone number to dial
   * @returns {void}
   *
   * @example
   * openPhone('+1234567890')
   */
  openPhone(phone) {
    window.location.href = `tel:${phone}`;
  },

  /**
   * Opens an external URL in the current tab
   *
   * @param {string} url - Full URL to navigate to
   * @returns {void}
   *
   * @example
   * openUrl('https://example.com')
   */
  openUrl(url) {
    window.location.href = url;
  },

  /**
   * Opens an external URL in a new browser tab with security best practices
   *
   * @param {string} url - Full URL to open in new tab
   * @returns {void}
   *
   * @description
   * Uses 'noopener,noreferrer' for security:
   * - noopener: Prevents new tab from accessing window.opener
   * - noreferrer: Prevents sending referrer information
   *
   * @example
   * openUrlNewTab('https://example.com')
   */
  openUrlNewTab(url) {
    window.open(url, "_blank", "noopener,noreferrer");
  },
};
