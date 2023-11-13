use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::testing::TestingPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <NavBar/>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
                <Route path="/testing" view=  move || view! { <TestingPage/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let main_style = "bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen";

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class={main_style}>
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button on:click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    let ul = "list-none m-0 p-0 overflow-hidden sticky";
    let li = "float-left hover:bg-blue-800";
    let logo = "float-left p-2 m-0 bg-gray-700 rounded";
    let li_a = "block text-center p-2 no-underline";

    view! {
        <div>
        <ul class={ul}>
            <li class={logo}>"Novartis"</li>
            <li class={li}><A class={li_a} href="/">"Home"</A></li>
            <li class={li}><A class={li_a} href="/testing">"Testing page"</A></li>
            <li class={li}><A class={li_a} href="/api">"API"</A></li>
        </ul>
        </div>
    }
}
