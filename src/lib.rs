// src/lib.rs
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // Hidden file input
    let input = document
        .get_element_by_id("file-input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()?;

    let process_closure = Closure::<dyn Fn(FileList)>::new(move |files: FileList| {
        process_files(&files);
    });
    input.set_onchange(Some(process_closure.as_ref().unchecked_ref()));
    process_closure.forget();

    // Drag & drop on body
    let drop_closure = Closure::<dyn Fn(DragEvent)>::new(move |e: DragEvent| {
        e.prevent_default();
        if let Some(dt) = e.data_transfer() {
            if let Some(files) = dt.files() {
                process_files(&files);
            }
        }
    });
    let body = document.body().unwrap();
    body.set_ondrop(Some(drop_closure.as_ref().unchecked_ref()));
    body.set_ondragover(Some(drop_closure.as_ref().unchecked_ref()));
    drop_closure.forget();

    log("Ready — drop any file!");
    Ok(())
}

fn process_files(files: &FileList) {
    for i in 0..files.length() {
        let file = files.item(i).unwrap();
        let name = file.name();
        let size = file.size() as u64;
        let file_type: String = file.type_();
        // start
        // ---------- 1. CREATE A BLOB URL ----------
        let object_url =
            Url::create_object_url_with_blob(&file).expect("failed to create object URL");

        log(&format!(
            "File: {name} — {size} bytes — {file_type} — URL: {object_url}"
        ));

        // ---------- 2. (optional) show it in the page ----------
        // we call a tiny JS helper that lives in index.html
        show_file_preview(&object_url, &file_type);
        // end

        log(&format!("File: {name} — {size} bytes — {file_type}"));

        let reader = FileReader::new().unwrap();
        let reader_clone = reader.clone();
        let onload = Closure::<dyn Fn()>::new(move || {
            let array = Uint8Array::new(&reader_clone.result().unwrap());
            let bytes: Vec<u8> = array.to_vec();

            log(&format!("Loaded {} bytes into Rust!", bytes.len()));

            // Quick preview based on type
            if file_type.starts_with("image/") {
                log("Image detected — ready for image crate processing");
            } else if file_type.starts_with("video/") {
                log("Video detected — MP4 parsing possible");
            } else if file_type.contains("text") || name.ends_with(".txt") {
                if let Ok(text) = String::from_utf8(bytes.clone()) {
                    let preview = text.lines().take(3).collect::<Vec<_>>().join(" ");
                    log(&format!("Text preview: {preview}..."));
                }
            }
        });

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.read_as_array_buffer(&file).unwrap();
        onload.forget();
    }
}

// Simple console.log wrapper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // called from Rust → JS
    #[wasm_bindgen(js_namespace = my_image_app)]
    fn show_file_preview(url: &str, mime: &str);
}
