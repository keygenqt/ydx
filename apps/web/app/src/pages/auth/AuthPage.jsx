import * as React from "react";
import { useTranslation } from "react-i18next";
import {
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Box,
  Stack,
  Card,
  Typography,
  Container,
  IconButton,
  useTheme,
  useMediaQuery,
  alpha,
} from "@mui/material";
import { Person, PersonAddAlt1, FiberManualRecord } from "@mui/icons-material";

import { FormSignin } from "./elements/FormSignin";
import { FormSignup } from "./elements/FormSignup";

export function AuthPage(props) {
  // localization
  const { t } = useTranslation();
  // theme
  const theme = useTheme();
  const isMD = useMediaQuery(theme.breakpoints.down("md"));
  // states
  const [toggleSignin, setToggleSignin] = React.useState(true);
  // page
  return (
    <Container
      maxWidth="xl"
      sx={{
        maxWidth: "1090px !important",
        paddingTop: 3,
        paddingBottom: 6,
      }}
    >
      <Card>
        <Stack direction={isMD ? "column" : "row"}>
          <Stack
            className="rt-circle"
            spacing={3}
            sx={{
              p: isMD ? 3 : 5,
              flex: 1,
              justifyContent: "center",
              justifyItems: "center",
            }}
          >
            <Typography
              variant="h4"
              color={"text.primary"}
              sx={{
                borderRadius: 3,
                p: isMD? 1 : 2,
                backgroundColor: theme => alpha(theme.palette.background.footer, 0.7),
                zIndex: 1
              }}
            >
              {toggleSignin
                ? t("pages.auth.t_title_signin")
                : t("pages.auth.t_title_signup")}
            </Typography>
            <Stack spacing={2} sx={{ display: isMD ? "none" : "inherit" }}>
              <Typography variant="subtitle1" color={"text.primary"}>
                {toggleSignin
                  ? t("pages.auth.t_subtitle_signin")
                  : t("pages.auth.t_subtitle_signup")}
              </Typography>
              <List dense sx={{ p: 0 }}>
                {(toggleSignin
                  ? t("pages.auth.benefits_signin", { returnObjects: true })
                  : t("pages.auth.benefits_signup", { returnObjects: true })
                ).map((benefit, index) => (
                  <ListItem key={index} sx={{ py: 0.5 }}>
                    <ListItemIcon sx={{ minWidth: 32 }}>
                      <FiberManualRecord
                        sx={{ fontSize: 8, color: "primary.main" }}
                      />
                    </ListItemIcon>
                    <ListItemText
                      primary={benefit}
                      slotProps={{
                        primary: {
                          variant: "body2",
                          color: "text.secondary",
                        },
                      }}
                    />
                  </ListItem>
                ))}
              </List>
            </Stack>
          </Stack>
          <Stack
            spacing={4}
            sx={{
              p: isMD ? 3 : 5,
              flex: 1,
              justifyContent: "center",
              justifyItems: "center",
              backgroundColor: "background.default",
            }}
          >
            <Box
              sx={{
                display: "flex",
                justifyContent: "end",
                justifyItems: "end",
              }}
            >
              <IconButton
                size={"small"}
                color={"primary"}
                onClick={() => {
                  setToggleSignin(!toggleSignin);
                }}
              >
                {toggleSignin ? <PersonAddAlt1 /> : <Person />}
              </IconButton>
            </Box>

            {toggleSignin ? <FormSignin /> : <FormSignup />}
          </Stack>
        </Stack>

        <Stack
          spacing={3}
          sx={{
            p: 3,
            display: "none",
          }}
        >
          <Stack
            direction={"row"}
            sx={{
              justifyContent: "space-between",
            }}
          >
            <Typography variant="h4" color={"text.primary"}>
              {toggleSignin
                ? t("pages.auth.t_title_signin")
                : t("pages.auth.t_title_signup")}
            </Typography>
            <Box
              sx={{
                display: "flex",
                alignItems: "center",
              }}
            >
              <Stack direction={"row"} spacing={1}></Stack>
            </Box>
          </Stack>
          <Stack direction={"row"}>
            <Stack
              spacing={1}
              sx={{
                flex: 1,
                justifyContent: "center",
                justifyItems: "center",
              }}
            >
              <Typography variant="subtitle1" color={"text.primary"}>
                {toggleSignin
                  ? t("pages.auth.t_subtitle_signin")
                  : t("pages.auth.t_subtitle_signup")}
              </Typography>
              <List dense sx={{ p: 0 }}>
                {(toggleSignin
                  ? t("pages.auth.benefits_signin", { returnObjects: true })
                  : t("pages.auth.benefits_signup", { returnObjects: true })
                ).map((benefit, index) => (
                  <ListItem key={index} sx={{ py: 0.5 }}>
                    <ListItemIcon sx={{ minWidth: 32 }}>
                      <FiberManualRecord
                        sx={{ fontSize: 8, color: "primary.main" }}
                      />
                    </ListItemIcon>
                    <ListItemText
                      primary={benefit}
                      slotProps={{
                        primary: {
                          variant: "body2",
                          color: "text.secondary",
                        },
                      }}
                    />
                  </ListItem>
                ))}
              </List>
            </Stack>

            <Stack
              spacing={1}
              sx={{
                minWidth: 400,
                justifyContent: "center",
                justifyItems: "center",
              }}
            >
              {toggleSignin ? <FormSignin /> : <FormSignup />}
            </Stack>
          </Stack>
        </Stack>
      </Card>
    </Container>
  );
}

AuthPage.propTypes = {};
