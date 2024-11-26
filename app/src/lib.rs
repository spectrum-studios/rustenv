mod views;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::home::Home;
use crate::views::not_found::NotFound;

// Application routes
#[derive(Clone, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Switch function
pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! { <Home /> },
        AppRoute::NotFound => html! { <NotFound /> },
    }
}

// Application component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <body class="w-screen h-screen bg-slate-50 dark:bg-slate-800 overflow-hidden">
                <main class="flex justify-center items-center w-full h-full">
                    <Switch<AppRoute> render={switch} />
                </main>
            </body>
        </BrowserRouter>
    }
}
