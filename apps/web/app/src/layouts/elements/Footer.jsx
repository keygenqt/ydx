import { useTranslation } from "react-i18next";
import {
  useTheme,
  Box,
  Button,
  Divider,
  Stack,
  SvgIcon,
  Typography,
  Tooltip,
  useMediaQuery,
  Container,
} from "@mui/material";
import { Telegram, GitHub } from "@mui/icons-material";
import { AppData } from "../../constants/AppData";
import { AppLinks } from "../../constants/AppLinks";
import { NavigationUtils } from "../../utils/NavigationUtils";
import { LanguageUtils } from "../../utils/LanguageUtils";

export function Footer(props) {
  // localization
  const { t, i18n } = useTranslation();
  const language = LanguageUtils.get(i18n);
  // theme
  const theme = useTheme();
  const isLG = useMediaQuery(theme.breakpoints.down("lg"));
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
        <Typography color="textPrimary" variant="body2">
          {t("layouts.footer.t_copyright")}
        </Typography>
      </Container>
    </Stack>
  );
}

Footer.propTypes = {};
