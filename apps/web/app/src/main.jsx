import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from 'react-redux';
import Store from './store/configure';
import App from "./App";

import init from "./wasm/ydx_wasm.js";
import './localization/Localization';
import './assets/css/style.css';
import { HttpClient } from './utils/HttpClient';
import { CacheUtils } from "./utils/CacheUtils.js";
import { AppCache } from "./constants/AppCache.js";

// Init library core wasm
await init();

// Initialize HTTP client
HttpClient.init();

// Check user is auth
CacheUtils.set(AppCache.isLogin, await HttpClient.signcheck());

// Start app react
createRoot(document.getElementById("root")).render(
  <StrictMode>
    <Provider store={Store}>
      <App />
    </Provider>
  </StrictMode>
);
