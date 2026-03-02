import React from "react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { Stack, Typography } from "@mui/material";
import { NavigationUtils } from "../../utils/NavigationUtils";
import { AppPath } from "../../constants/AppPath";
import { BaseLayout } from "../../layouts/BaseLayout";

import { ydx_wasm_request_signcheck } from "../../wasm/ydx_wasm.js";

async function authLoader() {
  const value = await ydx_wasm_request_signcheck("device_uuid");
  return value.includes("200");
}

export function ProtectedRoute({ children }) {
  // localization
  const { t } = useTranslation();
  // navigate
  const navigate = useNavigate();

  const [isAuth, setIsAuth] = React.useState(null);
  const wasCalled = React.useRef(false)

  React.useEffect(() => {
    if(wasCalled.current) return;
    wasCalled.current = true;
    authLoader()
      .then((isAuthenticated) => {
        if (isAuthenticated) {
          setIsAuth(true);
        } else {
          NavigationUtils.openPage(navigate, AppPath.auth);
        }
      })
      .catch(() => NavigationUtils.openPage(navigate, AppPath.auth));
  }, []);

  if (isAuth === null) {
    return (
      <BaseLayout isCenter={true}>
        <Stack sx={{ alignItems: "center" }}>
          <Typography color="textPrimary" variant="h4">
            {t("pages.protected.t_text")}
          </Typography>
        </Stack>
      </BaseLayout>
    );
  }

  return children;
}
