import { BorderColor } from "@mui/icons-material";

/**
 * @link https://mui.com/material-ui/customization/theme-components/
 */
export const components = {
  MuiButton: {
    styleOverrides: {
      root: {
        textTransform: "none",
        boxShadow: "none !important",
        paddingLeft: 14,
        paddingRight: 14,
        paddingTop: 5,
        paddingBottom: 5,
      },
    },
  },
};
