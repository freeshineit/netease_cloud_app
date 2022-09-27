import { Reducer, Slice } from "@reduxjs/toolkit";

import { authSlice } from "./auth";

const combineSlice = {
  auth: authSlice
};

type CombineSliceKeyType = keyof typeof combineSlice;

// https://jsonplaceholder.typicode.com/users/${state$.value.nextUserId}

// get all reducer
export const reducers = Object.entries<Slice>(combineSlice).reduce(
  (prev: Record<CombineSliceKeyType, Reducer>, entry) => {
    prev[entry[0] as CombineSliceKeyType] =
      combineSlice[entry[0] as CombineSliceKeyType].reducer;

    return prev;
  },
  {} as Record<CombineSliceKeyType, Reducer>
);

export type CombineSliceType = typeof combineSlice;

export default combineSlice;
