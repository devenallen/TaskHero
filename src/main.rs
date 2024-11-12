#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "TaskHero",
        native_options,
        Box::new(|cc| Ok(Box::new(task_hero::TemplateApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect log messages to the browser console
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();
        let start_result = runner
            .start(
                "the_canvas_id",  // Canvas ID in your HTML
                web_options,
                Box::new(|cc| Ok(Box::new(task_hero::TemplateApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner if successfully started
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
