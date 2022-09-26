import React, { FC } from "react";

import LayoutContainer from "./container";
import LayoutContent from "./content";
import LayoutHeader from "./header";
import LayoutSider from "./sider";
import LayoutFooter from "./footer";

import styles from "./layout.module.scss";

export interface LayoutProps {
  children: React.ReactNode;
}

const Layout: FC<LayoutProps> = props => {
  return <div className={styles.layout}>{props.children}</div>;
};

export {
  LayoutContainer,
  LayoutContent,
  LayoutHeader,
  LayoutSider,
  LayoutFooter
};

export default Layout;
