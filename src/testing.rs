use std::fs::{File, Metadata};

use leptos::{
    ev::SubmitEvent,
    html::{Input, Object},
    leptos_dom::EventHandler,
    logging::*,
    *,
};
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use web_sys::{FileList, FileReader, HtmlFormElement, HtmlInputElement};

#[component]
pub fn TestingPage() -> impl IntoView {
    let main_style = " bg-bg-1 content-content from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen";

    view! {
        <main class={main_style}>
            <SendForm/>
        </main>
    }
}

#[component]
fn SendForm() -> impl IntoView {
    let (raw_file, set_raw_file) = create_signal(None::<String>);
    let file_input = create_node_ref::<Input>();

    let parse_file = create_server_action::<ParseFile>();
    let value = parse_file.value();

    let submit_file = move |ev: SubmitEvent| {
        ev.prevent_default();
        log!("submit file triggered");

        if let Some(files) = file_input.get().and_then(|f| f.files()) {
            let file = files.get(0);
            if let Some(val) = file {
                let get_file = wasm_bindgen_futures::JsFuture::from(val.array_buffer());

                spawn_local(async move {
                    let res = get_file.await;

                    match res {
                        Err(er) => error!("no array data : {:?}", er),
                        Ok(data) => {
                            log!("{:?}", data);
                        }
                    }
                });
            } else {
                error!("no file");
            }
        }
    };

    let work_file = create_server_action::<ParseFile>();

    let error_msg = move || {
        value.get().map(|v| match v {
            Ok(_) => None,
            Err(er) => Some(er.to_string()),
        })
    };

    view! {
        <ActionForm
            action= parse_file
        >
        <label>"Send a file:"</label>
        <input
            type="file"
            accept="file/txt"
            name="query_file"
        />
        <input type="submit" value="Add" on:submit=submit_file />
        {error_msg}
        </ActionForm>

        <div>
            {value}
        </div>
        <div>
        <form>
            <input
                type="file"
                node_ref=file_input
                on:submit =submit_file
            />
            <input type="submit" value="Submit" on:submit=submit_file />
        </form>
            <p class="bg-blue-500 box">
                {raw_file}
            </p>
        </div>
    }
}

struct Datoteka {
    name: String,
    modified: i64,
    data: Vec<String>,
}

#[server(ParseFile)]
async fn parse_file(query_file: Option<String>) -> Result<String, ServerFnError> {
    // if let Some(file) = query_file {
    //     let output = Command::new("echo").arg(&file).output();
    //     Ok(output)
    // } else {
    //     Err(ServerFnError::Args(
    //         "There was a  problem with the file".to_string(),
    //     ))
    // }
    Ok(format!("{:?}", query_file))
}
