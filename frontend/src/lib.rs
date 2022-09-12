mod components;
mod contexts;
mod pages;
mod utils;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::atoms::layout::{
    Layout, LayoutContent, LayoutFooter, LayoutHeader, LayoutSider,
};

use crate::components::layout::content::{Content, Route};
use crate::components::layout::sider::Sider;

// use crate::contexts::theme::ThemeProvider;

pub fn switch(routes: &Route) -> Html {
    html! {
     <Layout>
       <LayoutHeader />
       <LayoutContent>
          <LayoutSider>
              <Sider />
          </LayoutSider>
          <Content routes={routes.clone()}/>
        </LayoutContent>
       <LayoutFooter />
     </Layout>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
      // <ThemeProvider>
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      // </ThemeProvider>
    }
}
