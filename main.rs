use eframe::egui;
use egui::{
    Align, Color32, FontId, Frame, Layout, Margin, RichText, Stroke, Vec2,
    CornerRadius,
};

const BG:         Color32 = Color32::from_rgb(13,  17,  27);
const SURFACE:    Color32 = Color32::from_rgb(22,  29,  45);
const SURFACE2:   Color32 = Color32::from_rgb(30,  39,  58);
const BORDER:     Color32 = Color32::from_rgb(45,  58,  82);
const ACCENT:     Color32 = Color32::from_rgb(56, 189, 248);
const ACCENT_DIM: Color32 = Color32::from_rgb(30,  90, 130);
const TEXT:       Color32 = Color32::from_rgb(226, 232, 240);
const TEXT_DIM:   Color32 = Color32::from_rgb(100, 116, 139);
const GOLD:       Color32 = Color32::from_rgb(251, 191,  36);

fn cr(r: u8) -> CornerRadius { CornerRadius::same(r) }

#[derive(Debug, Clone, PartialEq, Default)]
enum Status { Playing, Completed, Dropped, #[default] Backlog }

impl Status {
    fn label(&self) -> &str {
        match self { Status::Playing=>"Playing", Status::Completed=>"Completed",
                     Status::Dropped=>"Dropped", Status::Backlog=>"Backlog" }
    }
    fn icon(&self) -> &str {
        match self { Status::Playing=>"▶", Status::Completed=>"✓",
                     Status::Dropped=>"✕", Status::Backlog=>"◉" }
    }
    fn color(&self) -> Color32 {
        match self {
            Status::Playing   => Color32::from_rgb(52, 211, 153),
            Status::Completed => Color32::from_rgb(56, 189, 248),
            Status::Dropped   => Color32::from_rgb(248, 113, 113),
            Status::Backlog   => Color32::from_rgb(148, 163, 184),
        }
    }
    fn all() -> &'static [Status] {
        &[Status::Backlog, Status::Playing, Status::Completed, Status::Dropped]
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: usize, title: String, genre: String,
    status: Status, rating: u8, notes: String,
    pinned: bool, date_added: String,
}

#[derive(Default)]
struct App {
    games: Vec<Game>, next_id: usize,
    f_title: String, f_genre: String, f_status: Status,
    f_rating: u8, f_notes: String, f_error: String,
    editing: Option<usize>, show_form: bool,
    tab: Option<Status>, search: String,
    del_id: Option<usize>, focus_title: bool,
}

impl App {
    fn with_sample_data() -> Self {
        let mut a = App::default();
        let s: &[(&str,&str,Status,u8,&str,bool)] = &[
            ("Elden Ring",      "Action RPG",   Status::Completed,10,"Absolute masterpiece. 100h+ loved every second.",true),
            ("Hollow Knight",   "Metroidvania", Status::Playing,   0,"Currently in Deepnest. Send help.",false),
            ("Hades",           "Roguelite",    Status::Completed,10,"Peak game design. Cleared 32 heat.",true),
            ("Stardew Valley",  "Simulation",   Status::Completed, 9,"Perfection unlocked on year 4.",false),
            ("Cyberpunk 2077",  "RPG",          Status::Dropped,   6,"Lost interest after act 1.",false),
            ("Baldur's Gate 3", "RPG",          Status::Backlog,   0,"Everyone says this is a must play.",false),
            ("Sekiro",          "Action",       Status::Backlog,   0,"Dreading it but it's on the list.",false),
            ("Disco Elysium",   "RPG",          Status::Completed, 9,"One of the best stories in any game.",false),
        ];
        for (title,genre,status,rating,notes,pinned) in s {
            a.games.push(Game {
                id: a.next_id, title: title.to_string(), genre: genre.to_string(),
                status: status.clone(), rating: *rating, notes: notes.to_string(),
                pinned: *pinned, date_added: "Mar 2026".to_string(),
            });
            a.next_id += 1;
        }
        a
    }

    fn reset_form(&mut self) {
        self.f_title.clear(); self.f_genre.clear();
        self.f_status = Status::Backlog; self.f_rating = 0;
        self.f_notes.clear(); self.f_error.clear();
        self.editing = None;
    }

    fn open_add(&mut self) { self.reset_form(); self.show_form = true; self.focus_title = true; }

    fn load_edit(&mut self, g: &Game) {
        self.f_title = g.title.clone(); self.f_genre = g.genre.clone();
        self.f_status = g.status.clone(); self.f_rating = g.rating;
        self.f_notes = g.notes.clone(); self.f_error.clear();
        self.editing = Some(g.id); self.show_form = true;
    }

