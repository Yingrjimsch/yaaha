use leptos::html;
use leptos::tachys::html::node_ref::node_ref;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use leptos_meta::Stylesheet;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::components::A;
use leptos_router::components::{ParentRoute, Route, Router, Routes};
use leptos_router::hooks::use_params_map;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

// #[component]
// pub fn App() -> impl IntoView {
//     let (name, set_name) = signal(String::new());
//     let (greet_msg, set_greet_msg) = signal(String::new());

//     let update_name = move |ev| {
//         let v = event_target_value(&ev);
//         set_name.set(v);
//     };

//     let greet = move |ev: SubmitEvent| {
//         ev.prevent_default();
//         spawn_local(async move {
//             let name = name.get_untracked();
//             if name.is_empty() {
//                 return;
//             }

//             let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
//             // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//             let new_msg = invoke("greet", args).await.as_string().unwrap();
//             set_greet_msg.set(new_msg);
//         });
//     };

//     view! {
//         <main class="container">
//             <h1>"Welcome to Tauri + Leptos"</h1>

//             <div class="row">
//                 <a href="https://tauri.app" target="_blank">
//                     <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
//                 </a>
//                 <a href="https://docs.rs/leptos/" target="_blank">
//                     <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
//                 </a>
//             </div>
//             <p>"Click on the Tauri and Leptos logos to learn more."</p>

use leptos::prelude::signal;
//             <form class="row" on:submit=greet>
//                 <input
//                     id="greet-input"
//                     placeholder="Enter a name..."
//                     on:input=update_name
//                 />
//                 <button type="submit">"Greet"</button>
//             </form>
//             <p>{ move || greet_msg.get() }</p>
//         </main>
//     }
// }

// #[component]
// pub fn App() -> impl IntoView {
//     let (count, set_count) = signal(0);
//     provide_meta_context();

