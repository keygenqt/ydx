export const FormValidate = {
  /**
   * Find error array validate
   * @param {Function} t - i18n translate function
   * @param {string} field - Field name
   * @param {Object} error - Error object from server
   * @returns {string|null}
   */
  findError: function (t, field, error) {
    const fieldErrors = error?.data?.errors?.[field];
    if (!fieldErrors || !Array.isArray(fieldErrors)) {
      return null;
    }
    return fieldErrors
      .map((e) => t(e.message))
      .join(" ");
  },
};
