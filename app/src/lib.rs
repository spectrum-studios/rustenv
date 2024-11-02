use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <body class="flex flex-col space-between w-screen h-screen bg-slate-50 dark:bg-slate-700 overflow-hidden">
            <span>{"Spectrum"}</span>
        </body>
    }
}
