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
