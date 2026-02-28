import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Provider } from 'react-redux';
import Store from './store/configure';
import App from "./App";

import init from "./wasm/ydx_wasm.js";
import './localization/Localization';
import './assets/css/style.css';

// Init library core wasm
await init();

// Start app react
createRoot(document.getElementById("root")).render(
  <StrictMode>
    <Provider store={Store}>
      <App />
    </Provider>
  </StrictMode>
);
