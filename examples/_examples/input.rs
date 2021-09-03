use dioxus_core as dioxus;
use dioxus_core::prelude::*;
use dioxus_web::WebsysRenderer;
use dioxus_html as dioxus_elements;


fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();

    log::info!("hello world");
    wasm_bindgen_futures::spawn_local(WebsysRenderer::start(App));
}

static App: FC<()> = |cx| {
    let (val, set_val) = use_state_classic(cx, || "asd".to_string());

    cx.render(rsx! {
        div { class: "max-w-lg max-w-xs bg-blue-800 shadow-2xl rounded-lg mx-auto text-center py-12 mt-4 rounded-xl"
            div { class: "container py-5 max-w-md mx-auto"
                h1 { class: "text-gray-200 text-center font-extrabold -mt-3 text-3xl", 
                    "Text Input Example"
                } 
                div { class: "mb-4"
                    input {
                        placeholder: "Username"
                        class: "shadow appearance-none rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        id: "username" 
                        r#type: "text"
                        value: "{val}"
                        oninput: move |evet| set_val(evet.value())
                    }
                    p { "Val is: {val}" }
                }            
            }
        }
    })
};

pub static Example: FC<()> = |cx| {
    cx.render(rsx! {
        div { class: "max-w-lg max-w-xs bg-blue-800 shadow-2xl rounded-lg mx-auto text-center py-12 mt-4 rounded-xl"
            div { class: "container py-5 max-w-md mx-auto"
                h1 { class: "text-gray-200 text-center font-extrabold -mt-3 text-3xl",
                    "Text Input Example"
                }
                UserInput {}
            }
        }
    })
};

static UserInput: FC<()> = |cx| {
    let (val, set_val) = use_state_classic(cx, || "asd".to_string());

    rsx! { in cx,
        div { class: "mb-4"
            input { class: "shadow appearance-none rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                placeholder: "Username"
                id: "username"
                r#type: "text"
                oninput: move |evet| set_val(evet.value())
            }
            p { "Val is: {val}" }
        }
    }
};