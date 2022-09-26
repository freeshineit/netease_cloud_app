import React, { FC } from "react";
import styles from "./layout.module.scss";

export interface LayoutContentProps extends NetEase.CommonComponentProps {}

/**
 * 内容区
 * @returns
 */
const LayoutContent: FC<LayoutContentProps> = props => {
  return <main className={styles.content}>{props.children}</main>;
};

export default LayoutContent;
