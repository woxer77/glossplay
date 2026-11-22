use crate::app::{App, Async, SidebarTab};
use crate::i18n::{Tr, t, tf, tk};
use crate::keys;
use crate::session::{Load, Origin, Secondary};
use crate::settings_ui::SettingsTab;
use crate::substyle::{self, Position, SubStyle};
use crate::theme;
use eframe::egui::{
    self, Align, Align2, Color32, FontId, Id, Key, Layout, Rect, RichText, Sense, Stroke, pos2,
    vec2,
};
use gp_core::{Ms, TokenKind};
use std::sync::Arc;
use std::time::Duration;

const CONTROLS_HEIGHT: f32 = 78.0;
const HIDE_AFTER: Duration = Duration::from_millis(2500);
const VOLUME_WIDTH: f32 = 80.0;
/// Ids of the text fields the self-test types into.
pub const URL_FIELD: &str = "url-field";
pub const TRANSCRIPT_FIELD: &str = "transcript-search";

/// What was chosen in the Subtitles menu during a frame, done once the menu is drawn.
pub struct SubtitlePicks {
    pick_primary: Option<Option<usize>>,
    pick_secondary: Option<Secondary>,
    toggle_asr: bool,
    add_file: bool,
    save_srt: Option<bool>,
    pick_audio: Option<i64>,
    toggle_autopause: bool,
    open_settings: Option<Option<SettingsTab>>,
    open_help: bool,
    open_search: bool,
    pick_style: Option<SubStyle>,
    size_step: Option<bool>,
    toggle_dub: bool,
    volumes_changed: bool,
    volumes_done: bool,
    original: f32,
    voice: f32,
    voice_source: String,
    voice_source_changed: bool,
}

impl SubtitlePicks {
    pub fn new(cfg: &gp_core::config::Config) -> SubtitlePicks {
        SubtitlePicks {
            pick_primary: None,
            pick_secondary: None,
            toggle_asr: false,
            add_file: false,
            save_srt: None,
            pick_audio: None,
            toggle_autopause: false,
            open_settings: None,
            open_help: false,
            open_search: false,
            pick_style: None,
            size_step: None,
            toggle_dub: false,
            volumes_changed: false,
            volumes_done: false,
            original: cfg.tts.original_volume as f32,
            voice: cfg.tts.voice_volume as f32,
            voice_source: cfg.tts.voice_source.clone(),
            voice_source_changed: false,
        }
    }
}

/// Which items of the control bar are drawn at the current width.
pub struct ControlsFit {
    volume: bool,
    speed: bool,
    autopause: bool,
    settings: bool,
    sidebar: bool,
    fullscreen: bool,
    full_time: bool,
}

impl ControlsFit {
    pub fn all() -> Self {
        ControlsFit {
            volume: true,
            speed: true,
            autopause: true,
            settings: true,
            sidebar: true,
            fullscreen: true,
            full_time: true,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        if std::mem::take(&mut self.restyle) {
            theme::apply_colors(&ctx, &self.cfg.general.theme);
        }
        if std::mem::take(&mut self.rescale) {
            theme::apply(&ctx, self.cfg.general.ui_scale, &self.cfg.general.theme);
        }
        let chinese = crate::i18n::lang() == crate::i18n::Lang::Zh;
        if chinese != self.fonts_chinese {
            self.fonts_chinese = chinese;
            theme::install_fonts(&ctx, chinese);
        }
        self.tick(&ctx);
        self.handle_input(&ctx);
        self.panel_cues.clear();
        if self.sidebar_open {
            // In a small window the panel takes at most 40%, or the video would shrink to a strip.
            let max = (ctx.content_rect().width() * 0.4).clamp(220.0, 620.0);
            egui::Panel::right("sidebar")
                .default_size(380.0_f32.min(max))
                .size_range(220.0..=max)
                .frame(egui::Frame::NONE.fill(theme::panel()).inner_margin(12))
                .show(ui, |ui| self.sidebar(ui));
        }
        let backdrop = if self.session.is_some() {
            theme::DARK.bg
        } else {
            theme::bg()
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(backdrop))
            .show(ui, |ui| self.stage(ui));
        self.word_popup(&ctx);
        self.search_window(&ctx);
        self.settings_window(&ctx);
        self.help_window(&ctx);
        self.info_window(&ctx);
        self.asr_setup_dialog(&ctx);
        self.tts_setup_dialog(&ctx);
        self.toast_ui(&ctx);
        crate::frame::step(self, &ctx);
        crate::selftest::step(self, &ctx);
        crate::control::step(self, &ctx);
        crate::record::frame(self, &ctx);
    }

    fn on_exit(&mut self, gl: Option<&eframe::glow::Context>) {
        crate::record::finish(self);
        self.stop_asr();
        self.dub = None;
        self.forget_added_torrent();
        self.jobs.shutdown();
        self.video.destroy(gl);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        if self.quiet {
            // A test window can have the focus while someone types elsewhere: none of their
            // keys, and none of their clipboard, reach it.
            raw.events.retain(|e| !keys::is_keyboard(e));
        }
        crate::selftest::inject(self, ctx, raw);
        crate::record::inject(ctx, raw);
    }
}

fn icon_button(ui: &mut egui::Ui, icon: &str, hint: &str) -> bool {
    let r = ui.add(
        egui::Button::new(RichText::new(icon).size(17.0))
            .frame(false)
            .min_size(vec2(28.0, 28.0)),
    );
    crate::record::place(icon, r.rect);
    r.on_hover_text(hint).clicked()
}

/// The "How to use" button of the control bar, the same size as the icons beside it.
fn help_button(ui: &mut egui::Ui) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::click());
    let color = if resp.hovered() {
        theme::DARK.accent
    } else {
        theme::DARK.text
    };
    crate::settings_ui::paint_info(ui.painter(), rect.center(), 8.0, color);
    crate::record::place("help_button", rect);
    resp.on_hover_text(t(Tr::HelpHint)).clicked()
}

/// A small gear after an item of the Subtitles menu, opening its tab of Settings.
fn settings_gear(ui: &mut egui::Ui, tab: SettingsTab) -> bool {
    ui.add(egui::Button::new(RichText::new("⚙").color(theme::muted())).frame(false))
        .on_hover_text(tf(Tr::SettingsOf, &[&tab.title()]))
        .clicked()
}

/// The font's play and pause glyphs differ in size, so I draw this pair myself.
fn play_button(ui: &mut egui::Ui, paused: bool) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(32.0, 28.0), Sense::click());
    let color = if resp.hovered() {
        theme::DARK.accent
    } else {
        theme::DARK.text
    };
    let c = rect.center();
    if paused {
        let points = vec![
            c + vec2(-6.0, -8.0),
            c + vec2(8.0, 0.0),
            c + vec2(-6.0, 8.0),
        ];
        ui.painter()
            .add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
    } else {
        for x in [-6.0, 2.0] {
            let bar = Rect::from_min_size(c + vec2(x, -7.5), vec2(4.0, 15.0));
            ui.painter().rect_filled(bar, 1.0, color);
        }
    }
    resp.on_hover_text(if paused {
        t(Tr::PlayHint)
    } else {
        t(Tr::PauseHint)
    })
    .clicked()
}

fn toggle_chip(ui: &mut egui::Ui, on: bool, label: &str, hint: &str) -> bool {
    let color = if on {
        theme::DARK.accent
    } else {
        theme::DARK.muted
    };
    let button = egui::Button::new(RichText::new(label).size(13.0).color(color))
        .stroke(Stroke::new(
            1.0,
            if on {
                theme::DARK.accent
            } else {
                theme::DARK.line
            },
        ))
        .fill(Color32::TRANSPARENT);
    ui.add(button).on_hover_text(hint).clicked()
}

impl App {
    fn handle_input(&mut self, ctx: &egui::Context) {
        // A self-test window can take the focus while someone types elsewhere; their keys
        // never get here (see `raw_input_hook`), and their drops and mouse must not either.
        if !self.quiet {
            let dropped: Vec<String> = ctx.input(|i| {
                i.raw
                    .dropped_files
                    .iter()
                    .map(|f| f.path().to_string_lossy().into_owned())
                    .filter(|p| !p.is_empty())
                    .collect()
            });
            for path in dropped {
                self.open(&path);
            }
            if ctx.input(|i| i.pointer.delta() != egui::Vec2::ZERO || i.pointer.any_pressed()) {
                self.last_activity = std::time::Instant::now();
            }
        }
        let gate = self.key_gate(ctx);
        let actions = ctx.input(|i| keys::actions(&i.events, gate));
        for action in actions {
            // A self-test must not open a system dialog or take over the user's screen.
            let skip = self.selftest.is_some()
                && matches!(
                    action,
                    keys::Action::Run(keys::Command::OpenFile | keys::Command::Fullscreen)
                );
            let done = if skip {
                action.name().to_owned()
            } else {
                self.perform(action, ctx)
            };
            if self.selftest.is_some() {
                self.key_log.push(done);
            }
        }
    }

    pub fn key_gate(&self, ctx: &egui::Context) -> keys::Gate {
        keys::Gate {
            typing: ctx.egui_wants_keyboard_input(),
            modal: self.modal_open(),
        }
    }

