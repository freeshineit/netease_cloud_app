import { configureStore } from "@reduxjs/toolkit";
import { createLogger } from "redux-logger";
import { reducers } from "./slices";

const logger = createLogger({ collapsed: true }); // log every action to see what's happening behind the scenes.

// config the store
const store = configureStore({
  reducer: reducers,
  middleware: getDefaultMiddleware => getDefaultMiddleware().concat([logger])
});

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;

// export default the store
export default store;
