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

export function FormSignin(props) {
  // localization
  const { t, i18n } = useTranslation();
  // navigate
  const navigate = useNavigate();
  // states
  const [showPassword, setShowPassword] = React.useState(false);
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
      })}
      onSubmit={async (values, { setErrors, setStatus, resetForm }) => {
        setStatus({ success: null });
        setErrors({ submit: null });

        // Loading for animation
        await new Promise((r) => setTimeout(r, 500));

        try {
          await HttpClient.signin(values.email, values.password);
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
        <form style={{ width: "100%" }} noValidate onSubmit={handleSubmit}>
          <FormGroup
            sx={{
              width: 1,
              maxWidth: 480,
              m: "0 auto",
              "& .Logo": {
                width: 100,
                m: "0 auto",
              },
            }}
          >
            <Box id={"FormId"}>
              <Card>
                <Stack spacing={3} sx={{ p: 4 }}>
                  <Stack spacing={1} sx={{ textAlign: "center" }}>
                    <Typography variant="h3" color={"text.primary"}>
                      {t("pages.signin.t_welcome")}
                    </Typography>
                    <Typography variant="caption" color={"text.primary"}>
                      {t("pages.signin.t_subtitle")}
                    </Typography>
                  </Stack>

                  {errors.submit && (
                    <AlertError onClose={() => setErrors({ submit: null })}>
                      {errors.submit}
                    </AlertError>
                  )}

                  {status && status.success && (
                    <AlertSuccess onClose={() => setStatus({ success: false })}>
                      {t("pages.signin.t_success")}
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
                    helperText={
                      touched.email && errors.email ? errors.email : ""
                    }
                    error={Boolean(touched.email && errors.email)}
                    onBlur={handleBlur}
                    onChange={handleChange}
                    fullWidth
                    label={t("pages.signin.t_label_email")}
                    variant="filled"
                  />

                  <FormControl
                    disabled={isSubmitting || (status && status.success)}
                    error={Boolean(touched.password && errors.password)}
                    fullWidth
                    variant="filled"
                  >
                    <InputLabel required>
                      {t("pages.signin.t_label_password")}
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

                  <Button
                    fullWidth
                    type={"submit"}
                    variant={"contained"}
                    size={"large"}
                    disabled={Boolean(
                      isSubmitting ||
                      (status && status.success) ||
                      Object.keys(errors).length !== 0 ||
                      Object.keys(touched).length === 0,
                    )}
                    startIcon={
                      isSubmitting && !errors.submit ? (
                        <CircularProgress
                          sx={{
                            mr: 0.5,
                            height: "18px !important",
                            width: "18px !important",
                          }}
                        />
                      ) : (
                        <DoneOutlined
                          color={"text.primary"}
                          sx={{ height: 18 }}
                        />
                      )
                    }
                  >
                    {t("pages.signin.t_btn_submit")}
                  </Button>
                </Stack>
              </Card>
            </Box>
          </FormGroup>
        </form>
      )}
    </Formik>
  );
}

FormSignin.propTypes = {};
