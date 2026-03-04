import { useTranslation } from "react-i18next";
import { useLocation } from "react-router";
import {
  IconButton,
  Box,
  Stack,
  useTheme,
  useMediaQuery,
  Typography,
  Container,
} from "@mui/material";

import { LanguageUtils } from "../../utils/LanguageUtils";

export function Footer(props) {
  // localization
  const { t, i18n } = useTranslation();
  const language = LanguageUtils.get(i18n);
  // theme
  const isLoading = false;
  // theme
  const theme = useTheme();
  const isMD = useMediaQuery(theme.breakpoints.down("md"));
  const isSM = useMediaQuery(theme.breakpoints.down("sm"));
  // page
  return (
    <Stack
      direction={isMD ? "column" : "row"}
      sx={{
        paddingX: 3,
        paddingY: 4,
        backgroundColor: "background.footer",
      }}
    >
      <Container maxWidth="xl">
        <Stack
          direction={"row"}
          sx={{
            justifyItems: "center",
            justifyContent: "space-between",
          }}
        >
          <Typography
            color="textPrimary"
            variant="body2"
            sx={{
              display: "flex",
              alignItems: "center",
            }}
          >
            {t("layouts.footer.t_copyright")}
          </Typography>

          <IconButton
            size={isSM ? "small" : "medium"}
            disabled={isLoading}
            edge={"end"}
            color={"primary"}
            sx={{
              opacity: isLoading ? 0.7 : 1,
            }}
            onClick={() => {
              // Change lang
              if (language === "en") {
                i18n.changeLanguage("ru");
              } else if (language === "ru") {
                i18n.changeLanguage("cn");
              } else {
                i18n.changeLanguage("en");
              }
            }}
          >
            <Stack
              sx={{
                height: isSM ? 22 : 24,
                width: isSM ? 22 : 24,
                borderRadius: 50,
                borderColor: "primary",
                border: "1px solid",
                color: "primary",
                fontSize: 11,
                fontWeight: "bold",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <Box>{t("layouts.header.t_language")}</Box>
            </Stack>
          </IconButton>
        </Stack>
      </Container>
    </Stack>
  );
}

Footer.propTypes = {};