    fn save(&mut self) {
        if self.f_title.trim().is_empty() { self.f_error = "Title can't be empty".into(); return; }
        if let Some(id) = self.editing {
            if let Some(g) = self.games.iter_mut().find(|g| g.id == id) {
                g.title = self.f_title.trim().to_string(); g.genre = self.f_genre.trim().to_string();
                g.status = self.f_status.clone(); g.rating = self.f_rating;
                g.notes = self.f_notes.trim().to_string();
            }
        } else {
            self.games.push(Game {
                id: self.next_id, title: self.f_title.trim().to_string(),
                genre: self.f_genre.trim().to_string(), status: self.f_status.clone(),
                rating: self.f_rating, notes: self.f_notes.trim().to_string(),
                pinned: false, date_added: "Mar 2026".to_string(),
            });
            self.next_id += 1;
        }
        self.reset_form(); self.show_form = false;
    }

    fn visible_games(&self) -> Vec<usize> {
        let q = self.search.to_lowercase();
        let mut pinned = vec![]; let mut rest = vec![];
        for g in &self.games {
            let tab_ok = self.tab.as_ref().map(|t| t == &g.status).unwrap_or(true);
            let srch_ok = q.is_empty() || g.title.to_lowercase().contains(&q)
                || g.genre.to_lowercase().contains(&q) || g.notes.to_lowercase().contains(&q);
            if tab_ok && srch_ok { if g.pinned { pinned.push(g.id); } else { rest.push(g.id); } }
        }
        pinned.extend(rest); pinned
    }

