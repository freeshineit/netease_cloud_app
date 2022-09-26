import React, { FC, useEffect } from "react";
import Link from "next/link";
import cls from "classnames";
import { SIDER_MENU1, SIDER_MENU2, SIDER_MENU3 } from "../constant";
import layoutStyles from "../layout.module.scss";
import styles from "./Sider.module.scss";
import { useRouter } from "next/router";

export interface LayoutSiderProps extends NetEase.CommonComponentProps {}

/**
 * 侧边栏
 * @returns
 */
const LayoutSider: FC<LayoutSiderProps> = () => {
  const { pathname } = useRouter();

  useEffect(() => {
    console.log("pathname");
  }, []);

  return (
    <div className={layoutStyles.sider}>
      <div className={styles.account}>User</div>
      <div className={styles.menu}>
        {SIDER_MENU1.map(item => {
          return (
            <div key={item.title} className={styles.item}>
              <Link href={item.link as string}>
                <a
                  className={cls({
                    [styles.selected]: pathname === item.link
                  })}
                >
                  <span className={styles.icon}></span>
                  <div className="ellipsis1">{item.title}</div>
                </a>
              </Link>
            </div>
          );
        })}
      </div>
      <div className={styles.title}>我的音乐</div>
      <div className={styles.menu}>
        {SIDER_MENU2.map(item => {
          return (
            <div key={item.title} className={styles.item}>
              <Link href={item.link as string}>
                <a
                  className={cls({
                    [styles.selected]: pathname === item.link
                  })}
                >
                  <span className={styles.icon}></span>
                  <div className="ellipsis1">{item.title}</div>
                </a>
              </Link>
            </div>
          );
        })}
      </div>
      <div className={styles.title}>创建的歌单</div>
      <div className={styles.menu}>
        {SIDER_MENU3.map(item => {
          return (
            <div key={item.title} className={styles.item}>
              <Link href={item.link as string}>
                <a
                  className={cls({
                    [styles.selected]: pathname === item.link
                  })}
                >
                  <span className={styles.icon}></span>
                  <div className="ellipsis1">{item.title}</div>
                </a>
              </Link>
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default React.memo(LayoutSider);
