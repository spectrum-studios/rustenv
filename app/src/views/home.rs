use yew::prelude::*;

// Home view
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="flex flex-col md:flex-row justify-center items-center rounded-md space-y-10 border-slate-300 dark:border-slate-700 border-bg-slate-100 text-slate-800 dark:text-slate-100 shadow-md mx-10 p-10 dark:bg-slate-900">
            <div class="flex flex-col justify-center text-center space-y-2">
                <h1 class="text-4xl">{"rustenv"}</h1>
                <p>{"Rust application development environment"}</p>
                <p>{"Made by Spectrum Studios"}</p>
            </div>
        </div>
    }
}
