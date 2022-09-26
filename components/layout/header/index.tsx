import React, { FC, useCallback } from "react";
import { useRouter } from "next/router";
import cls from "classnames";
import { HEADER_MENU } from "../constant";
import layoutStyles from "../layout.module.scss";

import styles from "./Header.module.scss";

export interface LayoutHeaderProps extends NetEase.CommonComponentProps {}

/**
 * 顶部区
 * @returns
 */
const LayoutHeader: FC<LayoutHeaderProps> = () => {
  const router = useRouter();

  const handlePrev = useCallback(() => {}, []);
  const handleNext = useCallback(() => {}, []);

  return (
    <header className={cls(layoutStyles.header, styles.header)}>
      <div className={styles.left}>
        <span>&lt;</span>
        <span>&gt;</span>
      </div>
      <div className={styles.right}>
        <div className={styles.menu}></div>
        <div className={styles.btns}>btns</div>
      </div>
    </header>
  );
};

export default LayoutHeader;
