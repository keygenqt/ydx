import { useTranslation } from "react-i18next";
import { Stack } from "@mui/material";

import { FormSignin } from "./elements/FormSignin";

export function AuthPage(props) {
  // localization
  const { t } = useTranslation();
  // page
  return (
    <Stack sx={{ alignItems: "center" }}>
      <FormSignin />
    </Stack>
  );
}

AuthPage.propTypes = {};
