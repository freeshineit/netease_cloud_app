import React from "react";

declare global {
  declare namespace NetEase {
    /** 共组件 props */
    export interface CommonComponentProps {
      className?: string;
      style?: React.CSSProperties;
      children?: React.ReactNode;
    }
  }

  declare module "*.module.scss" {
    const classes: { readonly [key: string]: string };
    export default classes;
  }
}
