use crate::components::layout::content::Route;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_router::prelude::*;

struct Links {
    name: String,
    to: Route,
}

const SIDER_SUER_HEIGHT: &str = "60px";

#[styled_component(Sider)]
pub fn sider() -> Html {
    let style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        flex: 1 1 auto;

        .user {
          color: #fff;
          height: ${sh};
        }

        .user .user-content {
          height: ${sh};
        }

        a {
          color: #fff;
        }

        .list-wrapper {
          display: flex;
          flex: 1 1 auto;
          flex-direction: column;
          width: 100%;
          overflow: auto;
        }


        .list {
          margin-bottom: 30px;
        }

        .list-title {
          padding-left: 18px;
          font-size: 13px;
          color: #858585;
          line-height: 22px;
          height: 22px;
        }

        .list-item {
          height: 30px;
          font-size: 14px;
        }

        .list-item a {
          height: 35px;
          line-height: 35px;
          padding-left: 18px;
          text-align: left;
          display: block;
          margin-right: 8px;
          display: flex;
          align-items: center;
        }

        .list-item .ellipsis {
            overflow: hidden;
            text-overflow: ellipsis;
            display: -webkit-box;
            -webkit-line-clamp: 1;
            -webkit-box-orient: vertical;
        }

        .list-item a:hover {
          background-color: #1d1d1d;
        }

        .item-icon {
          display: inline-block;
          width: 15px;
          height: 15px;
        }

        .item-icon span {
          display: inline-block;
          width: 15px;
          height: 15px;
        }
    "#,
        sh = SIDER_SUER_HEIGHT.to_owned()
    );

    let list: Vec<Links> = vec![
        Links {
            name: "发现音乐".to_string(),
            to: Route::Home,
        },
        Links {
            name: "播客".to_string(),
            to: Route::TodoList,
        },
        Links {
            name: "私人FM".to_string(),
            to: Route::Home,
        },
        Links {
            name: "视频".to_string(),
            to: Route::Home,
        },
        Links {
            name: "关注".to_string(),
            to: Route::Home,
        },
    ];

    let my_music_menus: Vec<Links> = vec![
        Links {
            name: "我喜欢的音乐".to_string(),
            to: Route::Home,
        },
        Links {
            name: "iTunes音乐".to_string(),
            to: Route::TodoList,
        },
        Links {
            name: "下载管理".to_string(),
            to: Route::Home,
        },
        Links {
            name: "最近播放".to_string(),
            to: Route::Home,
        },
        Links {
            name: "我的音乐云盘".to_string(),
            to: Route::Home,
        },
        Links {
            name: "我的播客".to_string(),
            to: Route::Home,
        },
        Links {
            name: "我的收藏".to_string(),
            to: Route::Home,
        },
    ];

    let create_music_menus: Vec<Links> = vec![Links {
        name: "我喜欢的音乐".to_string(),
        to: Route::Home,
    }];

    html! {
      <div class={style}>
          <div class="user">
          <div class="user-content">{"user"}</div>
          </div>
          <div class="list-wrapper">
          <div class="list">
            {
              list.into_iter().map(|item|{
                html!(
                  <div class="list-item">
                    <Link<Route> to={item.to}>
                    <span class="item-icon"></span>
                    <div class="ellipsis">{item.name}</div>
                    </Link<Route>>
                  </div>
                )
              }).collect::<Html>()
            }
          </div>
          <div class="list-title">{"我的音乐"}</div>
          <div class="list">
          {
            my_music_menus.into_iter().map(|item|{
              html!(
                <div class="list-item">
                  <Link<Route> to={item.to}>
                  <span class="item-icon"></span>
                  <div class="ellipsis">{item.name}</div>
                  </Link<Route>>
                </div>
              )
            }).collect::<Html>()
          }
          </div>
          <div class="list-title">{"创建的歌单"}</div>
          <div class="list">
          {
            create_music_menus.into_iter().map(|item|{
              html!(
                <div class="list-item">
                  <Link<Route> to={item.to}>
                    <span class="item-icon">
                      <span />
                    </span>
                    <div class="ellipsis">
                        {item.name}
                    </div>
                  </Link<Route>>
                </div>
              )
            }).collect::<Html>()
          }
          </div>
        </div>
      </div>
    }
}
