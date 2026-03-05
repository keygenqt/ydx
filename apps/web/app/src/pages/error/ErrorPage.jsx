import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
import { Button, Stack, Typography } from "@mui/material";
import { NavigationUtils } from "../../utils/NavigationUtils";
import { LanguageUtils } from "../../utils/LanguageUtils";

export function ErrorPage(props) {
  // localization
  const { t, i18n } = useTranslation();
  const language = LanguageUtils.get(i18n);
  // navigate
  const navigate = useNavigate();
  // page
  return (
    <Stack
      spacing={2}
      sx={{
        border: "4px solid",
        borderColor: "primary.main",
        borderRadius: "50%",
        padding: 6,
        justifyContent: "space-around",
        alignItems: "center",
        width: 200,
        height: 200,
        margin: "48px auto",
        textAlign: "center",
        backgroundColor: "background.paper",
      }}
    >
      <Typography
        color="textPrimary"
        variant="h4"
        sx={{
          paddingTop: language === "cn" ? "12px" : 0,
        }}
      >
        {t("pages.error.t_title")}
      </Typography>
      <Typography
        color="textPrimary"
        variant="body2"
        sx={{ paddingBottom: "4px" }}
      >
        {t("pages.error.t_text")}
      </Typography>
      <Button
        size="large"
        variant="contained"
        onClick={() => {
          NavigationUtils.toHome(navigate);
        }}
      >
        {t("pages.error.t_btn")}
      </Button>
    </Stack>
  );
}

ErrorPage.propTypes = {};
