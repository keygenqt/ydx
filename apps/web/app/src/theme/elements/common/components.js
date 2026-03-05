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
  MuiCard: {
    styleOverrides: {
      root: ({ theme }) =>
        theme.unstable_sx({
          boxShadow: "none !important",
          border: "4px solid",
          borderColor: "primary.main",
          borderRadius: 3,
        }),
    },
  },
};
