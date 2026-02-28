import { useTranslation } from "react-i18next";
import { useNavigate, useLocation } from "react-router";
import {
  AppBar,
  Toolbar,
  IconButton,
  Box,
  Stack,
  Divider,
  useTheme,
  useMediaQuery,
  Button,
  Container,
} from "@mui/material";
import {
  Home,
  Brightness5Outlined,
  Brightness7Outlined,
} from "@mui/icons-material";
import { AppCache } from "../../constants/AppCache";
import { useEffectCache } from "../../components/hooks";
import { CacheUtils } from "../../utils/CacheUtils";
import { AppRoute } from "../../constants/AppRoute";
import { NavigationUtils } from "../../utils/NavigationUtils";
import { LanguageUtils } from "../../utils/LanguageUtils";

export function Header(props) {
  // route
  const { pathname } = useLocation();
  // localization
  const { t, i18n } = useTranslation();
  const language = LanguageUtils.get(i18n);
  // theme
  const isLoading = false;
  const theme = useTheme();
  const darkMode = useEffectCache(AppCache.darkModeKey);
  const isSM = useMediaQuery(theme.breakpoints.down("sm"));
  // navigate
  const navigate = useNavigate();
  // page
  return (
    <Container maxWidth="xl">
      <AppBar position="static" elevation={0} color="transparent">
        <Toolbar>
          <Stack direction={"row"} spacing={2}>
            <Button
              color={"primary"}
              startIcon={<Home />}
              onClick={() => {
                NavigationUtils.toHome(navigate);
              }}
            >
              {t("layouts.header.t_name")}
            </Button>
            <Divider
              orientation={"vertical"}
              variant={"middle"}
              flexItem
              sx={{
                backgroundColor: "primary.main"
              }}
            />
            <Stack direction={"row"} spacing={0}>
              <Button
                color={"primary"}
                onClick={() => {
                  NavigationUtils.openPage(navigate, AppRoute.app);
                }}
              >
                {t("layouts.header.t_manager")}
              </Button>
              <Button
                color={"primary"}
                onClick={() => {
                  NavigationUtils.openPage(navigate, AppRoute.apps);
                }}
              >
                {t("layouts.header.t_applications")}
              </Button>
              <Button
                color={"primary"}
                onClick={() => {
                  NavigationUtils.openPage(navigate, AppRoute.docs);
                }}
              >
                {t("layouts.header.t_documentation")}
              </Button>
            </Stack>
          </Stack>

          <Box sx={{ flexGrow: 1 }} />

          <Stack direction={"row"} spacing={1} sx={{ marginRight: -1 }}>
            <IconButton
              size={isSM ? "small" : "medium"}
              color={"primary"}
              onClick={() => {
                CacheUtils.set(AppCache.darkModeKey, !darkMode);
              }}
            >
              {darkMode ? <Brightness7Outlined /> : <Brightness5Outlined />}
            </IconButton>
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
        </Toolbar>
      </AppBar>
    </Container>
  );
}

Header.propTypes = {};
