import * as types from "./constant";

export const startFetchingUsers = () => ({
  type: types.START_FETCHING_USERS
});
export const stopFetchingUsers = () => ({
  type: types.STOP_FETCHING_USERS
});
export const fetchUser = (isServer = false) => ({
  type: types.FETCH_USER,
  payload: { isServer }
});
export const fetchUserSuccess = (response: any, isServer: boolean) => ({
  type: types.FETCH_USER_SUCCESS,
  payload: { response, isServer }
});

export const fetchUserFailure = (error: any, isServer: boolean) => ({
  type: types.FETCH_USER_FAILURE,
  payload: { error, isServer }
});
