use yew::prelude::*;

// Not found view
#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex flex-col justify-center items-center space-y-4">
            <p class="text-8xl text-slate-200 pb-6">{"404"}</p>
            <p class="text-2xl text-slate-200">{"Page not found :("}</p>
        </div>
    }
}