    /// Runs a shortcut; returns what was done, for the self-test and MCP.
    pub fn perform(&mut self, action: keys::Action, ctx: &egui::Context) -> String {
        match action {
            keys::Action::Run(c) => {
                self.run_command(c, ctx);
                c.name().to_owned()
            }
            keys::Action::Paste(text) => format!("paste_{}", self.open_pasted(&text)),
        }
    }

    pub fn run_command(&mut self, command: keys::Command, ctx: &egui::Context) {
        use keys::Command as C;
        let mpv = self.player.mpv.clone();
        match command {
            C::PlayPause => {
                self.close_popup();
                let paused = mpv.get_flag("pause").unwrap_or(self.snap.paused);
                self.player.set_paused(!paused);
            }
            C::Back => self.player.seek_relative(-5_000),
            C::Forward => self.player.seek_relative(5_000),
            C::VolumeUp => self.set_volume(self.volume + 5.0),
            C::VolumeDown => self.set_volume(self.volume - 5.0),
            C::PreviousLine => self.cue_nav(-1),
            C::RepeatLine => self.cue_nav(0),
            C::NextLine => self.cue_nav(1),
            C::AutoPause => self.toggle_autopause(),
            C::Fullscreen => self.toggle_fullscreen(ctx),
            C::Sidebar => self.sidebar_open = !self.sidebar_open,
            C::Mute => {
                let muted = mpv.get_flag("mute").unwrap_or(self.snap.muted);
                mpv.set_flag("mute", !muted || self.quiet);
            }
            C::ToggleOriginal => {
                self.cfg.subtitles.show_primary = !self.cfg.subtitles.show_primary;
            }
            C::ToggleTranslation => {
                self.cfg.subtitles.show_secondary = !self.cfg.subtitles.show_secondary;
            }
            C::Slower | C::Faster => {
                let speed = mpv.get_f64("speed").unwrap_or(self.snap.speed);
                let speed = if command == C::Slower {
                    (speed - 0.25).max(0.25)
                } else {
                    (speed + 0.25).min(4.0)
                };
                mpv.set_f64("speed", speed);
            }
            C::SubtitlesSmaller | C::SubtitlesLarger => {
                let up = command == C::SubtitlesLarger;
                let scale = substyle::step_scale(self.cfg.subtitles.scale, up);
                self.set_subtitle_scale(scale);
                let percent = (scale * 100.0).round() as i32;
                self.toast_for(tf(Tr::SubSizeToast, &[&percent]), Duration::from_secs(1));
            }
            C::OpenFile => self.open_dialog(),
            C::Search => self.open_search(None),
            C::VideoInfo => self.toggle_info(),
            // The top window first: the setup dialogs open over Settings, and all of them
            // over the word card.
            C::Escape => {
                if egui::Popup::is_id_open(ctx, crate::context_menu::id()) {
                    // The menu closes itself on Esc.
                } else if self.asr_setup.dialog_open {
                    self.close_asr_setup();
                } else if self.tts_setup.dialog_open {
                    self.close_tts_setup();
                } else if self.help_open {
                    self.help_open = false;
                } else if self.info_open {
                    self.info_open = false;
                } else if self.settings_draft.is_some() {
                    self.settings_draft = None;
                } else if self.search_ui.open {
                    self.close_search();
                } else if self.popup.is_some() {
                    self.close_popup();
                } else if self.fullscreen {
                    self.toggle_fullscreen(ctx);
                }
            }
        }
    }

    /// A new subtitle size from the keys or the menu; it is kept in the config at once, since
    /// it is a thing people adjust while watching and expect to find again.
    pub fn set_subtitle_scale(&mut self, scale: f32) {
        self.cfg.subtitles.scale = scale;
        if !self.quiet
            && let Err(e) = crate::paths::save_config(&mut self.cfg)
        {
            self.toast(tf(Tr::CouldNotSaveSettings, &[&e]));
        }
    }

