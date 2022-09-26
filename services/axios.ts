//
/**
 * 网络请求配置
 */
import axios from "axios";

const TIMEOUT = 60000;
const BASE_URL = "http://xxxx.xxxx.com/";

const instance = axios.create({
  baseURL: BASE_URL,
  timeout: TIMEOUT
});

/**
 * http request 拦截器
 */
instance.interceptors.request.use(
  config => {
    config.data = JSON.stringify(config.data);
    config.headers = {
      "Content-Type": "application/json"
    };
    return config;
  },
  error => {
    return Promise.reject(error);
  }
);

/**
 * http response 拦截器
 */
instance.interceptors.response.use(
  response => {
    if (response.data.errCode === 2) {
      console.log("过期");
    }
    return response;
  },
  error => {
    console.log("请求出错：", error);
  }
);

export default instance;
