import { createSlice } from "@reduxjs/toolkit";

// create a slice
export const authSlice = createSlice({
  name: "auth",
  initialState: {
    auth: "auth"
  },
  reducers: {
    iconMoon: state => {
      state.auth = "moon";
    },
    iconSun: state => {
      state.auth = "sun";
    }
  }
});

// export the action
export const authAction = authSlice.actions;
