import { configureStore } from "@reduxjs/toolkit";

import pageScrollReducer from "./impl/pageScroll";
import pageStateReducer from "./impl/pageState";

const Store = configureStore({
  reducer: {
    pageScroll: pageScrollReducer,
    pageState: pageStateReducer,
  },
});

export default Store;
