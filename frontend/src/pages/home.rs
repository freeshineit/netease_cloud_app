use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::common::carousel::{Carousel, CarouselItem};

#[function_component(Home)]
pub fn home() -> Html {
    let style = style!(
        r#"
          .banners {
            background-color: #252525;
          }
      "#
    )
    .expect("Failed to mount style!");

    let banners: Vec<CarouselItem> = vec![
        CarouselItem {
            link: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/6sVafSdb-H2CVATJxy86qg==/109951167856840904.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/SGqdQAfdQprAc-PMHDJeQw==/109951167856768394.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/SGqdQAfdQprAc-PMHDJeQw==/109951167856768394.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
          link: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89".to_string(),
          picture: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/6sVafSdb-H2CVATJxy86qg==/109951167856840904.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/SGqdQAfdQprAc-PMHDJeQw==/109951167856768394.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/SGqdQAfdQprAc-PMHDJeQw==/109951167856768394.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
          link: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89".to_string(),
          picture: "http://p1.music.126.net/GGeoW0LCRXJJ0jun3JMWpw==/109951167860351872.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/FBlyL7SGHRZaUGC2Lv1EAw==/109951167856761557.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/6sVafSdb-H2CVATJxy86qg==/109951167856840904.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/bkHSyyDciMZmOENYqGE3dQ==/109951167856855160.jpg?imageView&quality=89".to_string(),
        },
        CarouselItem {
            link: "http://p1.music.126.net/SGqdQAfdQprAc-PMHDJeQw==/109951167856768394.jpg?imageView&quality=89".to_string(),
            picture: "http://p1.music.126.net/SGqdQAfdQprAc-PMHDJeQw==/109951167856768394.jpg?imageView&quality=89".to_string(),
        },
      ];

    html! {
      <div class={style}>
        <div class="banners">
          <Carousel list={banners} />
        </div>
      </div>
    }
}