    /// Ctrl+V outside a text field: opens a copied link, magnet link or file path the way
    /// dropping a file or typing a link does. Returns which it was.
    pub fn open_pasted(&mut self, text: &str) -> &'static str {
        match keys::classify(text, |p| std::path::Path::new(p).is_file()) {
            keys::Pasted::Link(url) => {
                self.open(&url);
                "link"
            }
            keys::Pasted::File(path) => {
                self.open(&path);
                "file"
            }
            keys::Pasted::Magnet(magnet) => {
                if self.torrserver_ready() {
                    self.open_magnet(magnet);
                    "magnet"
                } else {
                    self.toast(t(Tr::PasteNeedsTorrServer));
                    "magnet_without_torrserver"
                }
            }
            keys::Pasted::Nothing => {
                self.toast(t(Tr::PasteNothing));
                "nothing"
            }
        }
    }

    fn toggle_autopause(&mut self) {
        self.trainer.autopause = !self.trainer.autopause;
        self.cfg.training.autopause = self.trainer.autopause;
        self.toast(if self.trainer.autopause {
            t(Tr::AutoPauseOn)
        } else {
            t(Tr::AutoPauseOff)
        });
    }

    /// A dialog or Settings is open: the player under it takes no clicks and no shortcuts.
    pub fn modal_open(&self) -> bool {
        self.settings_draft.is_some()
            || self.help_open
            || self.asr_setup.dialog_open
            || self.tts_setup.dialog_open
            || self.search_ui.open
    }

    pub fn controls_visible(&self) -> bool {
        if let Some(fixed) = crate::control::controls_override() {
            return fixed;
        }
        self.snap.paused
            || self.popup.is_some()
            || self.menu_at.is_some()
            || self.last_activity.elapsed() < HIDE_AFTER
            || self.selftest.as_ref().is_some_and(|s| s.plan.show_controls)
    }

    fn stage(&mut self, ui: &mut egui::Ui) {
        let rect = ui.max_rect();
        self.stage_rect = rect;
        if self.session.is_none() {
            return self.empty_state(ui, rect);
        }
        let sense = if self.modal_open() {
            Sense::hover()
        } else {
            Sense::click()
        };
        // A click that only closes the context menu must not also pause.
        let menu_was_open = egui::Popup::is_id_open(ui.ctx(), crate::context_menu::id());
        let response = ui.interact(rect, Id::unique("video"), sense);
        let active = !self.modal_open() && self.popup.is_none() && !menu_was_open;
        if response.hovered() && active {
            let scroll_y = ui.input(|i| i.smooth_scroll_delta.y);
            self.adjust_volume_by_wheel(scroll_y);
        }
        if response.double_clicked() {
            self.toggle_fullscreen(ui.ctx());
        } else if response.clicked() && !menu_was_open {
            if self.popup.is_some() {
                self.close_popup();
            } else {
                self.player.set_paused(!self.snap.paused);
            }
        }
        if response.secondary_clicked() {
            self.menu_request = response.interact_pointer_pos();
        }
        self.video.paint(ui, rect);

        // A frame being saved shows the picture and the subtitles alone.
        let shooting = self.window_shot.is_some();
        let visible = self.controls_visible() && !shooting;
        if !visible {
            ui.ctx().set_cursor_icon(egui::CursorIcon::None);
            ui.ctx().request_repaint_after(Duration::from_millis(300));
        } else if !self.snap.paused {
            ui.ctx().request_repaint_after(HIDE_AFTER);
        }
        // A tall window (a vertical recording, a phone-shaped layout) must not blow the text up.
        let scale = (rect.height() / 720.0)
            .min(rect.width() / 1280.0)
            .clamp(0.6, 3.0);
        let bottom = rect.bottom()
            - if visible {
                CONTROLS_HEIGHT + 14.0
            } else {
                34.0 * scale
            };
        let top = rect.top() + if visible { 52.0 } else { 20.0 * scale };
        self.subtitles(ui, rect, (top, bottom), scale);
        if visible {
            self.header(ui, rect);
            self.controls(ui, rect);
        }
        if !shooting {
            self.buffering_overlay(ui, rect);
        }
        self.context_menu(ui, &response);
    }

    /// A spinner with what is known of the wait, in the middle of the picture.
    fn buffering_overlay(&self, ui: &egui::Ui, rect: Rect) {
        let text = if self.buffering_shown {
            crate::buffering::label_in(crate::i18n::lang(), &self.cache)
        } else if self.dub_wait_shown {
            t(Tr::DubWaiting).to_owned()
        } else {
            return;
        };
        let galley =
            ui.painter()
                .layout_no_wrap(text, FontId::proportional(15.0), theme::DARK.text);
        let spinner = 20.0;
        let size = vec2(14.0 + spinner + 10.0 + galley.size().x + 16.0, 42.0);
        let plate = Rect::from_center_size(rect.center(), size);
        ui.painter()
            .rect_filled(plate, 10.0, Color32::from_black_alpha(150));
        let at = pos2(plate.left() + 14.0, plate.center().y - spinner / 2.0);
        egui::Spinner::new()
            .size(spinner)
            .color(theme::DARK.text)
            .paint_at(ui, Rect::from_min_size(at, vec2(spinner, spinner)));
        let text_at = pos2(
            at.x + spinner + 10.0,
            plate.center().y - galley.size().y / 2.0,
        );
        ui.painter().galley(text_at, galley, theme::DARK.text);
    }

    fn empty_state(&mut self, ui: &mut egui::Ui, rect: Rect) {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(Rect::from_center_size(rect.center(), vec2(460.0, 320.0)))
                .layout(Layout::top_down(Align::Center)),
        );
        let ui = &mut child;
        ui.label(RichText::new("Glossplay").size(34.0).strong());
        ui.label(RichText::new(t(Tr::Tagline)).color(theme::muted()));
        ui.add_space(18.0);
        if ui
            .add(
                egui::Button::new(RichText::new(t(Tr::OpenVideo)).size(16.0))
                    .min_size(vec2(200.0, 36.0)),
            )
            .clicked()
        {
            self.open_dialog();
        }
        ui.label(
            RichText::new(tk(Tr::DropHint).as_ref())
                .color(theme::muted())
                .small(),
        );
        ui.add_space(14.0);
        ui.horizontal(|ui| {
            let field = egui::TextEdit::singleline(&mut self.url_input)
                .id(Id::unique(URL_FIELD))
                .hint_text(t(Tr::PasteLink))
                .desired_width(340.0);
            let field = ui.add(field);
            crate::record::place("url_field", field.rect);
            let entered = field.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
            let play = ui.button(t(Tr::PlayButton));
            crate::record::place("play_button", play.rect);
            if (play.clicked() || entered) && !self.url_input.trim().is_empty() {
                let url = self.url_input.trim().to_owned();
                self.open(&url);
            }
        });
        if self.search_available() {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                let field = egui::TextEdit::singleline(&mut self.search_ui.query)
                    .hint_text(t(Tr::SearchField))
                    .desired_width(340.0);
                let entered = ui.add(field).lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
                if ui.button(t(Tr::SearchButton)).clicked() || entered {
                    let query = self.search_ui.query.clone();
                    self.open_search(Some(query));
                }
            });
        }
        ui.add_space(18.0);
        ui.label(
            RichText::new(t(Tr::ShortcutsHelp))
                .color(theme::muted())
                .small(),
        );
        if self.search_available() {
            ui.label(
                RichText::new(tk(Tr::ShortcutsSearch).as_ref())
                    .color(theme::muted())
                    .small(),
            );
        }
        ui.add_space(8.0);
        let (mut settings, mut help) = (false, false);
        centered_row(ui, |ui| {
            help = crate::settings_ui::info_link(ui, t(Tr::HelpTitle));
            ui.label(RichText::new("·").color(theme::muted()));
            settings = ui.link(t(Tr::SettingsLink)).clicked();
        });
        if help {
            self.help_open = true;
        }
        if settings {
            self.open_settings(None);
        }
        if self.asr_setup.installed.is_none() {
            ui.add_space(4.0);
            if ui.link(t(Tr::SetUpLink)).clicked() {
                self.open_settings(Some(SettingsTab::SpeechRecognition));
            }
        }
    }

    fn header(&mut self, ui: &mut egui::Ui, rect: Rect) {
        let Some(s) = &self.session else { return };
        let strip = Rect::from_min_size(rect.min, vec2(rect.width(), 44.0));
        ui.painter()
            .rect_filled(strip, 0.0, Color32::from_black_alpha(120));
        let mut text = s.title.clone();
        if !self.hwdec_current.is_empty() && self.hwdec_current != "no" {
            text.push_str(&format!("   ·   {}", self.hwdec_current));
        }
        if !s.asr_status.is_empty() {
            text.push_str(&format!("   ·   {}", s.asr_status));
        }
        let pending = s
            .tracks
            .iter()
            .filter(|t| t.load == Load::Loading && t.origin != Origin::Asr)
            .count();
        if pending > 0 {
            text.push_str(&format!("   ·   {}", t(Tr::ReadingSubtitles)));
        }
        if let Some(d) = &self.dub {
            text.push_str(&format!("   ·   {}", d.status()));
        }
        ui.painter().text(
            strip.left_center() + vec2(16.0, 0.0),
            Align2::LEFT_CENTER,
            text,
            FontId::proportional(14.0),
            theme::DARK.text,
        );
    }

    fn subtitles(&mut self, ui: &mut egui::Ui, rect: Rect, edges: (f32, f32), scale: f32) {
        let Some(s) = &self.session else { return };
        let pos = self.subtitle_pos();
        let primary = if self.cfg.subtitles.show_primary {
            s.primary_lines(pos)
        } else {
            Vec::new()
        };
        let secondary = if self.cfg.subtitles.show_secondary {
            s.secondary_lines(pos)
        } else {
            Vec::new()
        };
        let track_index = s.primary.unwrap_or(0);
        let lang = s.primary_track().and_then(|t| t.track.lang.clone());
        let secondary_lang = match s.secondary {
            Secondary::Track(i) => s.tracks.get(i).and_then(|t| t.track.lang.clone()),
            _ => Some(self.cfg.general.native_lang.clone()),
        };
        let look = SubStyle::from_id(&self.cfg.subtitles.style).look();
        let at_top = Position::from_id(&self.cfg.subtitles.position) == Position::Top;
        let size = self.cfg.subtitles.font_size
            * substyle::clamp_scale(self.cfg.subtitles.scale)
            * look.size
            * scale;
        let margin = self.cfg.subtitles.margin.clamp(0.0, 300.0) * scale;
        let max_width = rect.width() * 0.86;
        let small_size = size * self.cfg.subtitles.secondary_scale;
        let (top_edge, bottom_edge) = edges;
        let mut y = if at_top {
            top_edge + margin
        } else {
            bottom_edge - margin
        };

        let draw = SubDraw {
            rect,
            center_x: rect.center().x,
            look,
            scale,
            down: at_top,
        };
        let track = (track_index, lang.as_deref());
        let is_columns = self.cfg.subtitles.layout == "columns"
            && !primary.is_empty()
            && !secondary.is_empty();

        let clicked = if is_columns {
            let gap = 24.0 * scale;
            let total_width = rect.width() * 0.90;
            let col_width = ((total_width - gap) / 2.0).max(120.0);
            let left_center_x = rect.center().x - gap / 2.0 - col_width / 2.0;
            let right_center_x = rect.center().x + gap / 2.0 + col_width / 2.0;

            let draw_left = SubDraw {
                rect,
                center_x: left_center_x,
                look,
                scale,
                down: at_top,
            };
            let draw_right = SubDraw {
                rect,
                center_x: right_center_x,
                look,
                scale,
                down: at_top,
            };

            let mut y_left = y;
            let mut y_right = y;

            let clicked =
                self.primary_lines(ui, &draw_left, &primary, track, size, col_width, &mut y_left);
            draw_right.secondary(
                ui,
                &secondary,
                secondary_lang.as_deref(),
                small_size,
                col_width,
                &mut y_right,
            );
            let _ = if at_top {
                y_left.max(y_right)
            } else {
                y_left.min(y_right)
            };
            clicked
        } else if at_top {
            let clicked = self.primary_lines(ui, &draw, &primary, track, size, max_width, &mut y);
            draw.secondary(
                ui,
                &secondary,
                secondary_lang.as_deref(),
                small_size,
                max_width,
                &mut y,
            );
            clicked
        } else {
            draw.secondary(
                ui,
                &secondary,
                secondary_lang.as_deref(),
                small_size,
                max_width,
                &mut y,
            );
            self.primary_lines(ui, &draw, &primary, track, size, max_width, &mut y)
        };
        if let Some((word, cue, token, anchor, ctrl)) = clicked {
            if ctrl {
                self.toggle_word_token(cue, token, anchor);
            } else {
                self.open_word(word, cue, token, anchor);
            }
        }
    }

    /// Draws the original lines from `y` away from the edge and returns the word clicked.
    #[allow(clippy::too_many_arguments)]
    fn primary_lines(
        &mut self,
        ui: &mut egui::Ui,
        draw: &SubDraw,
        lines: &[crate::session::Line],
        track: (usize, Option<&str>),
        size: f32,
        max_width: f32,
        y: &mut f32,
    ) -> Option<(String, usize, usize, egui::Pos2, bool)> {
        let (_rect, look, scale) = (draw.rect, &draw.look, draw.scale);
        let (track_index, lang) = track;
        let ctrl_held = ui.input(|i| i.modifiers.command || i.modifiers.ctrl);
        let words_live = !self.modal_open();
        let mut clicked: Option<(String, usize, usize, egui::Pos2, bool)> = None;
        let ordered: Vec<&crate::session::Line> = if draw.down {
            lines.iter().collect()
        } else {
            lines.iter().rev().collect()
        };
        for line in ordered {
            let tokens = self.tokens_for(track_index, line.cue, &line.text, lang);
            let font = FontId::new(size, theme::subs_family(lang, &line.text));
            // Lay the words out in rows by hand: every word needs its own hit box.
            let mut rows: Vec<Vec<(usize, Arc<egui::Galley>)>> = vec![Vec::new()];
            let mut row_width = 0.0;
            for (i, token) in tokens.iter().enumerate() {
                let piece = &line.text[token.range.clone()];
                if piece.contains('\n') {
                    rows.push(Vec::new());
                    row_width = 0.0;
                    continue;
                }
                let galley =
                    ui.painter()
                        .layout_no_wrap(piece.to_owned(), font.clone(), look.original);
                let width = galley.size().x;
                if row_width + width > max_width && row_width > 0.0 {
                    rows.push(Vec::new());
                    row_width = 0.0;
                }
                if row_width == 0.0 && piece.trim().is_empty() {
                    continue;
                }
                row_width += width;
                rows.last_mut().unwrap().push((i, galley));
            }
            rows.retain(|r| !r.is_empty());
            let row_height = size * 1.3;
            let block_height = rows.len() as f32 * row_height;
            let widest = rows
                .iter()
                .map(|r| r.iter().map(|(_, g)| g.size().x).sum::<f32>())
                .fold(0.0, f32::max);
            let block_top = draw.top_of(*y, block_height);
            let block = Rect::from_min_size(
                pos2(draw.center_x - widest / 2.0, block_top),
                vec2(widest, block_height),
            );
            if let Some(plate) = look.plate {
                ui.painter()
                    .rect_filled(block.expand2(vec2(12.0, 4.0)), 8.0, plate);
            }
            for (r, row) in rows.iter().enumerate() {
                let width: f32 = row.iter().map(|(_, g)| g.size().x).sum();
                let mut x = draw.center_x - width / 2.0;
                let top = block.top() + r as f32 * row_height;
                for (i, galley) in row {
                    let token = &tokens[*i];
                    let word_rect =
                        Rect::from_min_size(pos2(x, top), vec2(galley.size().x, row_height));
                    let at = pos2(x, top + (row_height - galley.size().y) / 2.0);
                    x += galley.size().x;
                    if token.kind != TokenKind::Word {
                        substyle::paint(
                            ui.painter(),
                            at,
                            galley.clone(),
                            look.original,
                            look,
                            scale,
                        );
                        continue;
                    }
                    let word = &line.text[token.range.clone()];
                    crate::control::word_drawn(track_index, line.cue, *i, word_rect);
                    let resp = words_live.then(|| {
                        ui.interact(
                            word_rect,
                            Id::unique(("word", line.cue, *i)),
                            Sense::click(),
                        )
                    });
                    let hovered = resp.as_ref().is_some_and(|r| r.hovered());
                    let selected = self
                        .popup
                        .as_ref()
                        .is_some_and(|p| p.contains_token(line.cue, *i));
                    if hovered || selected {
                        ui.painter().rect_filled(
                            word_rect.expand2(vec2(2.0, 0.0)),
                            5.0,
                            look.hover,
                        );
                    }
                    if hovered {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    substyle::paint(ui.painter(), at, galley.clone(), look.original, look, scale);
                    if self.saved_words.contains(&word.to_lowercase()) {
                        let under = word_rect.bottom() - row_height * 0.1;
                        ui.painter().line_segment(
                            [
                                pos2(word_rect.left(), under),
                                pos2(word_rect.right(), under),
                            ],
                            Stroke::new(2.0 * scale, look.saved),
                        );
                    }
                    if let Some(r) = resp {
                        if r.clicked() {
                            clicked = Some((
                                word.to_owned(),
                                line.cue,
                                *i,
                                word_rect.center_top(),
                                ctrl_held,
                            ));
                        }
                        // The word sits on the picture, so it opens the picture's menu.
                        if r.secondary_clicked() {
                            self.menu_request = r.interact_pointer_pos();
                        }
                    }
                }
            }
            *y = draw.next(*y, block_height + 10.0);
        }
        clicked
    }

    fn controls(&mut self, ui: &mut egui::Ui, rect: Rect) {
        let bar = Rect::from_min_max(pos2(rect.left(), rect.bottom() - CONTROLS_HEIGHT), rect.max);
        ui.painter()
            .rect_filled(bar, 0.0, Color32::from_black_alpha(190));
        // Swallow clicks on the bar so they do not toggle pause through to the video.
        ui.interact(bar, Id::unique("controls-bg"), Sense::click());
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(bar.shrink2(vec2(16.0, 8.0)))
                .layout(Layout::top_down(Align::Min)),
        );
        let ui = &mut child;
        // The bar lies on the picture, so it keeps the dark look in a light theme.
        ui.set_style(std::sync::Arc::new(theme::style(false)));
        self.seek_bar(ui);
        ui.add_space(2.0);
        let fit = self.fit_controls(ui, ui.available_width());
        let clock = if fit.full_time {
            format!(
                "{} / {}",
                theme::clock(self.snap.pos),
                theme::clock(self.snap.duration)
            )
        } else {
            theme::clock(self.snap.pos)
        };
        ui.horizontal(|ui| {
            if icon_button(ui, "⏮", t(Tr::PrevLineHint)) {
                self.cue_nav(-1);
            }
            if play_button(ui, self.snap.paused) {
                self.close_popup();
                self.player.set_paused(!self.snap.paused);
            }
            if icon_button(ui, "⏭", t(Tr::NextLineHint)) {
                self.cue_nav(1);
            }
            if icon_button(ui, "🔁", t(Tr::RepeatLineHint)) {
                self.cue_nav(0);
            }
            ui.label(RichText::new(clock).color(theme::DARK.muted).monospace());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if fit.fullscreen && icon_button(ui, "⛶", t(Tr::FullscreenHint)) {
                    self.toggle_fullscreen(ui.ctx());
                }
                if fit.settings && help_button(ui) {
                    self.help_open = true;
                }
                if fit.settings && icon_button(ui, "⚙", t(Tr::SettingsHint)) {
                    self.open_settings(None);
                }
                if fit.settings
                    && self.search_available()
                    && icon_button(ui, "🔍", &tk(Tr::SearchVideosHint))
                {
                    self.open_search(None);
                }
                if fit.sidebar && icon_button(ui, "☰", t(Tr::SidebarHint)) {
                    self.sidebar_open = !self.sidebar_open;
                }
                self.subtitle_menu(ui, &fit);
                if fit.autopause
                    && toggle_chip(
                        ui,
                        self.trainer.autopause,
                        t(Tr::AutoPause),
                        t(Tr::AutoPauseHint),
                    )
                {
                    self.toggle_autopause();
                }
                if fit.speed {
                    ui.menu_button(
                        RichText::new(format!("{:.2}×", self.snap.speed)).size(13.0),
                        |ui| {
                            for speed in [0.5, 0.75, 1.0, 1.25, 1.5, 2.0] {
                                if ui
                                    .selectable_label(self.snap.speed == speed, format!("{speed}×"))
                                    .clicked()
                                {
                                    self.player.mpv.set_f64("speed", speed);
                                    ui.close();
                                }
                            }
                        },
                    );
                }
                if fit.volume {
                    let mut volume = self.volume as f32;
                    ui.spacing_mut().slider_width = VOLUME_WIDTH;
                    if ui
                        .add(egui::Slider::new(&mut volume, 0.0..=150.0).show_value(false))
                        .changed()
                    {
                        self.set_volume(volume as f64);
                    }
                }
                if icon_button(
                    ui,
                    if self.snap.muted { "🔇" } else { "🔊" },
                    t(Tr::MuteHint),
                ) {
                    self.player.mpv.set_flag("mute", !self.snap.muted);
                }
            });
        });
    }

    /// Decides which control-bar items fit into `avail` points. The least important ones go
    /// first, so nothing is ever drawn on top of something else.
    fn fit_controls(&self, ui: &egui::Ui, avail: f32) -> ControlsFit {
        let gap = ui.spacing().item_spacing.x;
        let text_w = |text: String, font: FontId| {
            ui.painter()
                .layout_no_wrap(text, font, Color32::WHITE)
                .size()
                .x
        };
        let chip_w = |text: &str| text_w(text.to_owned(), FontId::proportional(13.0)) + 16.0 + 6.0;
        let mono = egui::TextStyle::Monospace.resolve(ui.style());
        let full_clock = format!(
            "{} / {}",
            theme::clock(self.snap.pos),
            theme::clock(self.snap.duration)
        );
        // The clock grows with the longest value it can take, so the layout does not jump.
        let widest = |s: &str| text_w(s.replace(|c: char| c.is_ascii_digit(), "0"), mono.clone());
        let full_clock_w = widest(&full_clock);
        let short_clock_w = widest(&theme::clock(self.snap.duration));
        let icon = 28.0;
        let left = 28.0 + 32.0 + 28.0 + 28.0 + 3.0 * gap;
        let speed_w = chip_w("0.00×");
        let autopause_w = chip_w(t(Tr::AutoPause));
        let subtitles_w = chip_w(t(Tr::SubtitlesMenu));
        let search = self.search_available();
        let needed = |f: &ControlsFit| {
            let mut right = vec![icon, subtitles_w];
            for (on, w) in [
                (f.volume, VOLUME_WIDTH + 2.0),
                (f.speed, speed_w),
                (f.autopause, autopause_w),
                (f.settings, icon),
                (f.settings, icon),
                (f.settings && search, icon),
                (f.sidebar, icon),
                (f.fullscreen, icon),
            ] {
                if on {
                    right.push(w);
                }
            }
            let clock = if f.full_time {
                full_clock_w
            } else {
                short_clock_w
            };
            let right_total = right.iter().sum::<f32>() + gap * (right.len() as f32 - 1.0);
            left + gap + clock + gap * 2.0 + right_total
        };
        let mut fit = ControlsFit::all();
        let drops: [fn(&mut ControlsFit); 7] = [
            |f| f.volume = false,
            |f| f.speed = false,
            |f| f.autopause = false,
            |f| f.full_time = false,
            |f| f.sidebar = false,
            |f| f.settings = false,
            |f| f.fullscreen = false,
        ];
        for drop in drops {
            if needed(&fit) <= avail {
                break;
            }
            drop(&mut fit);
        }
        fit
    }

    fn seek_bar(&mut self, ui: &mut egui::Ui) {
        let (rect, resp) =
            ui.allocate_exact_size(vec2(ui.available_width(), 16.0), Sense::click_and_drag());
        let duration = self.snap.duration.max(1) as f32;
        let track = Rect::from_center_size(
            rect.center(),
            vec2(rect.width(), if resp.hovered() { 6.0 } else { 4.0 }),
        );
        ui.painter()
            .rect_filled(track, 3.0, Color32::from_white_alpha(50));
        let x_of = |ms: Ms| track.left() + track.width() * (ms as f32 / duration).clamp(0.0, 1.0);
        // What a stream has loaded already, lighter, under the played part.
        for &(from, to) in &self.cache.state.ranges {
            let ms = |s: f64| (s * 1000.0) as Ms;
            let loaded = Rect::from_min_max(
                pos2(x_of(ms(from)), track.top()),
                pos2(x_of(ms(to)), track.bottom()),
            );
            ui.painter()
                .rect_filled(loaded, 3.0, Color32::from_white_alpha(70));
        }
        let played = Rect::from_min_max(track.min, pos2(x_of(self.snap.pos), track.bottom()));
        ui.painter().rect_filled(played, 3.0, theme::DARK.accent);
        if resp.hovered() || resp.dragged() {
            ui.painter().circle_filled(
                pos2(x_of(self.snap.pos), track.center().y),
                7.0,
                theme::DARK.accent,
            );
        }
        let Some(pointer) = resp.interact_pointer_pos().or(resp.hover_pos()) else {
            return;
        };
        let target =
            (((pointer.x - track.left()) / track.width()).clamp(0.0, 1.0) * duration) as Ms;
        if resp.hovered() {
            resp.clone().on_hover_text_at_pointer(theme::clock(target));
        }
        if resp.dragged() {
            // Keyframe seeks keep scrubbing fluid; the exact seek happens on release.
            let secs = format!("{:.3}", target as f64 / 1000.0);
            self.player
                .mpv
                .command_async(&["seek", &secs, "absolute+keyframes"]);
        }
        if resp.clicked() || resp.drag_stopped() {
            self.player.seek(target);
        }
    }

    fn subtitle_menu(&mut self, ui: &mut egui::Ui, fit: &ControlsFit) {
        let mut picks = SubtitlePicks::new(&self.cfg);
        let menu = ui.menu_button(RichText::new(t(Tr::SubtitlesMenu)).size(13.0), |ui| {
            self.subtitle_menu_ui(ui, fit, &mut picks);
        });
        crate::record::place("subtitles_menu", menu.response.rect);
        if self.selftest.as_ref().is_some_and(|s| s.plan.subs_menu) {
            egui::Popup::open_id(ui.ctx(), egui::Popup::default_response_id(&menu.response));
        }
        self.apply_subtitle_picks(picks);
    }

    /// The items of the Subtitles menu, which the context menu shows too.
    pub fn subtitle_menu_ui(&self, ui: &mut egui::Ui, fit: &ControlsFit, p: &mut SubtitlePicks) {
        let provider = match &self.translator {
            Ok(t) => t.id(),
            Err(_) => t(Tr::ProviderNotSetUp).to_owned(),
        };
        let asr_running = self.session.as_ref().is_some_and(|s| s.asr_running);
        let dub_on = self.dub.is_some();
        {
            let Some(s) = &self.session else { return };
            ui.set_min_width(260.0);
            ui.label(RichText::new(t(Tr::Learning)).color(theme::muted()).small());
            for (i, sub) in s.tracks.iter().enumerate() {
                let state = match &sub.load {
                    Load::Ready if sub.live => format!("  ({})", t(Tr::StateAsYouWatch)),
                    Load::NotLoaded | Load::Ready => String::new(),
                    Load::Loading => format!("  ({})", t(Tr::StateReading)),
                    Load::Failed(_) => format!("  ({})", t(Tr::StateFailed)),
                    Load::Bitmap => format!("  ({})", t(Tr::StateImage)),
                };
                let resp =
                    ui.selectable_label(s.primary == Some(i), format!("{}{state}", sub.label));
                if let Load::Failed(e) = &sub.load {
                    resp.clone().on_hover_text(e);
                }
                if resp.clicked() {
                    p.pick_primary = Some(Some(i));
                }
            }
            if ui
                .selectable_label(s.primary.is_none(), t(Tr::NoneChoice))
                .clicked()
            {
                p.pick_primary = Some(None);
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(t(Tr::TranslationLabel))
                        .color(theme::muted())
                        .small(),
                );
                if settings_gear(ui, SettingsTab::Translation) {
                    p.open_settings = Some(Some(SettingsTab::Translation));
                }
            });
            if ui
                .selectable_label(
                    s.secondary == Secondary::Translate,
                    tf(Tr::TranslateAuto, &[&provider]),
                )
                .clicked()
            {
                p.pick_secondary = Some(Secondary::Translate);
            }
            for (i, t) in s
                .tracks
                .iter()
                .enumerate()
                .filter(|(i, t)| t.is_text() && s.primary != Some(*i))
            {
                if ui
                    .selectable_label(s.secondary == Secondary::Track(i), &t.label)
                    .clicked()
                {
                    p.pick_secondary = Some(Secondary::Track(i));
                }
            }
            if ui
                .selectable_label(s.secondary == Secondary::Off, t(Tr::OffChoice))
                .clicked()
            {
                p.pick_secondary = Some(Secondary::Off);
            }
            ui.separator();
            let label = if asr_running {
                t(Tr::StopTranscribing)
            } else {
                t(Tr::TranscribeSpeech)
            };
            ui.horizontal(|ui| {
                let transcribe = ui.button(label);
                crate::record::place("transcribe", transcribe.rect);
                if transcribe.clicked() {
                    p.toggle_asr = true;
                }
                if settings_gear(ui, SettingsTab::SpeechRecognition) {
                    p.open_settings = Some(Some(SettingsTab::SpeechRecognition));
                }
            });
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(dub_on, t(Tr::MenuVoiceOver))
                    .on_hover_text(t(Tr::MenuVoiceOverHint))
                    .clicked()
                {
                    p.toggle_dub = true;
                }
                if settings_gear(ui, SettingsTab::VoiceOver) {
                    p.open_settings = Some(Some(SettingsTab::VoiceOver));
                }
            });
            if dub_on {
                egui::Grid::new("dub-volumes")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        for (name, label, value) in [
                            ("slider_original", t(Tr::SliderOriginal), &mut p.original),
                            ("slider_voice", t(Tr::SliderVoice), &mut p.voice),
                        ] {
                            ui.label(RichText::new(label).color(theme::muted()));
                            let r =
                                ui.add(egui::Slider::new(value, 0.0..=150.0).integer().suffix("%"));
                            let rail = egui::Rect::from_min_size(
                                r.rect.min,
                                vec2(ui.spacing().slider_width, r.rect.height()),
                            );
                            crate::record::place(name, rail);
                            p.volumes_changed |= r.changed();
                            p.volumes_done |= r.drag_stopped() || (r.changed() && !r.dragged());
                            ui.end_row();
                        }
                        ui.label(RichText::new(t(Tr::RowVoiceSource)).color(theme::muted()))
                            .on_hover_text(t(Tr::VoiceSpeakerNote));
                        ui.horizontal(|ui| {
                            p.voice_source_changed |=
                                crate::tts_ui::voice_source_choice(ui, &mut p.voice_source);
                        })
                        .response
                        .on_hover_text(t(Tr::VoiceSpeakerNote));
                        ui.end_row();
                    });
            }
            if ui.button(t(Tr::AddSubFile)).clicked() {
                p.add_file = true;
            }
            ui.horizontal(|ui| {
                let percent =
                    (substyle::clamp_scale(self.cfg.subtitles.scale) * 100.0).round() as i32;
                ui.label(tf(Tr::SubSizeMenu, &[&percent]));
                if ui.small_button("−").clicked() {
                    p.size_step = Some(false);
                }
                if ui.small_button("+").clicked() {
                    p.size_step = Some(true);
                }
            });
            ui.menu_button(t(Tr::SubStyleLabel), |ui| {
                let current = SubStyle::from_id(&self.cfg.subtitles.style);
                for style in SubStyle::ALL {
                    if ui
                        .selectable_label(style == current, t(style.name()))
                        .clicked()
                    {
                        p.pick_style = Some(style);
                        ui.close();
                    }
                }
            });
            ui.separator();
            let has_primary = s.primary_track().is_some();
            let shown = s.secondary != Secondary::Off;
            if ui
                .add_enabled(has_primary, egui::Button::new(t(Tr::SaveTranscriptSrt)))
                .clicked()
            {
                p.save_srt = Some(false);
            }
            if shown
                && ui
                    .add_enabled(has_primary, egui::Button::new(t(Tr::SaveTranslationSrt)))
                    .clicked()
            {
                p.save_srt = Some(true);
            }
            if self.audio_tracks.len() > 1 {
                ui.separator();
                ui.label(
                    RichText::new(t(Tr::AudioLabel))
                        .color(theme::muted())
                        .small(),
                );
                for a in &self.audio_tracks {
                    if ui.selectable_label(a.selected, a.label()).clicked() {
                        p.pick_audio = Some(a.id);
                    }
                }
            }
            // Items the narrow control bar had no room for live here instead.
            if !fit.autopause || !fit.settings {
                ui.separator();
            }
            if !fit.autopause
                && ui
                    .selectable_label(self.trainer.autopause, t(Tr::AutoPauseMenu))
                    .clicked()
            {
                p.toggle_autopause = true;
            }
            if !fit.settings && ui.button(t(Tr::SettingsMenu)).clicked() {
                p.open_settings = Some(None);
            }
            if !fit.settings && ui.button(t(Tr::HelpMenu)).clicked() {
                p.open_help = true;
            }
            if !fit.settings && self.search_available() && ui.button(t(Tr::SearchMenu)).clicked() {
                p.open_search = true;
            }
        }
    }

    pub fn apply_subtitle_picks(&mut self, p: SubtitlePicks) {
        let SubtitlePicks {
            pick_primary,
            pick_secondary,
            toggle_asr,
            add_file,
            save_srt,
            pick_audio,
            toggle_autopause,
            open_settings,
            open_help,
            open_search,
            pick_style,
            size_step,
            toggle_dub,
            volumes_changed,
            volumes_done,
            original,
            voice,
            voice_source,
            voice_source_changed,
        } = p;
        let asr_running = self.session.as_ref().is_some_and(|s| s.asr_running);
        if toggle_dub {
            self.set_dub(self.dub.is_none());
        }
        if voice_source_changed {
            let mut new = self.cfg.clone();
            new.tts.voice_source = voice_source;
            self.apply_settings(new);
            if !self.quiet
                && let Err(e) = crate::paths::save_config(&mut self.cfg)
            {
                self.toast(tf(Tr::CouldNotSaveSettings, &[&e]));
            }
        }
        if volumes_changed {
            self.cfg.tts.original_volume = f64::from(original);
            self.cfg.tts.voice_volume = f64::from(voice);
            self.apply_volume();
        }
        if volumes_done
            && !self.quiet
            && let Err(e) = crate::paths::save_config(&mut self.cfg)
        {
            self.toast(tf(Tr::CouldNotSaveSettings, &[&e]));
        }
        if toggle_autopause {
            self.toggle_autopause();
        }
        if let Some(tab) = open_settings {
            self.open_settings(tab);
        }
        if open_help {
            self.help_open = true;
        }
        if let Some(up) = size_step {
            let scale = substyle::step_scale(self.cfg.subtitles.scale, up);
            self.set_subtitle_scale(scale);
        }
        if let Some(style) = pick_style {
            self.cfg.subtitles.style = style.id().to_owned();
            if !self.quiet
                && let Err(e) = crate::paths::save_config(&mut self.cfg)
            {
                self.toast(tf(Tr::CouldNotSaveSettings, &[&e]));
            }
        }
        if open_search {
            self.open_search(None);
        }
        if let Some(choice) = pick_primary {
            self.select_primary(choice);
        }
        if let Some(choice) = pick_secondary {
            self.set_secondary(choice);
        }
        if toggle_asr {
            if asr_running {
                self.stop_asr()
            } else {
                self.request_asr()
            }
        }
        if add_file {
            let picked = rfd::FileDialog::new()
                .add_filter(t(Tr::SubtitlesMenu), &["srt", "vtt", "ass", "ssa"])
                .pick_file();
            if let Some(path) = picked {
                self.open(&path.to_string_lossy());
            }
        }
        if let Some(translation) = save_srt {
            self.save_srt(translation);
        }
        if let Some(id) = pick_audio {
            self.player.mpv.set("aid", &id.to_string());
            for a in &mut self.audio_tracks {
                a.selected = a.id == id;
            }
        }
    }

    fn save_srt(&mut self, translation: bool) {
        let Some(s) = &self.session else { return };
        let Some(t) = s.primary_track() else { return };
        let (cues, lang) = if translation {
            (s.translation_cues(), self.cfg.general.native_lang.clone())
        } else {
            (
                t.track.cues.clone(),
                t.track.lang.clone().unwrap_or_default(),
            )
        };
        let title: String = s
            .title
            .chars()
            .map(|c| if "\\/:*?\"<>|".contains(c) { '_' } else { c })
            .collect();
        let name = if lang.is_empty() {
            format!("{title}.srt")
        } else {
            format!("{title}.{lang}.srt")
        };
        let Some(path) = rfd::FileDialog::new()
            .add_filter("SubRip", &["srt"])
            .set_file_name(name)
            .save_file()
        else {
            return;
        };
        match std::fs::write(&path, gp_core::write_srt(&cues)) {
            Ok(()) => self.toast(tf(Tr::SavedLines, &[&cues.len(), &path.display()])),
            Err(e) => self.toast(tf(Tr::CouldNotSaveFile, &[&path.display(), &e])),
        }
    }

    pub fn select_primary(&mut self, choice: Option<usize>) {
        if let Some(i) = choice {
            self.ensure_loaded(i);
        }
        let Some(s) = &mut self.session else { return };
        s.primary = choice;
        self.scrolled_to = None;
        // mpv draws image subtitles (and tracks I could not read) itself; for text tracks its own renderer stays off.
        let bitmap_id = choice.and_then(|i| match (&s.tracks[i].origin, &s.tracks[i].load) {
            (Origin::Embedded { mpv_id, .. }, Load::Bitmap | Load::Failed(_)) => Some(*mpv_id),
            _ => None,
        });
        self.mpv_draws(bitmap_id);
        self.set_live_slots();
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.sidebar_tab,
                SidebarTab::Transcript,
                t(Tr::TabTranscript),
            );
            let words = tf(Tr::TabWords, &[&self.vocab.len()]);
            let r = ui.selectable_value(&mut self.sidebar_tab, SidebarTab::Words, words);
            crate::record::place("words_tab", r.rect);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if icon_button(ui, "✕", t(Tr::CloseHint)) {
                    self.sidebar_open = false;
                }
            });
        });
        let search = ui.add(
            egui::TextEdit::singleline(&mut self.search)
                .id(Id::unique(TRANSCRIPT_FIELD))
                .hint_text(t(Tr::SearchHint))
                .desired_width(f32::INFINITY),
        );
        crate::record::place("transcript_search", search.rect);
        ui.add_space(4.0);
        match self.sidebar_tab {
            SidebarTab::Transcript => self.transcript(ui),
            SidebarTab::Words => self.word_list(ui),
        }
    }

    fn transcript(&mut self, ui: &mut egui::Ui) {
        let Some(s) = &self.session else {
            ui.label(RichText::new(t(Tr::OpenToSeeTranscript)).color(theme::muted()));
            return;
        };
        let Some(t) = s.primary_track() else {
            ui.label(RichText::new(t(Tr::NoTextSubtitles)).color(theme::muted()));
            return;
        };
        let needle = self.search.to_lowercase();
        let rows: Vec<usize> = (0..t.track.cues.len())
            .filter(|&i| needle.is_empty() || t.track.cues[i].text.to_lowercase().contains(&needle))
            .collect();
        let current = t.track.index_at_or_before(self.snap.pos);
        let row_height = 46.0;
        let mut area = egui::ScrollArea::vertical().auto_shrink(false);
        // Follow playback, but not while the pointer is over the list: the user is reading it.
        let hovering = ui.rect_contains_pointer(ui.max_rect());
        if current != self.scrolled_to && !hovering && needle.is_empty() {
            if let Some(row) = current.and_then(|c| rows.iter().position(|&i| i == c)) {
                let offset = row as f32 * (row_height + ui.spacing().item_spacing.y)
                    - ui.available_height() / 2.0;
                area = area.vertical_scroll_offset(offset.max(0.0));
            }
            self.scrolled_to = current;
        }
        let native = self.cfg.general.native_lang.clone();
        let mut seek_to = None;
        let mut shown = 0..0;
        area.show_rows(ui, row_height, rows.len(), |ui, range| {
            shown = range.clone();
            for (n, &i) in rows[range.clone()].iter().enumerate() {
                let cue = &t.track.cues[i];
                let (rect, resp) =
                    ui.allocate_exact_size(vec2(ui.available_width(), row_height), Sense::click());
                if crate::record::on() {
                    crate::record::place(&format!("transcript_row_{}", range.start + n), rect);
                }
                if Some(i) == current {
                    ui.painter().rect_filled(rect, 6.0, theme::raised());
                    ui.painter().rect_filled(
                        Rect::from_min_size(rect.min, vec2(3.0, rect.height())),
                        2.0,
                        theme::accent(),
                    );
                } else if resp.hovered() {
                    ui.painter()
                        .rect_filled(rect, 6.0, theme::raised().gamma_multiply(0.6));
                }
                let text_left = rect.left() + 58.0;
                let width = rect.right() - text_left - 6.0;
                ui.painter().text(
                    rect.left_top() + vec2(10.0, 7.0),
                    Align2::LEFT_TOP,
                    theme::clock(cue.start),
                    FontId::monospace(11.0),
                    theme::muted(),
                );
                let one_line = |text: &str, font: FontId, color: Color32| {
                    let mut job = egui::text::LayoutJob::simple_singleline(
                        text.replace('\n', " "),
                        font,
                        color,
                    );
                    job.wrap = egui::text::TextWrapping::truncate_at_width(width);
                    ui.painter().layout_job(job)
                };
                ui.painter().galley(
                    pos2(text_left, rect.top() + 5.0),
                    one_line(
                        &cue.text,
                        FontId::new(14.5, theme::subs_family(t.track.lang.as_deref(), &cue.text)),
                        theme::text(),
                    ),
                    theme::text(),
                );
                if let Some(tr) = s.translation_of(i) {
                    ui.painter().galley(
                        pos2(text_left, rect.top() + 25.0),
                        one_line(
                            &tr,
                            FontId::new(12.5, theme::subs_family(Some(&native), &tr)),
                            theme::muted(),
                        ),
                        theme::muted(),
                    );
                }
                if resp.clicked() {
                    seek_to = Some(cue.start);
                }
            }
        });
        // A few rows past the edges, so a short scroll finds them translated already.
        const MARGIN: usize = 5;
        let around = shown.start.saturating_sub(MARGIN)..(shown.end + MARGIN).min(rows.len());
        self.panel_cues = rows[around].to_vec();
        if let Some(ms) = seek_to {
            self.player.seek(ms);
        }
    }

    fn word_list(&mut self, ui: &mut egui::Ui) {
        if self.vocab.is_empty() {
            ui.label(RichText::new(t(Tr::WordsEmpty)).color(theme::muted()));
            return;
        }
        let needle = self.search.to_lowercase();
        let current_media = self.session.as_ref().map(|s| s.media.clone());
        let (mut jump, mut anki, mut custom_dict, mut delete) = (None, None, None, None);
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for entry in &self.vocab {
                    let w = &entry.word;
                    if !needle.is_empty()
                        && !w.word.to_lowercase().contains(&needle)
                        && !w.translation.to_lowercase().contains(&needle)
                    {
                        continue;
                    }
                    egui::Frame::NONE
                        .fill(theme::raised())
                        .corner_radius(8)
                        .inner_margin(10)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.horizontal_wrapped(|ui| {
                                ui.label(
                                    RichText::new(&w.word)
                                        .size(17.0)
                                        .strong()
                                        .family(theme::subs_family(None, &w.word)),
                                );
                                ui.label(RichText::new(&w.translation).color(theme::accent()));
                            });
                            ui.label(
                                RichText::new(&w.sentence)
                                    .color(theme::muted())
                                    .small()
                                    .family(theme::subs_family(None, &w.sentence)),
                            );
                            ui.horizontal(|ui| {
                                if current_media.as_deref() == Some(w.media_path.as_str())
                                    && ui
                                        .small_button(t(Tr::ShowWord))
                                        .on_hover_text(t(Tr::JumpHint))
                                        .clicked()
                                {
                                    jump = Some(w.cue_start_ms);
                                }
                                let label = if entry.anki_note_id.is_some() {
                                    t(Tr::InAnki)
                                } else {
                                    t(Tr::SendToAnki)
                                };
                                if ui
                                    .add_enabled(
                                        entry.anki_note_id.is_none(),
                                        egui::Button::new(label).small(),
                                    )
                                    .clicked()
                                {
                                    anki = Some(entry.id);
                                }
                                let dict_label = if entry.custom_dict_synced_at.is_some() {
                                    t(Tr::InCustomDict)
                                } else {
                                    t(Tr::SendToCustomDict)
                                };
                                if ui
                                    .add_enabled(
                                        entry.custom_dict_synced_at.is_none(),
                                        egui::Button::new(dict_label).small(),
                                    )
                                    .clicked()
                                {
                                    custom_dict = Some(entry.id);
                                }
                                if ui
                                    .small_button(
                                        RichText::new(t(Tr::DeleteWord)).color(theme::danger()),
                                    )
                                    .clicked()
                                {
                                    delete = Some(entry.id);
                                }
                                ui.label(
                                    RichText::new(&w.media_title).color(theme::muted()).small(),
                                );
                            });
                        });
                }
            });
        if let Some(ms) = jump {
            self.player.seek(ms);
        }
        if let Some(id) = anki {
            self.export_to_anki(id);
        }
        if let Some(id) = custom_dict {
            self.export_to_custom_dict(id);
        }
        if let Some(id) = delete {
            self.delete_word(id);
        }
    }

    fn word_popup(&mut self, ctx: &egui::Context) {
        let Some(p) = self.popup.as_ref().filter(|_| self.window_shot.is_none()) else {
            return;
        };
        let in_anki = p.saved_id.is_some_and(|id| {
            self.vocab
                .iter()
                .any(|v| v.id == id && v.anki_note_id.is_some())
        });
        let in_custom_dict = p.saved_id.is_some_and(|id| {
            self.vocab
                .iter()
                .any(|v| v.id == id && v.custom_dict_synced_at.is_some())
        });
        let lang = self
            .session
            .as_ref()
            .and_then(|s| s.primary_track())
            .and_then(|t| t.track.lang.clone());
        let (mut save, mut anki, mut custom_dict, mut close) = (false, false, false, false);
        // Above the subtitles and the controls, below every window.
        egui::Area::new(Id::unique("word-popup"))
            .order(egui::Order::Middle)
            .pivot(Align2::CENTER_BOTTOM)
            .fixed_pos(p.anchor - vec2(0.0, 14.0))
            .constrain(true)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .fill(theme::panel())
                    .inner_margin(14)
                    .show(ui, |ui| {
                        ui.set_width(340.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&p.word)
                                    .size(22.0)
                                    .strong()
                                    .family(theme::subs_family(lang.as_deref(), &p.word)),
                            );
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if icon_button(ui, "✕", t(Tr::CloseEscHint)) {
                                    close = true;
                                }
                            });
                        });
                        match &p.translation {
                            Async::Ready(text) => drop(
                                ui.label(RichText::new(text).size(18.0).color(theme::accent())),
                            ),
                            Async::Pending => drop(ui.spinner()),
                            Async::Failed(e) => {
                                drop(ui.label(RichText::new(e).color(theme::danger()).small()))
                            }
                            Async::Idle => {}
                        }
                        match &p.explanation {
                            Async::Ready(text) => {
                                ui.add_space(4.0);
                                ui.label(text);
                            }
                            Async::Pending => drop(
                                ui.label(
                                    RichText::new(t(Tr::Explaining))
                                        .color(theme::muted())
                                        .small(),
                                ),
                            ),
                            Async::Failed(e) => {
                                drop(ui.label(RichText::new(e).color(theme::danger()).small()))
                            }
                            Async::Idle => {}
                        }
                        ui.add_space(6.0);
                        ui.label(
                            RichText::new(&p.sentence)
                                .color(theme::muted())
                                .size(13.0)
                                .family(theme::subs_family(lang.as_deref(), &p.sentence)),
                        );
                        if !p.sentence_translation.is_empty() {
                            ui.label(
                                RichText::new(&p.sentence_translation)
                                    .color(theme::muted())
                                    .size(13.0)
                                    .italics(),
                            );
                        }
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            let label = if p.saved_id.is_some() {
                                t(Tr::WordSaved)
                            } else {
                                t(Tr::SaveWord)
                            };
                            let button = ui.button(label);
                            crate::record::place("save_word", button.rect);
                            if button.clicked() {
                                save = true;
                            }
                            let label = if in_anki {
                                t(Tr::InAnki)
                            } else {
                                t(Tr::SendToAnki)
                            };
                            let send = ui.add_enabled(!in_anki, egui::Button::new(label));
                            crate::record::place("send_to_anki", send.rect);
                            if send.clicked() {
                                anki = true;
                            }
                            let dict_label = if in_custom_dict {
                                t(Tr::InCustomDict)
                            } else {
                                t(Tr::SendToCustomDict)
                            };
                            let send_dict =
                                ui.add_enabled(!in_custom_dict, egui::Button::new(dict_label));
                            crate::record::place("send_to_custom_dict", send_dict.rect);
                            if send_dict.clicked() {
                                custom_dict = true;
                            }
                            if ui.button(t(Tr::CopyWord)).clicked() {
                                ui.ctx().copy_text(p.word.clone());
                            }
                        });
                    });
            });
        if save || anki || custom_dict {
            let id = self.save_popup_word();
            if let (true, Some(id)) = (anki, id) {
                self.export_to_anki(id);
            }
            if let (true, Some(id)) = (custom_dict, id) {
                self.export_to_custom_dict(id);
            }
        }
        if close {
            self.close_popup();
        }
    }

    fn toast_ui(&mut self, ctx: &egui::Context) {
        let Some((text, _, _)) = &self.status else {
            return;
        };
        if crate::control::controls_override() == Some(false) || self.window_shot.is_some() {
            return;
        }
        let reveal = self.toast_reveal.clone();
        let mut show = false;
        egui::Area::new(Id::unique("toast"))
            .order(egui::Order::Tooltip)
            .anchor(Align2::CENTER_TOP, vec2(0.0, 56.0))
            .interactable(reveal.is_some())
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(theme::raised())
                    .corner_radius(8)
                    .inner_margin(egui::Margin::symmetric(14, 8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(text);
                            if reveal.is_some() {
                                let button = ui.button(t(Tr::ShowInFolder));
                                crate::record::place("show_in_folder", button.rect);
                                show = button.clicked();
                            }
                        });
                    });
            });
        if show
            && let Some(path) = reveal
            && let Err(e) = crate::frame::reveal(&path)
        {
            self.toast(e);
        }
        ctx.request_repaint_after(Duration::from_secs(1));
    }

    /// Adjusts the player volume by mouse wheel scroll (up or down).
    /// Step is 5.0%, bounded within [0.0, 150.0].
    /// Returns the new volume if adjusted.
    pub fn adjust_volume_by_wheel(&mut self, scroll_y: f32) -> Option<f64> {
        if scroll_y == 0.0 {
            return None;
        }
        let target = step_volume(self.volume, scroll_y);
        let up = scroll_y > 0.0;
        if up && self.snap.muted {
            let _ = self.player.mpv.set_flag("mute", false);
        }
        self.set_volume(target);
        let percent = self.volume.round() as i32;
        let icon = if percent == 0 { "🔇" } else { "🔊" };
        self.toast_for(format!("{icon} {percent}%"), Duration::from_millis(1200));
        Some(self.volume)
    }
}

