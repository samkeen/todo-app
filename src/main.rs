use eframe::egui;
use eframe::egui::{CentralPanel, Label, RichText, ScrollArea};

struct Todos     {
    todo_items: Vec<Todo>,
}

struct Todo {
    headline: String,
    description: String,
    is_complete: bool,
}
#[derive(Default)]
struct TodoApp {
    todo_items: Vec<Todo>,
}

impl TodoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        // Self::default()
        let iter = (0..20).map(|a| Todo {
            headline: format!("title-{}", a),
            description: format!("desc...{}", a),
            is_complete: a % 2 == 0,
        });
        Self {
            todo_items: Vec::from_iter(iter),
        }
    }

    fn render_todo_list(&self, ui: &mut eframe::egui::Ui){
        for t in &self.todo_items{
            let headline =
                Label::new(RichText::new(&t.headline).text_style(eframe::egui::TextStyle::Button));
            ui.add(headline);
        }
    }
}

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Todo App",
        native_options,
        Box::new(|cc| Box::new(TodoApp::new(cc))),
    ).expect("Failed to start app");
}

impl eframe::App for TodoApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todo");
            let mut todo_desc: String = String::from("");
            ui.text_edit_multiline(&mut todo_desc);
            ScrollArea::vertical().show(ui, |ui| {
                self.render_todo_list(ui);
            });
        });
    }
}
