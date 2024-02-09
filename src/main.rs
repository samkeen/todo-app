mod todo;
use todo::TodoCollection;

use eframe::egui;
use eframe::egui::{CentralPanel, Checkbox, Label, RichText, ScrollArea};

struct TodoApp {
    todo_items: TodoCollection,
}

impl TodoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        // Self::default()
        let mut todo_collection = TodoCollection::new("./todos.json");
        Self {
            todo_items: todo_collection,
        }
    }

    fn render_todo_list(&self, ui: &mut eframe::egui::Ui){
        for mut t in self.todo_items.get(){
            let headline =
                Label::new(RichText::new(&t.headline).text_style(egui::TextStyle::Button));
            let description = Label::new(RichText::new(&t.description).text_style(egui::TextStyle::Button));
            let is_complete = Checkbox::new(&mut t.is_complete, "complete");
            ui.add(headline);
            ui.add(description);
            ui.add(is_complete);
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
