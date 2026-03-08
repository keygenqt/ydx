import * as React from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { Formik } from "formik";
import * as Yup from "yup";
import {
  Box,
  Button,
  CircularProgress,
  FormGroup,
  Stack,
  TextField,
  Card,
  Typography,
  InputAdornment,
  IconButton,
  FilledInput,
  FormControl,
  InputLabel,
  FormHelperText,
} from "@mui/material";
import { AlertError, AlertSuccess } from "../../../components/ui";
import { DoneOutlined, VisibilityOff, Visibility } from "@mui/icons-material";
import { HttpClient } from "../../../utils/HttpClient";
import { FormValidate } from "../../../utils/FormValidate";
import { CacheUtils } from "../../../utils/CacheUtils";
import { AppCache } from "../../../constants/AppCache";
import { NavigationUtils } from "../../../utils/NavigationUtils";
import { AppPath } from "../../../constants/AppPath";

export function FormSignup(props) {
  // localization
  const { t, i18n } = useTranslation();
  // navigate
  const navigate = useNavigate();
  // states
  const [showPassword, setShowPassword] = React.useState(false);
  const [showPasswordConfirm, setShowPasswordConfirm] = React.useState(false);
  const formikRef = React.useRef(null);
  // init callbacks
  React.useEffect(() => {
    if (formikRef.current) {
      setTimeout(() => {
        formikRef.current?.resetForm?.();
      }, 0);
    }
  }, [i18n.language]);
  // page
  return (
    <Formik
      innerRef={formikRef}
      initialValues={{
        email: "",
        password: "",
        passwordConfirm: "",
        submit: null,
      }}
      validationSchema={Yup.object().shape({
        email: Yup.string()
          .min(3, t("Size must be between 3 and 250."))
          .max(250, t("Size must be between 3 and 250."))
          // .email(t("Must be a well-formed email address."))
          .required(t("Must not be not blank.")),
        password: Yup.string()
          .min(8, t("Size must be between 8 and 16."))
          .max(16, t("Size must be between 8 and 16."))
          .required(t("Must not be not blank.")),
        passwordConfirm: Yup.string()
          .min(8, t("Size must be between 8 and 16."))
          .max(16, t("Size must be between 8 and 16."))
          .required(t("Must not be not blank."))
          .oneOf([Yup.ref("password")], t("Passwords must match.")),
      })}
      onSubmit={async (values, { setErrors, setStatus, resetForm }) => {
        setStatus({ success: null });
        setErrors({ submit: null });

        // Loading for animation
        await new Promise((r) => setTimeout(r, 500));

        try {
          await HttpClient.signup(values.email, values.password);
          resetForm();
          setStatus({ success: true });
          // Loading for animation
          await new Promise((r) => setTimeout(r, 1000));
          // Set login to cache
          CacheUtils.set(AppCache.isLogin, true);
          // Back to user page
          NavigationUtils.replacePage(navigate, AppPath.manager);
        } catch (error) {
          if (error.code === 422 && error.validates !== null) {
            setErrors({
              email: FormValidate.findError(t, "email", error),
              password: FormValidate.findError(t, "password", error),
            });
          } else {
            setErrors({
              submit: t(error.message),
            });
          }
        }
      }}
    >
      {({
        status,
        errors,
        touched,
        values,
        isSubmitting,
        setStatus,
        setErrors,
        handleBlur,
        handleChange,
        handleSubmit,
      }) => (
        <form
          style={{
            flex: 1,
            display: "flex",
          }}
          noValidate
          onSubmit={handleSubmit}
        >
          <FormGroup
            sx={{
              flex: 1,
              display: "flex",
            }}
          >
            <Stack
              spacing={3}
              sx={{
                flex: 1,
                justifyContent: "space-between",
              }}
            >
              <Stack spacing={3}>
                {errors.submit && (
                  <AlertError onClose={() => setErrors({ submit: null })}>
                    {errors.submit}
                  </AlertError>
                )}

                {status && status.success && (
                  <AlertSuccess onClose={() => setStatus({ success: false })}>
                    {t("pages.signup.t_success")}
                  </AlertSuccess>
                )}

                <TextField
                  disabled={isSubmitting || (status && status.success)}
                  required
                  type={"text"}
                  name={"email"}
                  value={values.email}
                  slotProps={{
                    htmlInput: {
                      maxLength: 250,
                    },
                  }}
                  helperText={touched.email && errors.email ? errors.email : ""}
                  error={Boolean(touched.email && errors.email)}
                  onBlur={handleBlur}
                  onChange={handleChange}
                  fullWidth
                  label={t("pages.signup.t_label_email")}
                  variant="filled"
                />

                <FormControl
                  disabled={isSubmitting || (status && status.success)}
                  error={Boolean(touched.password && errors.password)}
                  fullWidth
                  variant="filled"
                >
                  <InputLabel required>
                    {t("pages.signup.t_label_password")}
                  </InputLabel>

                  <FilledInput
                    required
                    type={showPassword ? "text" : "password"}
                    name={"password"}
                    value={values.password}
                    onBlur={handleBlur}
                    onChange={handleChange}
                    fullWidth
                    inputProps={{
                      maxLength: 16,
                    }}
                    endAdornment={
                      <InputAdornment position="end">
                        <IconButton
                          aria-label="Password visibility"
                          onClick={() => {
                            setShowPassword(!showPassword);
                          }}
                          onMouseDown={() => {
                            setShowPassword(false);
                          }}
                          edge="end"
                        >
                          {showPassword ? <VisibilityOff /> : <Visibility />}
                        </IconButton>
                      </InputAdornment>
                    }
                  />

                  {touched.password && errors.password && (
                    <FormHelperText error>{errors.password}</FormHelperText>
                  )}
                </FormControl>

                <FormControl
                  disabled={isSubmitting || (status && status.success)}
                  error={Boolean(
                    touched.passwordConfirm && errors.passwordConfirm,
                  )}
                  fullWidth
                  variant="filled"
                >
                  <InputLabel required>
                    {t("pages.signup.t_label_password_confirm")}
                  </InputLabel>

                  <FilledInput
                    required
                    type={showPasswordConfirm ? "text" : "password"}
                    name={"passwordConfirm"}
                    value={values.passwordConfirm}
                    onBlur={handleBlur}
                    onChange={handleChange}
                    fullWidth
                    inputProps={{
                      maxLength: 16,
                    }}
                    endAdornment={
                      <InputAdornment position="end">
                        <IconButton
                          aria-label="Password confirm visibility"
                          onClick={() => {
                            setShowPasswordConfirm(!showPassword);
                          }}
                          onMouseDown={() => {
                            setShowPasswordConfirm(false);
                          }}
                          edge="end"
                        >
                          {showPasswordConfirm ? (
                            <VisibilityOff />
                          ) : (
                            <Visibility />
                          )}
                        </IconButton>
                      </InputAdornment>
                    }
                  />

                  {touched.passwordConfirm && errors.passwordConfirm && (
                    <FormHelperText error>
                      {errors.passwordConfirm}
                    </FormHelperText>
                  )}
                </FormControl>
              </Stack>
              <Button
                fullWidth
                type={"submit"}
                variant={"contained"}
                size={"large"}
                disabled={Boolean(isSubmitting || (status && status.success))}
                startIcon={
                  isSubmitting && !errors.submit ? (
                    <CircularProgress
                      sx={{
                        mr: 0.5,
                        height: "18px !important",
                        width: "18px !important",
                      }}
                    />
                  ) : null
                }
              >
                {t("pages.signup.t_btn_submit")}
              </Button>
            </Stack>
          </FormGroup>
        </form>
      )}
    </Formik>
  );
}

FormSignup.propTypes = {};
