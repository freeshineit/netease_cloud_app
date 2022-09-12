use gloo::{console, timers::callback::Interval};
use stylist::{style, yew::use_style};
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct CarouselItem {
    pub link: String,
    pub picture: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CarouselProps {
    pub list: Vec<CarouselItem>,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let style = use_style!(
        r#"
          height: 200px;
          position: relative;
          background-color: transparent;

          .item-wrapper {
            height: 200px;
            position: absolute;
            text-align: center;
            width: 100%;
            display: flex;
            justify-content: center;
            transition: transform 0.4s ease, z-index 0.3s ease, opacity 0.5s ease;
            z-index: 1;
            opacity: 0;
            transform: translate(0px, 0px) scale(1);
            transform-origin:center;
          }

          .item-wrapper.active {
            left:0;
            right: 0;
            z-index: 6;
            opacity: 1;
            transform: translate(0px, 0px) scale(1);
          }

          .item-wrapper.active_prev {
            z-index: 2;
            opacity: 1;
            transform: translate(-218px, 0px) scale(0.8);
          }

          .item-wrapper.active_next {
            transform: translate(218px, 0px) scale(0.8);
            z-index: 2;
            opacity: 1;
          }

          .item {
            height: 200px;
            width: 540px;
            position: relative;
            border-radius: 8px;
            color: #fff;
          }

          .item img{
            height: 200px;
            width: 540px;
            outline: none;
            font-size: 0;
          }

          .item-wrapper.active_prev .item-content, .item-wrapper.active_next .item-content {
              position: relative;
              height: 200px;
              width: 540px;
          }

          .mask {
            position: absolute;
            z-index: 4;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: rgba(0,0,0,.5)
          }

           .prev, .next {
              position: absolute;
              z-index: 10;
              top: 0;
              bottom: 0;
              width: 100px;
              font-size: 20px;
              color: #fff;
              cursor: pointer;
              display: flex;
              align-items: center;
              justify-content: center;
          }
  
            .prev {
              left: 0;
            }
  
            .next {
              right: 0;
            }

        "#
    );

    let len = props.list.clone().len();

    let list = props.list.clone();

    let active = use_state(|| 0);

    let handle_click_left = {
        let active = active.clone();

        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            if *active == 0 {
                active.set(len - 1);
            } else {
                active.set(*active - 1);
            }
        })
    };
    let handle_click_right = {
        let active = active.clone();

        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            if *active == len - 1 {
                active.set(0);
            } else {
                active.set(*active + 1);
            }
        })
    };

    html!(
      <div class={style}>
          {
            list.iter().enumerate().map(|(idx, entry)| {

              let active = *active.clone();

              let active_prev_class = if  active == 0 && idx == len - 1 || active != 0 && idx  == active - 1 {
                Some("active_prev")
              } else {
                  None
              };

              let active_class = if idx == active {
                Some("active")
              } else {
                  None
              };

              let active_next_class = if idx == active + 1 || active == len -1 && idx == 0 {
                Some("active_next")
              } else {
                  None
              };

              html!(
                <div class={classes!("item-wrapper", active_class, active_prev_class,  active_next_class)}>
                  <div class="item" >
                    <div class="item-content">
                      <img src={entry.picture.to_owned()} />
                      if idx  != active   {
                        <div class="mask" />
                      }
                    </div>
                  </div>
                </div>
              )
            }).collect::<Html>()
          }
          <div class="prev" onclick={handle_click_left}>{"<"}</div>
          <div class="next" onclick={handle_click_right}>{">"}</div>
      </div>
    )
}
