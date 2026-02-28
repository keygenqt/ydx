import { createTheme } from "@mui/material/styles";
import { palette } from "./elements/dark/palette";
import { breakpoints } from "./elements/common/breakpoints";
import { components } from "./elements/common/components";
import { typography } from "./elements/common/typography";

export const ThemeDark = createTheme({
  palette: palette,
  breakpoints: breakpoints,
  typography: typography,
  components: components,
});
