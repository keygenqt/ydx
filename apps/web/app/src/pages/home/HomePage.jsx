import { useTranslation } from "react-i18next";
import { Stack, useMediaQuery, useTheme, Typography } from "@mui/material";

export function HomePage(props) {
  // localization
  const { t } = useTranslation();
  // theme
  const theme = useTheme();
  const isSM = useMediaQuery(theme.breakpoints.down("sm"));
  // page
  return (
    <Stack
      spacing={isSM ? 4 : 7}
      sx={{
        justifyContent: "center",
        alignItems: "center",
        height: "100%",
        paddingTop: isSM ? 4 : 7,
        paddingBottom: isSM ? 4 : 7,
      }}
    >
      <Typography color="textPrimary" variant="h4">
        {t("pages.home.t_text")}
      </Typography>
    </Stack>
  );
}

HomePage.propTypes = {};
