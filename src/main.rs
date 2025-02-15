use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::fs;
use std::path::PathBuf;

struct GalacticMarkdown {
    markdown_input: String,
    commonmark_cache: CommonMarkCache,
    current_file: Option<PathBuf>,
    unsaved_changes: bool,
}

impl Default for GalacticMarkdown {
    fn default() -> Self {
        let default_text = "# Welcome to Galactic Markdown

Start typing your markdown here!

## Features
- Live preview
- **Bold** and *italic* text
- `inline code` and code blocks

```rust
fn main() {
    println!(\"Hello, Markdown!\");
}
```

> Edit this text to see the preview update.

### Lists
1. Numbered lists
2. With sub-items
   - Like this
   - And this

### Tables
| Header 1 | Header 2 |
|----------|----------|
| Cell 1    | Cell 2   |
| Cell 3    | Cell 4   |

### Links and Images
[Visit Rust](https://www.rust-lang.org/)

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-128x128.png)
";

        Self {
            markdown_input: String::from(default_text),
            commonmark_cache: CommonMarkCache::default(),
            current_file: None,
            unsaved_changes: false,
        }
    }
}

impl GalacticMarkdown {
    fn new_file(&mut self) {
        if self.unsaved_changes {
            // TODO: Show confirmation dialog
        }
        self.markdown_input.clear();
        self.current_file = None;
        self.unsaved_changes = false;
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown"])
            .pick_file() 
        {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    self.markdown_input = content;
                    self.current_file = Some(path);
                    self.unsaved_changes = false;
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                    // TODO: Show error dialog
                }
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.current_file {
            if let Err(err) = fs::write(path, &self.markdown_input) {
                eprintln!("Error saving file: {}", err);
                // TODO: Show error dialog
            } else {
                self.unsaved_changes = false;
            }
        } else {
            self.save_file_as();
        }
    }

    fn save_file_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md", "markdown"])
            .save_file()
        {
            if let Err(err) = fs::write(&path, &self.markdown_input) {
                eprintln!("Error saving file: {}", err);
                // TODO: Show error dialog
            } else {
                self.current_file = Some(path);
                self.unsaved_changes = false;
            }
        }
    }

    fn show_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    self.new_file();
                    ui.close_menu();
                }
                if ui.button("Open...").clicked() {
                    self.open_file();
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Save").clicked() {
                    self.save_file();
                    ui.close_menu();
                }
                if ui.button("Save As...").clicked() {
                    self.save_file_as();
                    ui.close_menu();
                }
            });

            // Show current file name or "Untitled"
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let file_name = self.current_file
                    .as_ref()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("Untitled");
                let text = if self.unsaved_changes {
                    format!("{} *", file_name)
                } else {
                    file_name.to_string()
                };
                ui.label(text);
            });
        });
    }
}

impl eframe::App for GalacticMarkdown {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let available_width = ctx.screen_rect().width();
        let panel_width = available_width * 0.5;

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.show_menu_bar(ui);
        });

        egui::SidePanel::left("editor_panel")
            .exact_width(panel_width)
            .resizable(false)
            .show(ctx, |ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.markdown_input)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(30)
                    .lock_focus(true);
                
                if ui.add_sized(ui.available_size(), text_edit).changed() {
                    self.unsaved_changes = true;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    CommonMarkViewer::new("preview")
                        .show(ui, &mut self.commonmark_cache, &self.markdown_input);
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Galactic Markdown",
        options,
        Box::new(|_cc| Box::<GalacticMarkdown>::default()),
    )
} 