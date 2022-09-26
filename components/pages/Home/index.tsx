import { useEffect } from "react";
import type { NextPage } from "next";
import Head from "next/head";
import Carousel from "@/components/carousel";
import styles from "./Home.module.scss";

const Home: NextPage = () => {
  return (
    <div className={styles.home}>
      <Carousel
        list={[
          {
            link: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89",
            picture:
              "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89"
          },
          {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89",
            picture:
              "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89"
          },
          {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89",
            picture:
              "http://p1.music.126.net/6sVafSdb-H2CVATJxy86qg==/109951167856840904.jpg?imageView&quality=89"
          },
          {
            link: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89",
            picture:
              "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89"
          }
        ]}
      />
    </div>
  );
};

export default Home;
