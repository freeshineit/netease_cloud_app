import { authSlice } from "./auth";

const combineSlice = {
  auth: authSlice
};

// https://jsonplaceholder.typicode.com/users/${state$.value.nextUserId}

// get all reducer
export const reducers = Object.keys(combineSlice).reduce(
  (prev: Record<string, any>, key: string) => {
    prev[key] = combineSlice[key].reducer;
    return prev;
  },
  {} as Record<string, any>
);

export type CombineSliceType = typeof combineSlice;

export default combineSlice;
