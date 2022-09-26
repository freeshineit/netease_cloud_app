import React, { FC } from "react";
import styles from "./layout.module.scss";

export interface LayoutContainerProps {
  children: React.ReactNode;
}

/**
 * 内容区
 * @returns
 */
const LayoutContainer: FC<LayoutContainerProps> = props => {
  return <main className={styles.container}>{props.children}</main>;
};

export default LayoutContainer;