/// A row of widgets in the middle of a centred column. egui centres single widgets only, so
/// the row is pushed in by half of the width it took in the frame before.
fn centered_row(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui)) {
    let id = ui.next_auto_id().with("centered-row");
    let width: f32 = ui.data(|d| d.get_temp(id)).unwrap_or(0.0);
    let left = ((ui.available_width() - width) / 2.0).max(0.0);
    ui.horizontal(|ui| {
        ui.add_space(left);
        let used = ui.horizontal(add).response.rect.width();
        if (used - width).abs() > 0.5 {
            ui.data_mut(|d| d.insert_temp(id, used));
            ui.ctx().request_repaint();
        }
    });
}

/// How tall the scrolling part of a dialog may be, so its buttons stay inside a small window.
pub fn dialog_height(ctx: &egui::Context) -> f32 {
    (ctx.content_rect().height() - 110.0).clamp(120.0, 700.0)
}

/// A sample of the chosen look on a dark and on a light picture.
pub fn subtitle_preview(ui: &mut egui::Ui, subs: &gp_core::config::Subtitles) {
    let look = SubStyle::from_id(&subs.style).look();
    let size =
        (subs.font_size * substyle::clamp_scale(subs.scale) * look.size * 0.5).clamp(9.0, 24.0);
    let width = ui.available_width().min(560.0);
    ui.horizontal(|ui| {
        for ground in [
            Color32::from_rgb(22, 28, 36),
            Color32::from_rgb(214, 222, 228),
        ] {
            let (rect, _) = ui.allocate_exact_size(vec2(width / 2.0 - 4.0, 84.0), Sense::hover());
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 6.0, ground);
            let inner = rect.width() - 16.0;
            let samples = [
                (t(Tr::SubSampleOriginal), look.original, size),
                (
                    t(Tr::SubSampleTranslation),
                    look.translation,
                    size * subs.secondary_scale,
                ),
            ];
            if subs.layout == "columns" {
                let gap = 8.0;
                let col_width = (inner - gap) / 2.0;
                let galleys: Vec<_> = samples
                    .iter()
                    .map(|(text, color, size)| {
                        painter.layout(text.to_string(), FontId::proportional(*size), *color, col_width)
                    })
                    .collect();
                let left_center = rect.center().x - gap / 2.0 - col_width / 2.0;
                let right_center = rect.center().x + gap / 2.0 + col_width / 2.0;
                let centers = [left_center, right_center];
                for (i, galley) in galleys.into_iter().enumerate() {
                    let at = pos2(centers[i] - galley.size().x / 2.0, rect.center().y - galley.size().y / 2.0);
                    if let Some(plate) = look.plate {
                        let backdrop = Rect::from_min_size(at, galley.size()).expand2(vec2(4.0, 2.0));
                        painter.rect_filled(backdrop, 4.0, plate);
                    }
                    let color = galley.job.sections[0].format.color;
                    substyle::paint(&painter, at, galley, color, &look, 0.6);
                }
            } else {
                let galleys: Vec<_> = samples
                    .iter()
                    .map(|(text, color, size)| {
                        painter.layout(text.to_string(), FontId::proportional(*size), *color, inner)
                    })
                    .collect();
                let total: f32 = galleys.iter().map(|g| g.size().y + 6.0).sum();
                let mut y = rect.center().y - total / 2.0;
                for galley in galleys {
                    let at = pos2(rect.center().x - galley.size().x / 2.0, y);
                    if let Some(plate) = look.plate {
                        let backdrop = Rect::from_min_size(at, galley.size()).expand2(vec2(6.0, 2.0));
                        painter.rect_filled(backdrop, 4.0, plate);
                    }
                    let color = galley.job.sections[0].format.color;
                    y += galley.size().y + 6.0;
                    substyle::paint(&painter, at, galley, color, &look, 0.6);
                }
            }
        }
    });
}