    fn stat_count(&self, s: &Status) -> usize { self.games.iter().filter(|g| &g.status == s).count() }
}

fn main() -> eframe::Result<()> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 750.0])
            .with_min_inner_size([900.0, 550.0])
            .with_title("Game Backlog"),
        ..Default::default()
    };
    eframe::run_native("Game Backlog", opts,
        Box::new(|_cc| Ok(Box::new(App::with_sample_data()))))
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // scale up everything by 1.2x
        ctx.set_pixels_per_point(1.2);

        let mut vis = egui::Visuals::dark();
        vis.panel_fill       = BG;
        vis.window_fill      = SURFACE;
        vis.faint_bg_color   = SURFACE2;
        vis.extreme_bg_color = BG;
        vis.widgets.noninteractive.bg_fill   = SURFACE;
        vis.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_DIM);
        vis.widgets.inactive.bg_fill         = SURFACE2;
        vis.widgets.inactive.fg_stroke       = Stroke::new(1.0, TEXT_DIM);
        vis.widgets.hovered.bg_fill          = SURFACE2;
        vis.widgets.hovered.fg_stroke        = Stroke::new(1.5, ACCENT);
        vis.widgets.active.bg_fill           = ACCENT_DIM;
        vis.widgets.active.fg_stroke         = Stroke::new(1.5, ACCENT);
        vis.selection.bg_fill                = ACCENT_DIM;
        vis.selection.stroke                 = Stroke::new(1.0, ACCENT);
        vis.window_corner_radius             = cr(10);
        vis.window_stroke                    = Stroke::new(1.0, BORDER);
        ctx.set_visuals(vis);

        if ctx.input(|i| i.key_pressed(egui::Key::N)) && !self.show_form && !ctx.wants_keyboard_input() {
            self.open_add();
        }

        // ── delete confirm modal ──
        if let Some(did) = self.del_id {
            let title = self.games.iter().find(|g| g.id == did).map(|g| g.title.clone()).unwrap_or_default();
            egui::Window::new("Confirm Delete")
                .collapsible(false).resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .frame(Frame::new().fill(SURFACE).stroke(Stroke::new(1.0, BORDER))
                    .inner_margin(Margin::same(24)).corner_radius(cr(12)))
                .show(ctx, |ui| {
                    ui.label(RichText::new(format!("Delete \"{}\"?", title)).color(TEXT).size(16.0));
                    ui.add_space(6.0);
                    ui.label(RichText::new("This can't be undone.").color(TEXT_DIM).size(13.0));
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        if ui.add(egui::Button::new(RichText::new("   Delete   ").color(Color32::WHITE).size(14.0))
                            .fill(Color32::from_rgb(185,40,40)).corner_radius(cr(7))
                            .min_size(Vec2::new(100.0, 36.0))).clicked()
                        {
                            self.games.retain(|g| g.id != did);
                            if self.editing == Some(did) { self.reset_form(); self.show_form = false; }
                            self.del_id = None;
                        }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(RichText::new("   Cancel   ").color(TEXT).size(14.0))
                            .fill(SURFACE2).corner_radius(cr(7))
                            .min_size(Vec2::new(100.0, 36.0))).clicked()
                        { self.del_id = None; }
                    });
                });
        }

        // ── add / edit modal ──
        if self.show_form {
            let heading = if self.editing.is_some() { "Edit Game" } else { "Add Game" };
            egui::Window::new(heading)
                .collapsible(false).resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .fixed_size([480.0, 500.0])
                .frame(Frame::new().fill(SURFACE).stroke(Stroke::new(1.0, BORDER))
                    .inner_margin(Margin::same(28)).corner_radius(cr(14)))
                .show(ctx, |ui| {
                    ui.visuals_mut().override_text_color = Some(TEXT);
                    ui.label(RichText::new(heading).size(20.0).color(ACCENT).strong());
                    ui.add_space(16.0);

                    ui.label(RichText::new("Title").color(TEXT_DIM).size(13.0));
                    let r = ui.add(egui::TextEdit::singleline(&mut self.f_title)
                        .desired_width(f32::INFINITY).hint_text("e.g. Elden Ring")
                        .font(FontId::proportional(15.0)));
                    if self.focus_title { r.request_focus(); self.focus_title = false; }
                    ui.add_space(12.0);

                    ui.label(RichText::new("Genre").color(TEXT_DIM).size(13.0));
                    ui.add(egui::TextEdit::singleline(&mut self.f_genre)
                        .desired_width(f32::INFINITY).hint_text("e.g. Action RPG")
                        .font(FontId::proportional(15.0)));
                    ui.add_space(12.0);

                    ui.label(RichText::new("Status").color(TEXT_DIM).size(13.0));
                    ui.horizontal(|ui| {
                        for s in Status::all() {
                            let active = &self.f_status == s;
                            if ui.add(egui::Button::new(
                                RichText::new(format!("{} {}", s.icon(), s.label()))
                                    .color(if active { Color32::BLACK } else { s.color() }).size(13.0))
                                .fill(if active { s.color() } else { SURFACE2 })
                                .stroke(Stroke::new(1.0, if active { s.color() } else { BORDER }))
                                .corner_radius(cr(7))
                                .min_size(Vec2::new(0.0, 32.0))).clicked()
                            { self.f_status = s.clone(); }
                        }
                    });
                    ui.add_space(12.0);

                    let rating_label = if self.f_rating == 0 { "Rating  —  unrated".into() }
                                       else { format!("Rating  —  {}/10", self.f_rating) };
                    ui.label(RichText::new(rating_label).color(TEXT_DIM).size(13.0));
                    ui.add(egui::Slider::new(&mut self.f_rating, 0..=10).show_value(false).trailing_fill(true));
                    ui.add_space(12.0);

                    ui.label(RichText::new("Notes").color(TEXT_DIM).size(13.0));
                    ui.add(egui::TextEdit::multiline(&mut self.f_notes)
                        .desired_rows(3).desired_width(f32::INFINITY)
                        .hint_text("Optional notes...").font(FontId::proportional(14.0)));

                    if !self.f_error.is_empty() {
                        ui.add_space(6.0);
                        ui.label(RichText::new(&self.f_error.clone()).color(Color32::from_rgb(248,113,113)).size(13.0));
                    }
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        let lbl = if self.editing.is_some() { "  Save Changes  " } else { "  Add Game  " };
                        if ui.add(egui::Button::new(RichText::new(lbl).color(Color32::BLACK).size(14.0).strong())
                            .fill(ACCENT).corner_radius(cr(8))
                            .min_size(Vec2::new(130.0, 38.0))).clicked()
                        { self.save(); }
                        ui.add_space(10.0);
                        if ui.add(egui::Button::new(RichText::new("  Cancel  ").color(TEXT_DIM).size(14.0))
                            .fill(SURFACE2).corner_radius(cr(8))
                            .min_size(Vec2::new(100.0, 38.0))).clicked()
                        { self.reset_form(); self.show_form = false; }
                    });
                });
        }

        // ── main panel ──
        egui::CentralPanel::default()
            .frame(Frame::new().fill(BG).inner_margin(Margin::same(0)))
            .show(ctx, |ui| {

            // top bar
            Frame::new().fill(SURFACE).stroke(Stroke::new(1.0, BORDER))
                .inner_margin(Margin::symmetric(28, 16))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🎮  BACKLOG").size(20.0).color(ACCENT).strong());
                        ui.add_space(20.0);
                        for s in Status::all() {
                            let count = self.stat_count(s);
                            Frame::new().fill(SURFACE2).stroke(Stroke::new(1.0, BORDER))
                                .corner_radius(cr(20)).inner_margin(Margin::symmetric(12, 5))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        let dot = ui.allocate_exact_size(Vec2::new(10.0,10.0), egui::Sense::hover()).0;
                                        ui.painter().circle_filled(dot.center(), 5.0, s.color());
                                        ui.label(RichText::new(format!("{} {}", s.label(), count)).size(13.0).color(TEXT_DIM));
                                    });
                                });
                            ui.add_space(6.0);
                        }
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui.add(egui::Button::new(
                                RichText::new("  + Add Game  ").color(Color32::BLACK).size(14.0).strong())
                                .fill(ACCENT).corner_radius(cr(8))
                                .min_size(Vec2::new(120.0, 36.0)))
                                .on_hover_text("Press N").clicked()
                            { self.open_add(); }
                            ui.add_space(14.0);
                            ui.add(egui::TextEdit::singleline(&mut self.search)
                                .desired_width(240.0).hint_text("🔍  Search games")
                                .font(FontId::proportional(20.0)));
                        });
                    });
                });

            // tab bar
            Frame::new().fill(SURFACE).inner_margin(Margin::symmetric(28, 0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        let tabs: &[Option<Status>] = &[None,
                            Some(Status::Backlog), Some(Status::Playing),
                            Some(Status::Completed), Some(Status::Dropped)];
                        let labels = ["All","Backlog","Playing","Completed","Dropped"];
                        for (tab, label) in tabs.iter().zip(labels.iter()) {
                            let active = &self.tab == tab;
                            let color = tab.as_ref().map(|t| t.color()).unwrap_or(ACCENT);
                            let resp = ui.add(egui::Button::new(
                                RichText::new(*label).size(14.0).color(if active { color } else { TEXT_DIM }))
                                .fill(Color32::TRANSPARENT).stroke(Stroke::NONE)
                                .corner_radius(CornerRadius::ZERO)
                                .min_size(Vec2::new(0.0, 38.0)));
                            if resp.clicked() { self.tab = tab.clone(); }
                            if active {
                                let r = resp.rect;
                                ui.painter().line_segment(
                                    [egui::pos2(r.min.x, r.max.y), egui::pos2(r.max.x, r.max.y)],
                                    Stroke::new(2.5, color));
                            }
                        }
                    });
                    ui.separator();
                });

            // game cards
            let ids = self.visible_games();

            if ids.is_empty() {
                ui.add_space(80.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("No games found").size(20.0).color(TEXT_DIM));
                    ui.add_space(8.0);
                    ui.label(RichText::new(
                        if self.search.is_empty() { "Press N or click + Add Game to get started".into() }
                        else { format!("No results for \"{}\"", self.search) }
                    ).size(14.0).color(TEXT_DIM));
                });
                return;
            }

            let mut toggle_pin: Option<usize> = None;
            let mut edit_id:    Option<usize> = None;
            let mut delete_id:  Option<usize> = None;
            let mut new_status: Option<(usize, Status)> = None;

            egui::ScrollArea::vertical().auto_shrink([false;2]).show(ui, |ui| {
                ui.add_space(16.0);
                let available = ui.available_width();

                for gid in &ids {
                    let game = match self.games.iter().find(|g| g.id == *gid) {
                        Some(g) => g.clone(), None => continue,
                    };

                    Frame::new().fill(SURFACE).stroke(Stroke::new(1.0, BORDER))
                        .corner_radius(cr(12)).inner_margin(Margin::same(0))
                        .show(ui, |ui| {
                            ui.set_width(available - 40.0);
                            ui.horizontal(|ui| {
                                // ── thick colored left strip ──
                                let (strip, _) = ui.allocate_exact_size(Vec2::new(8.0, 110.0), egui::Sense::hover());
                                ui.painter().rect_filled(strip, CornerRadius::ZERO, game.status.color());
                                ui.add_space(16.0);

                                ui.vertical(|ui| {
                                    ui.add_space(12.0);

                                    // row 1: title + action buttons
                                    ui.horizontal(|ui| {
                                        if game.pinned {
                                            ui.label(RichText::new("📌").size(15.0));
                                            ui.add_space(2.0);
                                        }
                                        ui.label(RichText::new(&game.title).size(17.0).color(TEXT).strong());

                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            ui.add_space(16.0);
                                            // Delete button
                                            if ui.add(egui::Button::new(
                                                RichText::new("🗑").size(16.0).color(Color32::from_rgb(220,80,80)))
                                                .fill(SURFACE2).stroke(Stroke::new(1.0, BORDER))
                                                .corner_radius(cr(6))
                                                .min_size(Vec2::new(34.0, 34.0)))
                                                .on_hover_text("Delete game").clicked()
                                            { delete_id = Some(game.id); }
                                            ui.add_space(6.0);
                                            // Edit button
                                            if ui.add(egui::Button::new(
                                                RichText::new("✏").size(16.0).color(ACCENT))
                                                .fill(SURFACE2).stroke(Stroke::new(1.0, BORDER))
                                                .corner_radius(cr(6))
                                                .min_size(Vec2::new(34.0, 34.0)))
                                                .on_hover_text("Edit game").clicked()
                                            { edit_id = Some(game.id); }
                                            ui.add_space(6.0);
                                            // Pin button
                                            let pin_col = if game.pinned { GOLD } else { TEXT_DIM };
                                            if ui.add(egui::Button::new(
                                                RichText::new("★").size(16.0).color(pin_col))
                                                .fill(SURFACE2).stroke(Stroke::new(1.0, if game.pinned { GOLD } else { BORDER }))
                                                .corner_radius(cr(6))
                                                .min_size(Vec2::new(34.0, 34.0)))
                                                .on_hover_text(if game.pinned {"Unpin"} else {"Pin to top"})
                                                .clicked()
                                            { toggle_pin = Some(game.id); }
                                        });
                                    });

                                    ui.add_space(4.0);

                                    // row 2: genre + date
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(&game.genre).size(13.0).color(TEXT_DIM));
                                        if !game.date_added.is_empty() {
                                            ui.label(RichText::new("  ·  ").color(TEXT_DIM).size(13.0));
                                            ui.label(RichText::new(&game.date_added).size(13.0).color(TEXT_DIM));
                                        }
                                    });

                                    ui.add_space(10.0);

                                    // row 3: status chips + rating
                                    ui.horizontal(|ui| {
                                        for s in Status::all() {
                                            let active = &game.status == s;
                                            if ui.add(egui::Button::new(
                                                RichText::new(format!("{} {}", s.icon(), s.label()))
                                                    .size(12.0).color(if active { Color32::BLACK } else { TEXT_DIM }))
                                                .fill(if active { s.color() } else { SURFACE2 })
                                                .stroke(Stroke::new(1.0, if active { s.color() } else { BORDER }))
                                                .corner_radius(cr(20))
                                                .min_size(Vec2::new(0.0, 28.0)))
                                                .on_hover_text("Quick change status").clicked() && !active
                                            { new_status = Some((game.id, s.clone())); }
                                            ui.add_space(2.0);
                                        }
                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            ui.add_space(16.0);
                                            if game.rating > 0 {
                                                ui.label(RichText::new(format!("★  {}/10", game.rating))
                                                    .size(14.0).color(GOLD).strong());
                                            } else {
                                                ui.label(RichText::new("Not rated").size(13.0).color(TEXT_DIM));
                                            }
                                        });
                                    });

                                    // notes
                                    if !game.notes.is_empty() {
                                        ui.add_space(6.0);
                                        ui.label(RichText::new(&game.notes).size(13.0).color(TEXT_DIM).italics());
                                    }
                                    ui.add_space(12.0);
                                });
                            });
                        });
                    ui.add_space(10.0);
                }

                if let Some(id) = toggle_pin {
                    if let Some(g) = self.games.iter_mut().find(|g| g.id == id) { g.pinned = !g.pinned; }
                }
                if let Some(id) = edit_id {
                    if let Some(g) = self.games.iter().find(|g| g.id == id).cloned() { self.load_edit(&g); }
                }
                if let Some(id) = delete_id { self.del_id = Some(id); }
                if let Some((id, status)) = new_status {
                    if let Some(g) = self.games.iter_mut().find(|g| g.id == id) { g.status = status; }
                }

                ui.add_space(24.0);
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Press  N  to quickly add a game").size(12.0).color(TEXT_DIM));
                });
                ui.add_space(16.0);
            });
        });
    }
}