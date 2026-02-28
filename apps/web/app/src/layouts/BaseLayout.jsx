import PropTypes from "prop-types";
import { useEffect } from "react";
import { useLocation } from "react-router";
import { Box, Container, Stack } from "@mui/material";
import { Header } from "./elements/Header";
import { Footer } from "./elements/Footer";
import { useEffectCache, useEffectPageScroll } from "../components/hooks";
import { AppCache } from "../constants/AppCache";

export function BaseLayout(props) {
  const { pathname } = useLocation();
  const scrollTop = useEffectPageScroll();
  const darkMode = useEffectCache(AppCache.darkModeKey);

  useEffect(() => {
    document.getElementById("root").scrollTop = scrollTop;
  }, [pathname /* scrollTop - only start set scrollTop */]);

  return (
    <Stack
      className={darkMode ? "ThemeDark" : "ThemeLight"}
      sx={{
        flex: 1,
        minHeight: 1,
        backgroundColor: "background.default",
        justifyContent: "space-between",
      }}
    >
      <Header />
      <Box>{props.children}</Box>
      <Footer />
    </Stack>
  );
}

BaseLayout.propTypes = {
  children: PropTypes.element.isRequired,
  isCenter: PropTypes.bool,
};