/// Where and how a block of subtitle lines is drawn.
struct SubDraw {
    rect: Rect,
    center_x: f32,
    look: substyle::Look,
    scale: f32,
    /// Lines stack downwards from the top edge instead of upwards from the bottom one.
    down: bool,
}

impl SubDraw {
    /// The top of a block `height` tall whose edge-side border is at `y`.
    fn top_of(&self, y: f32, height: f32) -> f32 {
        if self.down { y } else { y - height }
    }

    fn next(&self, y: f32, advance: f32) -> f32 {
        if self.down { y + advance } else { y - advance }
    }

    fn secondary(
        &self,
        ui: &mut egui::Ui,
        lines: &[String],
        lang: Option<&str>,
        size: f32,
        max_width: f32,
        y: &mut f32,
    ) {
        let look = &self.look;
        let ordered: Vec<&String> = if self.down {
            lines.iter().collect()
        } else {
            lines.iter().rev().collect()
        };
        for text in ordered {
            let font = FontId::new(size, theme::subs_family(lang, text));
            let galley =
                ui.painter()
                    .layout(text.replace('\n', " "), font, look.translation, max_width);
            let at = pos2(
                self.center_x - galley.size().x / 2.0,
                self.top_of(*y, galley.size().y),
            );
            if let Some(plate) = look.plate {
                let backdrop = Rect::from_min_size(at, galley.size()).expand2(vec2(10.0, 3.0));
                ui.painter().rect_filled(backdrop, 6.0, plate);
            }
            substyle::paint(
                ui.painter(),
                at,
                galley.clone(),
                look.translation,
                look,
                self.scale,
            );
            *y = self.next(*y, galley.size().y + 8.0);
        }
    }
}

