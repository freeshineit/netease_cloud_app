use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::not_found::NotFound;
use crate::pages::todo_list::TodoList;
use crate::pages::videos::Videos;

// root route
#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/todo_list")]
    TodoList,
    #[at("/videos")]
    Videos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ContentProps {
    pub routes: Route,
}

#[function_component(Content)]
pub fn content(props: &ContentProps) -> Html {
    let style = style!(
        r#"
          display: flex;
          flex-direction: column;
          flex: 1 1 auto;
          overflow-y: scroll;
          overflow-x: hidden;
        "#
    )
    .expect("Failed to mount style!");

    html! {
      <div class={style}>
      {
        match props.routes {
          Route::Home => html! {
            <Home />
          },
          Route::Login => html! {
            <Login />
          },
          Route::TodoList => html! {
            <TodoList />
          },
          Route::Videos => html! {
            <Videos />
          },
          Route::NotFound => html! {<NotFound /> },
        }
      }
      </div>
    }
}
