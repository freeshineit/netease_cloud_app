import React, { FC } from "react";
import layoutStyles from "../layout.module.scss";

export interface LayoutFooterProps extends NetEase.CommonComponentProps {}

/**
 * 底部区
 * @returns
 */
const LayoutFooter: FC<LayoutFooterProps> = () => {
  return <footer className={layoutStyles.footer}>footer</footer>;
};

export default LayoutFooter;
