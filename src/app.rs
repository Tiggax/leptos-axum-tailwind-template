use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::testing::TestingPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <NavBar/>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
                <Route path="/testing" view=  move || view! { <TestingPage/> }/>
                <Route path="/description" view=  move || view! { <DescriptionPage/> }/>
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
    let ul = "list-none m-0 p-0 overflow-hidden sticky top-0 rounded-full bg-gray-200";
    let li = "float-left m-2 rounded p-2";
    let li_a = "block text-center p-2 no-underline rounded hover:bg-dbg-info";
    let nav = "m-2 sticky top-2";
    let banner = "m-2 p-2 bg-gray-700 text-3xl rounded text-white";

    view! {
        <h1 class={banner}>"Novartis"</h1>
        <nav class={nav}>

        <ul class={ul}>
            <li class={li}><A class={li_a} href="/"><p class ="fa fa-home">"Home"</p></A></li>
            <li class={li}><A class={li_a} href="/testing"><p class="fa fa-flask">"Testing page"</p></A></li>
            <li class={li}><A class={li_a} href="/api"><p class="fa fa-cloud-download">"API"</p></A></li>
            <li class={li}><A class={li_a} href="/description"><p class="fa fa-note">"Description test page"</p></A></li>
        </ul>
        </nav>
    }
}

#[component]
fn DescriptionPage() -> impl IntoView {
    let main_style = "bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen";

    view! {
        <div class={main_style}>
            <p>
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed enim est, dignissim in tortor sit amet, commodo tristique nunc. Pellentesque consequat augue id dolor consectetur, at efficitur quam congue. Pellentesque tincidunt nunc et nunc efficitur, sed consequat sem convallis. Fusce consequat odio venenatis quam mollis, vel convallis leo tincidunt. Nam vel malesuada magna. Vivamus ac felis finibus, cursus odio quis, porttitor enim. Donec eleifend rhoncus facilisis. "
            </p>
            <p>
                "Integer id ullamcorper nulla. Sed sodales blandit eros sed rhoncus. Duis suscipit enim et lectus luctus, et aliquam quam varius. Cras mauris enim, elementum at purus sit amet, semper posuere massa. Mauris tincidunt tincidunt condimentum. Fusce in viverra libero, a tincidunt enim. Aliquam in lacus fermentum, vestibulum elit non, elementum erat. Proin mollis pretium auctor. Suspendisse suscipit purus lectus, quis porttitor felis tincidunt eu. Praesent luctus pharetra justo vitae ullamcorper. Vivamus blandit mollis dolor, vitae sollicitudin nibh sagittis eget. Donec rhoncus sit amet libero ut euismod. Mauris pharetra a lectus id molestie. Donec hendrerit eros commodo ex faucibus, quis consectetur neque blandit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum eu commodo lectus. "
            </p>
            " Vivamus aliquet interdum ipsum. Nam vitae fringilla nulla. Nam a metus ac quam tincidunt porttitor et vestibulum dui. Etiam justo enim, convallis eu elit vitae, cursus pulvinar arcu. Vivamus dolor ex, semper a lectus ut, fringilla vestibulum eros. Nulla sit amet scelerisque odio. Cras nec lorem tincidunt, imperdiet eros in, porttitor risus. Aliquam nec eros eget nulla porttitor egestas.

Donec ac commodo nibh, in ultricies justo. Nulla sem odio, hendrerit id gravida id, bibendum eu risus. Nunc et massa convallis, suscipit tortor vitae, vehicula est. Vivamus ex ligula, facilisis vitae hendrerit ac, scelerisque ut orci. Proin sagittis tristique urna vel interdum. Nunc ultricies sodales velit a rutrum. Sed mollis egestas diam non laoreet. Aliquam sit amet vestibulum metus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque in elementum mauris. Nulla leo leo, accumsan nec metus ut, sollicitudin porta odio. Nunc laoreet congue enim vel molestie. Nam sed ex et arcu lacinia suscipit sit amet eu est. Aliquam arcu lectus, accumsan sit amet est eget, aliquam porta metus.

Nunc feugiat lorem non maximus ultricies. Praesent orci odio, tristique vel bibendum vel, semper ut sem. Proin vulputate nisl at tellus consectetur, et aliquam lectus venenatis. Pellentesque dignissim nulla non erat tincidunt sagittis. Suspendisse potenti. Cras sit amet lectus bibendum, volutpat augue maximus, efficitur nulla. Cras elementum dui id ligula tempor, et pretium lectus faucibus. Morbi id ullamcorper quam. Pellentesque sodales at diam quis malesuada. Aenean ac leo a mauris semper malesuada. Donec vehicula vitae nisl in tempus. "
        </div>
    }
}
