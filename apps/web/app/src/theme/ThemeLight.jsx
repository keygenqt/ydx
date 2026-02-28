import { createTheme } from "@mui/material/styles";
import { palette } from "./elements/light/palette";
import { breakpoints } from "./elements/common/breakpoints";
import { typography } from "./elements/common/typography";
import { components } from "./elements/common/components";

export const ThemeLight = createTheme({
  palette: palette,
  breakpoints: breakpoints,
  typography: typography,
  components: components,
});
