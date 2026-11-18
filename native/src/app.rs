use std::time::{Duration, Instant};

use eframe::egui::{self, Align, Color32, FontId, RichText, Sense, Stroke, Vec2};

use crate::model::{load_checklist, Task};
use crate::platform::{apply_backdrop, launch_target, log_error, play_ping};
use crate::routines::{routine_title, RoutineKind, Screen, ROUTINES};
use crate::ui::{
    configure_fonts, configure_style, make_job, style_launcher, style_quick_job, CREAM, GREEN,
    YELLOW_GREEN,
};

const OVERLAY_WIDTH: f32 = 380.0;
const OVERLAY_HEIGHT: f32 = 220.0;
const OVERLAY_MARGIN: f32 = 24.0;

pub struct Genesis {
    screen: Screen,
    tasks: Vec<Task>,
    index: usize,
    waiting: bool,
    wait_remaining: f32,
    wait_last: Instant,
    ready: bool,
    completed: bool,
    advance_at: Option<Instant>,
    qj_input: String,
    focus_qj: bool,
    routine_name: String,
}

impl Genesis {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_style(&cc.egui_ctx);
        apply_backdrop(cc);

        Self {
            screen: Screen::Launcher,
            tasks: Vec::new(),
            index: 0,
            waiting: false,
            wait_remaining: 0.0,
            wait_last: Instant::now(),
            ready: false,
            completed: false,
            advance_at: None,
            qj_input: String::new(),
            focus_qj: false,
            routine_name: String::new(),
        }
    }

    fn close(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    fn enter_checklist(&mut self, ctx: &egui::Context, filename: &str) {
        let screen = ctx.screen_rect().size();
        self.tasks = load_checklist(filename);
        self.index = 0;
        self.routine_name = routine_title(filename);
        self.init_current();
        self.screen = Screen::Checklist;

        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
            OVERLAY_WIDTH,
            OVERLAY_HEIGHT,
        )));
        ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
            egui::WindowLevel::AlwaysOnTop,
        ));
        let x = (screen.x - OVERLAY_WIDTH - OVERLAY_MARGIN).max(OVERLAY_MARGIN);
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
            x,
            OVERLAY_MARGIN,
        )));

        if self.tasks.is_empty() {
            self.close(ctx);
        }
    }

    fn enter_quick_job(&mut self, ctx: &egui::Context) {
        self.screen = Screen::QuickJob;
        self.qj_input.clear();
        self.focus_qj = true;

        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(false));
        ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(true));
    }

    fn init_current(&mut self) {
        self.completed = false;
        self.advance_at = None;
        self.ready = false;
        self.waiting = false;
        self.wait_remaining = 0.0;

        if let Some(task) = self.tasks.get(self.index) {
            if task.kind == "W" {
                let duration = task.duration.unwrap_or(0.0).max(0.0) as f32;
                self.wait_remaining = duration;
                self.wait_last = Instant::now();
                self.waiting = duration > 0.0;
                self.ready = !self.waiting;
            } else {
                self.ready = true;
            }
        }
    }

    fn tick(&mut self, ctx: &egui::Context) {
        if self.screen != Screen::Checklist {
            return;
        }

        if self.waiting {
            let now = Instant::now();
            self.wait_remaining -= now.duration_since(self.wait_last).as_secs_f32();
            self.wait_last = now;

            if self.wait_remaining <= 0.0 {
                self.wait_remaining = 0.0;
                self.waiting = false;
                self.ready = true;
                play_ping();
            }
            ctx.request_repaint_after(Duration::from_millis(50));
        }

        if let Some(at) = self.advance_at {
            if Instant::now() >= at {
                self.advance_at = None;
                let launched = self
                    .tasks
                    .get(self.index)
                    .map(|task| task.kind == "L")
                    .unwrap_or(false);
                if launched {
                    play_ping();
                }

                self.index += 1;
                if self.index >= self.tasks.len() {
                    self.close(ctx);
                } else {
                    self.init_current();
                }
            } else {
                ctx.request_repaint_after(Duration::from_millis(50));
            }
        }
    }

    fn on_task_click(&mut self, ctx: &egui::Context) {
        if !self.ready || self.completed || self.advance_at.is_some() {
            return;
        }

        let Some(task) = self.tasks.get(self.index) else {
            self.close(ctx);
            return;
        };

        self.completed = true;
        self.ready = false;

        if task.kind == "L" {
            if let Some(target) = task.target.as_deref() {
                launch_target(target);
            } else {
                log_error("Launch task is missing a target");
            }
            self.advance_at = Some(Instant::now() + Duration::from_secs(5));
        } else {
            self.advance_at = Some(Instant::now() + Duration::from_millis(300));
        }
    }

    fn task_display(&self, task: &Task) -> (String, String, bool) {
        let info = if self.completed {
            if task.kind == "L" {
                "Launching... wait 5s".to_owned()
            } else {
                String::new()
            }
        } else if self.waiting {
            format!("Wait {}s...", self.wait_remaining.ceil().max(0.0) as i64)
        } else if task.kind == "W" {
            "Ready".to_owned()
        } else if task.kind == "L" {
            "Click to launch".to_owned()
        } else {
            "Click to complete".to_owned()
        };

        (task.text.clone(), info, self.completed)
    }

    fn launcher(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(CREAM))
            .show(ctx, |ui| {
                style_launcher(ui);

                let available = ui.available_height();
                let total = 3.0 * 58.0 + 2.0 * 12.0;

                ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                    ui.add_space(((available - total) * 0.5).max(0.0));
                    ui.spacing_mut().item_spacing = Vec2::new(0.0, 12.0);

                    for routine in ROUTINES.iter() {
                        let button = egui::Button::new(RichText::new(routine.name).size(24.0));
                        if ui.add_sized([300.0, 58.0], button).clicked() {
                            match routine.kind {
                                RoutineKind::Checklist(file) => self.enter_checklist(ctx, file),
                                RoutineKind::Quit => self.close(ctx),
                                RoutineKind::QuickJob => self.enter_quick_job(ctx),
                            }
                        }
                    }
                });
            });
    }

    fn checklist(&mut self, ctx: &egui::Context) {
        let frame = egui::Frame::none()
            .fill(Color32::from_rgba_unmultiplied(10, 14, 10, 140))
            .stroke(Stroke::new(
                1.0_f32,
                Color32::from_rgba_unmultiplied(255, 255, 255, 18),
            ))
            .rounding(egui::Rounding::same(14.0))
            .inner_margin(egui::Margin::ZERO);

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let rect = ui.max_rect();
            let content = rect.shrink(16.0);
            let response = ui.interact(
                rect,
                egui::Id::new("genesis_task_area"),
                Sense::click_and_drag(),
            );

            if response.drag_started() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            if response.dragged() {
                ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            } else if self.ready && !self.completed {
                ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
            } else if response.hovered() {
                ctx.set_cursor_icon(egui::CursorIcon::Grab);
            }

            if response.clicked() {
                self.on_task_click(ctx);
            }

            let Some(task) = self.tasks.get(self.index) else {
                return;
            };

            let accent = Color32::from_rgba_unmultiplied(167, 201, 87, 200);
            let painter = ui.painter();
            painter.text(
                content.left_top(),
                egui::Align2::LEFT_TOP,
                &self.routine_name,
                FontId::proportional(13.0),
                accent,
            );
            let progress = format!(
                "{}/{}",
                (self.index + 1).min(self.tasks.len()),
                self.tasks.len()
            );
            painter.text(
                content.right_top(),
                egui::Align2::RIGHT_TOP,
                progress,
                FontId::proportional(13.0),
                accent,
            );
            for offset in -1..=1 {
                painter.circle_filled(
                    egui::pos2(
                        content.center().x + offset as f32 * 6.0,
                        content.top() + 1.0,
                    ),
                    1.4,
                    Color32::from_rgba_unmultiplied(255, 255, 255, 60),
                );
            }

            let (text, info, strike) = self.task_display(task);

            let text_color = if strike { YELLOW_GREEN } else { CREAM };
            let text_job = make_job(&text, 24.0, text_color, strike, content.width());
            let text_galley = ctx.fonts(|fonts| fonts.layout_job(text_job));

            let info_galley = if info.is_empty() {
                None
            } else {
                let job = make_job(&info, 15.0, YELLOW_GREEN, false, content.width());
                Some(ctx.fonts(|fonts| fonts.layout_job(job)))
            };

            let mut total = text_galley.size().y;
            if let Some(galley) = &info_galley {
                total += 8.0 + galley.size().y;
            }

            let content_top = content.top() + 24.0;
            let content_center_y = (content_top + content.bottom()) * 0.5;
            let mut y = content_center_y - total * 0.5;
            painter.galley(
                egui::pos2(content.center().x, y),
                text_galley.clone(),
                text_color,
            );
            y += text_galley.size().y;

            if let Some(galley) = info_galley {
                y += 8.0;
                painter.galley(egui::pos2(content.center().x, y), galley, YELLOW_GREEN);
            }

            let done = self.index + if self.completed { 1 } else { 0 };
            let fraction = done as f32 / self.tasks.len().max(1) as f32;
            let bar = egui::Rect::from_min_size(
                egui::pos2(content.left(), content.bottom() - 1.5),
                Vec2::new(content.width(), 1.5),
            );
            painter.rect_filled(
                bar,
                egui::Rounding::same(1.0),
                Color32::from_rgba_unmultiplied(255, 255, 255, 22),
            );
            if fraction > 0.0 {
                let filled = egui::Rect::from_min_size(
                    bar.min,
                    Vec2::new(bar.width() * fraction, bar.height()),
                );
                painter.rect_filled(filled, egui::Rounding::same(1.0), YELLOW_GREEN);
            }
        });
    }

    fn quick_job(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(CREAM))
            .show(ctx, |ui| {
                style_quick_job(ui);

                let available = ui.available_height();
                let total = 48.0 + 32.0 + 96.0;

                ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                    ui.add_space(((available - total) * 0.5).max(0.0));
                    ui.label(
                        RichText::new("How long will this take?")
                            .size(48.0)
                            .color(GREEN),
                    );
                    ui.add_space(32.0);

                    ui.allocate_ui_with_layout(
                        Vec2::new(360.0, 96.0),
                        egui::Layout::left_to_right(Align::Center),
                        |ui| {
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.qj_input)
                                    .font(FontId::proportional(64.0))
                                    .desired_width(150.0)
                                    .horizontal_align(Align::Center)
                                    .frame(false),
                            );

                            if self.focus_qj {
                                response.request_focus();
                                self.focus_qj = false;
                            }
                            if response.changed() {
                                self.qj_input.retain(|ch| ch.is_ascii_digit());
                            }

                            let rect = response.rect;
                            ui.painter().line_segment(
                                [
                                    egui::pos2(rect.left(), rect.bottom() + 2.0),
                                    egui::pos2(rect.right(), rect.bottom() + 2.0),
                                ],
                                Stroke::new(4.0_f32, GREEN),
                            );

                            ui.add_space(16.0);
                            ui.label(RichText::new("minutes").size(64.0).color(GREEN));
                        },
                    );

                    if ui.input(|input| input.key_pressed(egui::Key::Enter))
                        && !self.qj_input.is_empty()
                    {
                        self.close(ctx);
                    }
                });
            });
    }
}

impl eframe::App for Genesis {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick(ctx);

        match self.screen {
            Screen::Launcher => self.launcher(ctx),
            Screen::Checklist => self.checklist(ctx),
            Screen::QuickJob => self.quick_job(ctx),
        }
    }
}
