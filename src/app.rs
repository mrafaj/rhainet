use std::sync::mpsc::{Receiver, Sender};
use egui::Response;
use rhai::{Engine, AST};


pub struct RhaiBrowserApp {
    tx: Sender<String>,
    rx: Receiver<String>,
    engine: Engine,

    url: String,
    loaded_page: Option<AST>,
}

fn paragraph(ui: *mut egui::Ui, text: &str) {
    unsafe {
        (*ui).label(text);
    }
}

fn button(ui: *mut egui::Ui, text: &str) -> Response {
    unsafe {
        (*ui).button(text)
    }
}

fn response_clicked(response: &mut Response) -> bool {
    response.clicked()
}

impl Default for RhaiBrowserApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        let mut engine = Engine::new();
        engine.register_type_with_name::<egui::Response>("Response")
              .register_fn("clicked", response_clicked)
              .register_fn("p", paragraph)
              .register_fn("button", button);

        Self {
            tx,
            rx,
            engine: engine,
            url: "http://127.0.0.1:5000/example".to_owned(),
            loaded_page: None,
        }
    }
}

impl RhaiBrowserApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for RhaiBrowserApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let panel = egui::CentralPanel::default();
        panel.show(ctx, |ui| {
            match self.rx.try_recv() {
                Ok(loaded_page) => { self.loaded_page = Some(self.engine.compile(loaded_page).expect("Failed to compile page")) },
                Err(_) => {},
            }

            ui.text_edit_singleline(&mut self.url);
            if ui.button("Load page").clicked() {
                request_page(&self.url, self.tx.clone());
            }
            ui.separator();

            // let shareabl_ui = Arc::new(Mutex::new(ui));
            let shareable_ui: *mut egui::Ui = ui;

            if let Some(loaded_page_ast) = &self.loaded_page {
                let mut scope = rhai::Scope::new();
                scope.push("ui", shareable_ui);
                self.engine.call_fn::<()>(&mut scope, &loaded_page_ast, "update", ()).expect("Failed to render page");
            }
        });
    }
}

fn request_page(url: &str, tx: Sender<String>) {
    let url_clone = url.to_owned();
    tokio::spawn(async move  {
        let page = reqwest::get(url_clone).await.expect("Failed to get page").text().await.expect("Failed to parse page");
        let _ = tx.send(page);
    });
}