//     view! {
//         <Stylesheet id="leptos" href="pkg/yaaha-ui.css"/>
//         <Title text="Leptos + Tailwindcss"/>
//         <main>
//             <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
//                 <div class="flex flex-row-reverse flex-wrap m-auto">
//                     <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg privacy-public-bg text-white">
//                         "Click number " {count}
//                     </button>
//                 </div>
//             </div>
//         </main>
//         // <div>
//         //     <button on:click=move |_| {
//         //         set_count.try_update(|n| *n += 1);
//         //     }>"+1"</button>
//         //     <button on:click=move |_| {
//         //         set_count.update(|n| *n -= 1);
//         //     }>"-1"</button>
//         //     <p class:red=move || count.get() < 0>"Counter: "{count}</p>
//         // </div>
//         // <textarea />
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    view! { <Navbar /> }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="drawer lg:drawer-open">
            <input id="my-drawer" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content">
                // <navbar class="shadow-sm navbar bg-base-100">
                // <label for="my-drawer" class="lg:hidden btn btn-primary drawer-button">
                // <svg
                // xmlns="http://www.w3.org/2000/svg"
                // class="w-6 h-6"
                // fill="none"
                // viewBox="0 0 24 24"
                // stroke="currentColor"
                // >
                // <path
                // stroke-linecap="round"
                // stroke-linejoin="round"
                // stroke-width="2"
                // d="M4 6h16M4 12h16m-7 6h7"
                // />
                // </svg>

                // </label>
                // <a class="text-xl">Yaaha</a>
                // </navbar>
                <div>
                    <Router>
                        <main>
                            // <Routes/> both defines our routes and shows them on the page
                            <Routes fallback=|| "Not found.">
                                // our root route: the contact list is always shown
                                <Route path=path!("") view=ContactList />
                                <Route path=path!("/c/:id") view=Contact />
                                <Route path=path!("/settings") view=Settings />
                            // a fallback if the /:id segment is missing from the URL
                            </Routes>
                        </main>
                    </Router>
                </div>

            </div>
            <div class="drawer-side">
                <label for="my-drawer" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="p-4 w-80 min-h-full menu bg-base-200 text-base-content">
                    <li>
                        <label for="my-drawer" class="lg:hidden btn btn-primary drawer-button">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="w-6 h-6"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M6 18L18 6M6 6l12 12"
                                />
                            </svg>

                        </label>
                    </li>
                    <li>
                        <A href="/c/1">Chat 1</A>
                    </li>
                    <li>
                        <A href="/c/2">Chat 2</A>
                    </li>
                    <li>
                        <A href="/settings">Settings</A>
                    </li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    let (msgs, set_msgs) = signal::<Vec<String>>(vec![]);
    // 2) this is a handle to the contenteditable div
    let editableField: NodeRef<html::Div> = NodeRef::new();

    // 3) the send handler
    // let on_send = move |_| {
    //     // get the div element
    //     if let Some(div) = editable.get() {
    //         leptos::logging::log!("test",);
    //         // grab its textContent (or inner_text)
    //         let text = div.text_content().unwrap_or_default().trim().to_string();

    //         if !text.is_empty() {
    //             // push into our Vec<String> signal
    //             set_msgs.update(|msgs_vec| msgs_vec.push(text));

    //             // clear out the div
    //             div.set_inner_html("");
    //         }
    //     }
    // };
    let on_click = move |_| {
        console::log_1(&"🍋 Button clicked!".into());
        console::log_1(&JsValue::from(editableField.get().clone()));

        if let Some(div) = editableField.get() {
            console::log_1(&JsValue::from(div.clone()));

            // grab its textContent (or inner_text)
            let text = div.text_content().unwrap_or_default().trim().to_string();

            if !text.is_empty() {
                // push into our Vec<String> signal
                set_msgs.update(|msgs_vec| msgs_vec.push(text));

                // clear out the div
                div.set_inner_html("");
            }
        }
    };

    // loads the contact list data once; doesn't reload when nested routes change
    view! {
        <div class="flex flex-col h-screen bg-base-200">
            <div class="flex justify-between items-center p-4 bg-base-300">
                <div>
                    <label for="my-drawer" class="lg:hidden btn btn-primary drawer-button">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>

                    </label>
                </div>
                <div class="flex gap-4">
                    <button class="btn btn-ghost">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-4.418 0-8 2.015-8 4.5v1.5h16v-1.5c0-2.485-3.582-4.5-8-4.5z"
                            />
                        </svg>
                    </button>
                    // <input
                    <div class="join join-horizontal">
                        <input
                            type="radio"
                            name="theme-buttons"
                            class="btn theme-controller join-item"
                            aria-label="Public"
                            value="default"
                            checked
                        />
                        <input
                            type="radio"
                            name="theme-buttons"
                            class="btn theme-controller join-item"
                            aria-label="Private"
                            value="retro"
                        />
                        <input
                            type="radio"
                            name="theme-buttons"
                            class="btn theme-controller join-item"
                            aria-label="Ultra Private"
                            value="cyberpunk"
                        />
                    </div>
                </div>
            </div>

            <div class="overflow-y-auto flex-1 p-4">
                <div class="chat chat-start">
                    <div class="chat-bubble">Hello! How can I help you?</div>
                </div>
                <For each=move || msgs.get() key=|state| state.clone() let(child)>

                    <div class="chat chat-end">
                        <div class="chat-bubble">{child}</div>
                    </div>
                </For>

            </div>

            <div class="p-4">
                <div class="flex overflow-scroll gap-2 items-center w-full textarea textarea-bordered max-h-[10rem]">
                    <div class="w-full outline-none" node_ref=editableField contenteditable="true"></div>
                    <button class="btn btn-circle btn-primary" on:click=on_click>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-linejoin="round"
                            stroke-linecap="round"
                        >
                            <path
                                stroke="currentColor"
                                stroke-width="2"
                                d="M22 2 11 13M22 2l-7 20-4-9M22 2 2 9l9 4"
                            ></path>
                        </svg>
                    </button>

                // <button class="btn btn-primary" on:click=on_click>
                // Send
                // </button>
                // <textarea
                // class="w-full resize-none textarea textarea-bordered [field-sizing:content]"
                // placeholder="Type your message..."
                // ></textarea>
                </div>
            // <button class="btn btn-primary" on:click=on_click>
            // Send
            // </button>
            </div>
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    let params: Memo<params::ParamsMap> = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    view! {
        <div>
            <h1>"Contact Page"</h1>
            // Use the reactive ID
            <p>{move || format!("ID: {}", id())}</p>
        </div>
    }
}

#[component]
fn Settings() -> impl IntoView {
    view! { <div>"Here are the settings"</div> }
}
