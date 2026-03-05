import { useTranslation } from "react-i18next";
import { Stack, Typography } from "@mui/material";

export function ManagerPage(props) {
  // localization
  const { t } = useTranslation();
  // page
  return (
    <Stack sx={{ alignItems: "center" }}>
      <Typography color="textPrimary" variant="h4">
        {t("pages.home.t_text")}
      </Typography>
    </Stack>
  );
}

ManagerPage.propTypes = {};
