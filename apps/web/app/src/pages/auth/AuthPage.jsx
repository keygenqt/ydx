import { useTranslation } from "react-i18next";
import { Stack, Typography } from "@mui/material";

export function AuthPage(props) {
  // localization
  const { t } = useTranslation();
  // page
  return (
    <Stack sx={{ alignItems: "center" }}>
      <Typography color="textPrimary" variant="h4">
        Auth
      </Typography>
    </Stack>
  );
}

AuthPage.propTypes = {};
