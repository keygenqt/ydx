import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";
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
  Book,
  Logout,
  Brightness5Outlined,
  Brightness7Outlined,
} from "@mui/icons-material";
import { AppCache } from "../../constants/AppCache";
import { useEffectCache } from "../../components/hooks";
import { CacheUtils } from "../../utils/CacheUtils";
import { AppPath } from "../../constants/AppPath";
import { NavigationUtils } from "../../utils/NavigationUtils";
import { HttpClient } from "../../utils/HttpClient";

export function Header(props) {
  // localization
  const { t } = useTranslation();
  // theme
  const theme = useTheme();
  const darkMode = useEffectCache(AppCache.darkModeKey);
  const isLogin = useEffectCache(AppCache.isLogin);
  const isSM = useMediaQuery(theme.breakpoints.down("sm"));
  // navigate
  const navigate = useNavigate();
  // page
  return (
    <Container maxWidth="xl">
      <AppBar position="static" elevation={0} color="transparent">
        <Toolbar
          sx={{
            paddingLeft: "0 !important",
            paddingRight: "0 !important",
          }}
        >
          <Stack direction={"row"} spacing={isSM ? 0 : 2}>
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
                backgroundColor: "primary.main",
              }}
            />
            <Stack direction={"row"} spacing={0}>
              <Button
                size={isSM ? "small" : "medium"}
                color={"primary"}
                onClick={() => {
                  NavigationUtils.openPage(navigate, AppPath.manager);
                }}
              >
                {t("layouts.header.t_manager")}
              </Button>
              <Button
                sx={{
                  display: isSM ? "none" : "inherit",
                }}
                size={isSM ? "small" : "medium"}
                color={"primary"}
                onClick={() => {
                  NavigationUtils.openUrl(AppPath.docs);
                }}
              >
                {t("layouts.header.t_documentation")}
              </Button>
            </Stack>
          </Stack>

          <Box sx={{ flexGrow: 1 }} />

          <Stack direction={"row"} spacing={1} sx={{ marginRight: -1 }}>
            {!isSM ? null : (
              <IconButton
                size={isSM ? "small" : "medium"}
                color={"primary"}
                onClick={async () => {
                  NavigationUtils.openUrl(AppPath.docs);
                }}
              >
                <Book />
              </IconButton>
            )}
            <IconButton
              size={isSM ? "small" : "medium"}
              color={"primary"}
              onClick={() => {
                CacheUtils.set(AppCache.darkModeKey, !darkMode);
              }}
            >
              {darkMode ? <Brightness7Outlined /> : <Brightness5Outlined />}
            </IconButton>
            {!isLogin ? null : (
              <IconButton
                size={isSM ? "small" : "medium"}
                color={"primary"}
                onClick={async () => {
                  await HttpClient.signout();
                  CacheUtils.set(AppCache.isLogin, false);
                }}
              >
                <Logout />
              </IconButton>
            )}
          </Stack>
        </Toolbar>
      </AppBar>
    </Container>
  );
}

Header.propTypes = {};