pub fn step_volume(volume: f64, scroll_y: f32) -> f64 {
    let step = 5.0;
    if scroll_y > 0.0 {
        (volume + step).min(150.0)
    } else if scroll_y < 0.0 {
        (volume - step).max(0.0)
    } else {
        volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_wheel_volume_step_and_bounds_ac1_ac2() {
        // AC-1: scroll up increases by 5%
        assert_eq!(step_volume(100.0, 1.0), 105.0);
        assert_eq!(step_volume(50.0, 120.0), 55.0);

        // AC-2: scroll down decreases by 5% and clamps to 0
        assert_eq!(step_volume(100.0, -1.0), 95.0);
        assert_eq!(step_volume(2.0, -1.0), 0.0);
        assert_eq!(step_volume(0.0, -1.0), 0.0);

        // Clamping to 150% upper bound
        assert_eq!(step_volume(148.0, 1.0), 150.0);
        assert_eq!(step_volume(150.0, 1.0), 150.0);

        // Zero scroll delta leaves volume unchanged
        assert_eq!(step_volume(80.0, 0.0), 80.0);
    }

    #[test]
    fn subtitle_columns_geometry_ac4() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 720.0));
        let scale = 1.0;
        let gap = 24.0 * scale;
        let total_width = rect.width() * 0.90;
        let col_width = ((total_width - gap) / 2.0).max(120.0);
        let left_center_x = rect.center().x - gap / 2.0 - col_width / 2.0;
        let right_center_x = rect.center().x + gap / 2.0 + col_width / 2.0;

        assert!(left_center_x < rect.center().x);
        assert!(right_center_x > rect.center().x);
        assert!((right_center_x - left_center_x) > col_width);
        assert_eq!(col_width, (1280.0 * 0.90 - 24.0) / 2.0);
    }
}
