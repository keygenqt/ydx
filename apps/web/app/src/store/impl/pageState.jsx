import { createSlice } from "@reduxjs/toolkit";

export const pageStateSlice = createSlice({
  name: "pageState",
  initialState: {
    value: {},
  },
  reducers: {
    setData: (state, data) => {
      state.value[data.payload.route] = data.payload;
    },
  },
});

export const { setData } = pageStateSlice.actions;

export default pageStateSlice.reducer;
