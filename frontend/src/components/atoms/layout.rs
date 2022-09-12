use crate::Route;
use stylist::style;
use stylist::yew::{styled_component, use_style};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutProps {
    pub class: Option<String>,
    pub children: Children,
}

const LAYOUT_HEADER_HEIGHT: &str = "50px";
const LAYOUT_SIDER_WIDTH: &str = "200px";
const LAYOUT_FOOTER_HEIGHT: &str = "56px";

#[styled_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let style = style!(
        r#"
          display: flex;
          flex-direction: column;
          width: 100%;
          height: 100vh;
          background-color: #252525;
          overflow: hidden;
        "#
    )
    .expect("Failed to mount style!");

    html! {<div class={classes!(style, props.class.to_owned())}>{ props.children.clone() }</div>}
}

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutHeaderProps {
    pub class: Option<String>,
}

//
#[styled_component(LayoutHeader)]
pub fn layout_header(props: &LayoutHeaderProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        height: ${hg};
        width: 100%;
        overflow: hidden;
        background-color:  #2d2d2d;
        padding:10px 20px;
        
        ul {
            display: flex;
            align-items: center;
        }

        li {
            height: 40px;
            margin-right: 15px;
            display: inline-flex;
        }

        li > a {
            height: 40px;
            line-height: 24px;
            padding: 8px 10px;
        }

        img {
            width: 34px;
            height: 34px;
            border-radius: 50%;
        }
      "#,
        hg = LAYOUT_HEADER_HEIGHT.to_owned()
    );

    html! {
        <div class={classes!(style, props.class.to_owned())}></div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutContentProps {
    pub class: Option<String>,
    pub children: Children,
}

///
#[styled_component(LayoutContent)]
pub fn layout_content(props: &LayoutContentProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        background-color: #252525;
        flex: 1 1 auto;
        height: calc(100vh - ${hh} - ${fh});
        overflow: auto;
      "#,
        hh = LAYOUT_HEADER_HEIGHT.to_owned(),
        fh = LAYOUT_FOOTER_HEIGHT.to_owned()
    );

    html! {<main class={classes!(style, props.class.to_owned())}>{ props.children.clone() }</main>}
}

#[derive(Properties, PartialEq, Clone)]
pub struct LayoutSiderProps {
    pub class: Option<String>,
    pub children: Children,
}

//
#[styled_component(LayoutSider)]
pub fn layout_sider(props: &LayoutSiderProps) -> Html {
    let style = use_style!(
        r#"
        width: ${sw};
        display: flex;
        background-color:  #202020;
        overflow-y: scroll;
        overflow-x: hidden;

        span, a {
            line-height: 22px;
            height: 22px;
        }
      "#,
        sw = LAYOUT_SIDER_WIDTH.to_owned()
    );

    html! {<sider class={classes!(style, props.class.to_owned())}>{ props.children.clone() }</sider>}
}

///
///
///
///
#[styled_component(LayoutFooter)]
pub fn layout_footer(props: &LayoutHeaderProps) -> Html {
    let style = use_style!(
        r#"
        display: flex;
        height: ${hg};
        background-color: #252525;
        border-top: 1px solid #232323;
        align-items: center;
        justify-content: center;

        span, a {
            line-height: 22px;
            height: 22px;
        }
      "#,
        hg = LAYOUT_FOOTER_HEIGHT.to_owned()
    );

    html! {
        <footer class={classes!(style, props.class.to_owned())}></footer>
    }
}
