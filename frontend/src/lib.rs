use std::ops::RangeInclusive;

use gloo_net::http::Request;
use serde::Deserialize;
use web_sys::{FormData, HtmlInputElement};
use yew::prelude::*;

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct UploadResponse {
    image_id: String,
    format: String,
    width: u32,
    height: u32,
    message: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct ConvertResponse {
    ascii_art_id: String,
    ascii_art: String,
    width: u32,
    height: u32,
}

#[function_component(App)]
fn app() -> Html {
    let file_input_ref = use_node_ref();
    let ascii_art = use_state(|| None::<String>);
    let status = use_state(|| None::<String>);
    let is_loading = use_state(|| false);

    let width = use_state(|| 80u32);
    let detail = use_state(|| String::from("high"));
    let contrast = use_state(|| 1.2f32);
    let blur = use_state(|| 0.5f32);

    let on_convert = {
        let file_input_ref = file_input_ref.clone();
        let ascii_art = ascii_art.clone();
        let status = status.clone();
        let is_loading = is_loading.clone();
        let width_state = width.clone();
        let detail_state = detail.clone();
        let contrast_state = contrast.clone();
        let blur_state = blur.clone();

        Callback::from(move |_| {
            let file_input = file_input_ref
                .cast::<HtmlInputElement>()
                .expect("file input ref");
            let files = file_input.files();
            if files.is_none() || files.as_ref().unwrap().length() == 0 {
                status.set(Some("Choose an image".to_string()));
                return;
            }
            let file = files.unwrap().get(0).unwrap();

            let status = status.clone();
            let ascii_art = ascii_art.clone();
            let is_loading = is_loading.clone();
            let width_value = *width_state;
            let detail_value = (*detail_state).clone();
            let contrast_value = *contrast_state;
            let blur_value = *blur_state;

            wasm_bindgen_futures::spawn_local(async move {
                is_loading.set(true);
                status.set(Some("Loading image...".into()));

                let form = FormData::new().expect("formdata");
                form.append_with_blob_and_filename("image", &file, &file.name())
                    .expect("append file");

                let upload_resp = Request::post("/api/upload")
                    .body(form)
                    .expect("failed to set body")
                    .send()
                    .await;

                match upload_resp {
                    Ok(resp) => {
                        if !resp.ok() {
                            status.set(Some(format!("Error loading: {}", resp.status())));
                            is_loading.set(false);
                            return;
                        }
                        let data: UploadResponse = resp.json().await.unwrap();
                        status.set(Some(format!("Loaded: {}x{}", data.width, data.height)));

                        let url = format!(
                            "/api/convert/{}?width={}&detail={}&contrast={}&blur={}",
                            data.image_id, width_value, detail_value, contrast_value, blur_value
                        );
                        status.set(Some("Converting to ASCII...".into()));

                        match Request::post(&url).send().await {
                            Ok(resp2) => {
                                if !resp2.ok() {
                                    status
                                        .set(Some(format!("Error converting: {}", resp2.status())));
                                    is_loading.set(false);
                                    return;
                                }
                                let conv: ConvertResponse = resp2.json().await.unwrap();
                                ascii_art.set(Some(conv.ascii_art));
                                status.set(Some(format!(
                                    "Done! ASCII {}x{}",
                                    conv.width, conv.height
                                )));
                                is_loading.set(false);
                            }
                            Err(e) => {
                                status.set(Some(format!("Network error: {}", e)));
                                is_loading.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        status.set(Some(format!("Network error: {}", e)));
                        is_loading.set(false);
                    }
                }
            });
        })
    };

    let on_width_change = {
        let width = width.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<u32>() {
                if RangeInclusive::new(10, 500).contains(&v) {
                    width.set(v);
                }
            }
        })
    };

    let on_detail_change = {
        let detail = detail.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            detail.set(input.value());
        })
    };

    let on_contrast_change = {
        let contrast = contrast.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<f32>() {
                if RangeInclusive::new(0.1, 3.0).contains(&v) {
                    contrast.set(v);
                }
            }
        })
    };

    let on_blur_change = {
        let blur = blur.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<f32>() {
                if RangeInclusive::new(0.0, 5.0).contains(&v) {
                    blur.set(v);
                }
            }
        })
    };

    html! {
        <>
        <div style="
            max-width: 1000px; 
            margin: 0 auto; 
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; 
            padding: 20px;
            color: white;
            flex: 1;
        ">
            <div style="
                background: rgba(255, 255, 255, 0.1);
                backdrop-filter: blur(10px);
                border-radius: 20px;
                padding: 30px;
                box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
                border: 1px solid rgba(255, 255, 255, 0.18);
            ">
                <h1 style="text-align: center; font-size: 2.5em; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);">
                    {"🎨 ASCII Converter (Rust WASM)"}
                </h1>

                <div style="
                    margin: 20px 0;
                    padding: 20px;
                    background: rgba(255, 255, 255, 0.05);
                    border-radius: 15px;
                    border: 2px dashed rgba(255, 255, 255, 0.3);
                ">
                    <label style="display: block; margin-bottom: 10px; font-weight: bold;">
                        {"📁 Choose an image:"}
                    </label>
                    <input
                        type="file"
                        ref={file_input_ref}
                        accept="image/*"
                        style="
                            width: 100%;
                            padding: 12px;
                            border: none;
                            border-radius: 8px;
                            font-size: 16px;
                            box-sizing: border-box;
                        "
                    />
                </div>

                <div style="
                    display: grid; 
                    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); 
                    gap: 15px; 
                    margin-bottom: 20px;
                ">
                    <div>
                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                            {"Width (symbols):"}
                        </label>
                        <input
                            type="number"
                            min="10"
                            max="500"
                            value={width.to_string()}
                            oninput={on_width_change}
                            style="
                                width: 100%;
                                padding: 12px;
                                border: none;
                                border-radius: 8px;
                                font-size: 16px;
                                box-sizing: border-box;
                            "
                        />
                    </div>
                    <div>
                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                            {"Detail:"}
                        </label>
                        <select
                            value={(*detail).clone()}
                            onchange={on_detail_change}
                            style="
                                width: 100%;
                                padding: 12px;
                                border: none;
                                border-radius: 8px;
                                font-size: 16px;
                                box-sizing: border-box;
                            "
                        >
                            <option value="high">{"High"}</option>
                            <option value="low">{"Low"}</option>
                        </select>
                    </div>
                    <div>
                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                            {"Contrast:"}
                        </label>
                        <input
                            type="number"
                            step="0.1"
                            min="0.1"
                            max="3.0"
                            value={contrast.to_string()}
                            oninput={on_contrast_change}
                            style="
                                width: 100%;
                                padding: 12px;
                                border: none;
                                border-radius: 8px;
                                font-size: 16px;
                                box-sizing: border-box;
                            "
                        />
                    </div>
                    <div>
                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">
                            {"Blur:"}
                        </label>
                        <input
                            type="number"
                            step="0.1"
                            min="0.0"
                            max="5.0"
                            value={blur.to_string()}
                            oninput={on_blur_change}
                            style="
                                width: 100%;
                                padding: 12px;
                                border: none;
                                border-radius: 8px;
                                font-size: 16px;
                                box-sizing: border-box;
                            "
                        />
                    </div>
                </div>

                <div style="margin: 20px 0;">
                    <button
                        onclick={on_convert}
                        disabled={*is_loading}
                        style="
                            width: 100%;
                            padding: 15px;
                            border: none;
                            border-radius: 8px;
                            font-size: 18px;
                            font-weight: bold;
                            text-transform: uppercase;
                            letter-spacing: 1px;
                            background: linear-gradient(45deg, #FF6B6B, #4ECDC4);
                            color: white;
                            cursor: pointer;
                            transition: transform 0.2s, box-shadow 0.2s;
                        "
                    >
                        if *is_loading {
                            {"🔄 Processing..."}
                        } else {
                            {"🚀 Convert to ASCII"}
                        }
                    </button>
                </div>

                if let Some(s) = &*status {
                    <div style="
                        padding: 15px;
                        background: rgba(0, 255, 0, 0.2);
                        border-left: 4px solid #44ff44;
                        border-radius: 5px;
                        margin: 15px 0;
                    ">
                        {s.clone()}
                    </div>
                }

                if let Some(a) = &*ascii_art {
                    <div style="
                        margin-top: 20px;
                        padding: 20px;
                        background: rgba(0, 0, 0, 0.3);
                        border-radius: 10px;
                        border-left: 4px solid #4ECDC4;
                    ">
                        <h3>{"✨ ASCII Art Result:"}</h3>
                        <pre style="
                            white-space: pre;
                            font-family: 'Courier New', monospace;
                            font-size: 8px;
                            line-height: 1;
                            background: black;
                            color: #00ff00;
                            padding: 15px;
                            border-radius: 5px;
                            overflow: auto;
                            max-height: 600px;
                            border: 1px solid #00ff00;
                        ">{a.clone()}</pre>
                    </div>
                }
            </div>
        </div>
        
        <div style="
            width: 100%;
            max-width: 1000px;
            margin: 20px auto 0 auto;
            padding: 20px;
            text-align: center;
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            border-radius: 15px;
            border: 1px solid rgba(255, 255, 255, 0.2);
            font-size: 14px;
            color: rgba(255, 255, 255, 0.9);
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
        ">
            {"Made with ❤️ and 🦀 by "}
            <a 
                href="https://github.com/0xataru" 
                target="_blank" 
                style="
                    color: #4ECDC4;
                    text-decoration: none;
                    font-weight: bold;
                    transition: color 0.2s ease;
                "
            >
                {"Ataru"}
            </a>
        </div>
        </>
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    yew::Renderer::<App>::new().render();
}

