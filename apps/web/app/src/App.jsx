import { ThemeProvider } from "@mui/material";
import { AppRoute } from "./AppRoute";
import { ThemeLight } from "./theme/ThemeLight";
import { ThemeDark } from "./theme/ThemeDark";
import { AppCache } from "./constants/AppCache";
import { useEffectCache } from "./components/hooks";

function App() {
  const darkMode = useEffectCache(AppCache.darkModeKey);
  return (
    <ThemeProvider theme={darkMode ? ThemeDark : ThemeLight}>
        <AppRoute />
    </ThemeProvider>
  );
}

export default App;
