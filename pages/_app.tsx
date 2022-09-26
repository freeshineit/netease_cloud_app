import type { AppProps } from "next/app";
import { Provider } from "react-redux";
import store from "@/store";

import Layout, {
  LayoutHeader,
  LayoutContainer,
  LayoutFooter,
  LayoutSider,
  LayoutContent
} from "@/components/layout";

import "../styles/globals.scss";

function MyApp({ Component, pageProps }: AppProps) {
  // const store = useStore(pageProps?.initialReduxState || {});

  console.log("2341234");

  return (
    <Provider store={store}>
      <Layout>
        <LayoutHeader />
        <LayoutContainer>
          <LayoutSider />
          <LayoutContent>
            <Component {...pageProps} />
          </LayoutContent>
        </LayoutContainer>
        <LayoutFooter />
      </Layout>
    </Provider>
  );
}

export default MyApp;
