//! The Settings window: the list of tabs on the left, the chosen tab on the right, Save and
//! Cancel always in view under both. Also the small pieces the tabs are built from: a row
//! with its label, the "i" sign with a note, and the language picker.

use crate::app::App;
use crate::i18n::{self, Tr, t, tf};
use crate::keys::{self, Command};
use crate::substyle::{self, Position, SubStyle};
use crate::theme;
use eframe::egui::{self, Align, Align2, FontId, Id, Layout, RichText, Sense, Stroke, Vec2, vec2};
use gp_core::config::Config;
use gp_core::lang;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SettingsTab {
    #[default]
    Languages,
    Subtitles,
    Translation,
    SpeechRecognition,
    VoiceOver,
    Words,
    Sources,
    Player,
}

impl SettingsTab {
    pub const ALL: [SettingsTab; 8] = [
        SettingsTab::Languages,
        SettingsTab::Subtitles,
        SettingsTab::Translation,
        SettingsTab::SpeechRecognition,
        SettingsTab::VoiceOver,
        SettingsTab::Words,
        SettingsTab::Sources,
        SettingsTab::Player,
    ];

    /// The name MCP and the self-test use.
    pub fn id(self) -> &'static str {
        match self {
            SettingsTab::Languages => "languages",
            SettingsTab::Subtitles => "subtitles",
            SettingsTab::Translation => "translation",
            SettingsTab::SpeechRecognition => "speech_recognition",
            SettingsTab::VoiceOver => "voice_over",
            SettingsTab::Words => "words",
            SettingsTab::Sources => "sources",
            SettingsTab::Player => "player",
        }
    }

    pub fn parse(id: &str) -> Option<SettingsTab> {
        SettingsTab::ALL.into_iter().find(|tab| tab.id() == id)
    }

    pub fn title(self) -> &'static str {
        t(match self {
            SettingsTab::Languages => Tr::SectionLanguages,
            SettingsTab::Subtitles => Tr::SectionSubtitles,
            SettingsTab::Translation => Tr::SectionTranslation,
            SettingsTab::SpeechRecognition => Tr::SectionSpeechRecognition,
            SettingsTab::VoiceOver => Tr::SectionVoiceOver,
            SettingsTab::Words => Tr::SectionWords,
            SettingsTab::Sources => Tr::SectionSources,
            SettingsTab::Player => Tr::SectionPlayer,
        })
    }

    /// One or two lines on what the tab is for, under its title.
    fn intro(self) -> String {
        let key = |c: Command| keys::keys_of(c).into_iter().next().unwrap_or_default();
        match self {
            SettingsTab::Languages => t(Tr::IntroLanguages).to_owned(),
            SettingsTab::Subtitles => tf(
                Tr::IntroSubtitles,
                &[
                    &key(Command::SubtitlesSmaller),
                    &key(Command::SubtitlesLarger),
                ],
            ),
            SettingsTab::Translation => t(Tr::IntroTranslation).to_owned(),
            SettingsTab::SpeechRecognition => tf(Tr::IntroSpeech, &[&t(Tr::TranscribeSpeech)]),
            SettingsTab::VoiceOver => tf(Tr::IntroVoice, &[&t(Tr::MenuVoiceOver)]),
            SettingsTab::Words => tf(Tr::IntroWords, &[&key(Command::Sidebar)]),
            SettingsTab::Sources => t(Tr::SourcesNote).to_owned(),
            SettingsTab::Player => t(Tr::IntroPlayer).to_owned(),
        }
    }
}

/// Where the Save and Cancel buttons go.
const FOOTER: f32 = 40.0;

impl App {
    /// Opens Settings, at `tab` if given and otherwise where it was last. An open window
    /// keeps the changes made in it.
    pub fn open_settings(&mut self, tab: Option<SettingsTab>) {
        if let Some(tab) = tab {
            self.settings_tab = tab;
        }
        if self.settings_draft.is_none() {
            self.settings_draft = Some(self.new_settings_draft());
        }
    }

    pub fn settings_window(&mut self, ctx: &egui::Context) {
        let Some(mut draft) = self.settings_draft.take() else {
            return;
        };
        let (mut open, mut save, mut cancel, mut help) = (true, false, false, false);
        let (asr_setup, mut asr_action) = (&self.asr_setup, None);
        let (tts_setup, mut tts_action) = (&self.tts_setup, None);
        let (search_ui, mut check_sources) = (&self.search_ui, false);
        let mut tab = self.settings_tab;
        let size = window_size(ctx);
        egui::Window::new(t(Tr::SettingsTitle))
            .id(Id::unique("settings-window"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .order(egui::Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .fixed_size(size)
            .show(ctx, |ui| {
                let body = size.y - FOOTER;
                let nav = nav_width(ui, size.x);
                // All the tabs stay in view down to the smallest window.
                let gap = ui.spacing().item_spacing.y;
                let row = (body / SettingsTab::ALL.len() as f32 - gap).clamp(22.0, 30.0);
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        vec2(nav, body),
                        Layout::top_down_justified(Align::Min),
                        |ui| {
                            ui.set_min_size(vec2(nav, body));
                            egui::ScrollArea::vertical()
                                .id_salt("settings-nav")
                                .max_height(body)
                                .show(ui, |ui| {
                                    for each in SettingsTab::ALL {
                                        if nav_item(ui, each == tab, each.title(), row) {
                                            tab = each;
                                        }
                                    }
                                });
                        },
                    );
                    ui.separator();
                    let width = ui.available_width();
                    ui.allocate_ui_with_layout(
                        vec2(width, body),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.set_min_size(vec2(width, body));
                            egui::ScrollArea::vertical()
                                .id_salt(("settings-tab", tab.id()))
                                .max_height(body)
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    // Room for the scroll bar, so nothing runs under it.
                                    ui.set_width(ui.available_width() - 14.0);
                                    header(ui, tab);
                                    let form = Form::new(ui);
                                    match tab {
                                        SettingsTab::Languages => languages(ui, &form, &mut draft),
                                        SettingsTab::Subtitles => subtitles(ui, &form, &mut draft),
                                        SettingsTab::Translation => {
                                            translation(ui, &form, &mut draft)
                                        }
                                        SettingsTab::SpeechRecognition => {
                                            let (action, change) =
                                                speech(ui, &form, &mut draft, asr_setup);
                                            asr_action = action;
                                            if change {
                                                tab = SettingsTab::Languages;
                                            }
                                        }
                                        SettingsTab::VoiceOver => {
                                            tts_action = crate::tts_ui::settings_section(
                                                ui, &form, &mut draft, tts_setup,
                                            );
                                        }
                                        SettingsTab::Words => words(ui, &form, &mut draft),
                                        SettingsTab::Sources => {
                                            check_sources = crate::search_ui::sources_section(
                                                ui, &form, &mut draft, search_ui,
                                            );
                                        }
                                        SettingsTab::Player => player(ui, &form, &mut draft),
                                    }
                                    ui.add_space(8.0);
                                });
                        },
                    );
                });
                ui.separator();
                ui.horizontal(|ui| {
                    let button = ui.button(RichText::new(t(Tr::SaveButton)).strong());
                    crate::record::place("settings_save", button.rect);
                    save = button.clicked();
                    cancel = ui.button(t(Tr::Cancel)).clicked();
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        help = info_link(ui, t(Tr::HelpTitle));
                    });
                });
            });
        if matches!(tts_action, Some(crate::tts_ui::Action::Languages)) {
            tab = SettingsTab::Languages;
            tts_action = None;
        }
        self.settings_tab = tab;
        if help {
            self.help_open = true;
        }
        if check_sources {
            self.check_sources(draft.sources.clone());
        }
        if let Some(action) = asr_action {
            self.asr_settings_action(action, &draft);
        }
        if let Some(action) = tts_action {
            self.tts_settings_action(action, &draft);
        }
        if save {
            self.apply_settings(draft);
        } else if open && !cancel {
            self.settings_draft = Some(draft);
        }
    }
}

/// The inside of the window: as large as is comfortable, never larger than the screen.
fn window_size(ctx: &egui::Context) -> Vec2 {
    let area = ctx.content_rect().size();
    vec2(
        (area.x - 40.0).clamp(300.0, 800.0),
        (area.y - 84.0).clamp(160.0, 560.0),
    )
}

/// Wide enough for the longest tab name, but never more than a third of the window.
fn nav_width(ui: &egui::Ui, window: f32) -> f32 {
    let font = egui::TextStyle::Button.resolve(ui.style());
    let widest = SettingsTab::ALL
        .iter()
        .map(|tab| {
            ui.painter()
                .layout_no_wrap(tab.title().to_owned(), font.clone(), theme::text())
                .size()
                .x
        })
        .fold(0.0, f32::max);
    (widest + 28.0).clamp(110.0, (window / 3.0).max(110.0))
}

fn nav_item(ui: &mut egui::Ui, selected: bool, title: &str, height: f32) -> bool {
    let text = if selected {
        RichText::new(title).strong().color(theme::accent())
    } else {
        RichText::new(title)
    };
    let r = ui.add(
        egui::Button::selectable(selected, text)
            .wrap()
            .min_size(vec2(ui.available_width(), height)),
    );
    crate::record::place(&format!("settings_tab_{title}"), r.rect);
    r.clicked()
}

fn header(ui: &mut egui::Ui, tab: SettingsTab) {
    ui.label(RichText::new(tab.title()).size(18.0).strong());
    ui.add_space(2.0);
    ui.label(RichText::new(tab.intro()).color(theme::muted()));
    ui.add_space(10.0);
}

/// Rows of a tab: a label column on the left, the field on the right, both sized to the
/// width there is, so a small window wraps the labels instead of cutting the fields off.
pub struct Form {
    label: f32,
    pub field: f32,
}

impl Form {
    fn new(ui: &egui::Ui) -> Form {
        let width = ui.available_width();
        let label = (width * 0.36).clamp(100.0, 210.0);
        Form {
            label,
            field: (width - label - 12.0).clamp(120.0, 340.0),
        }
    }

    /// A labelled row; `note` puts the "i" sign after the label.
    pub fn row<R>(
        &self,
        ui: &mut egui::Ui,
        label: &str,
        note: Option<&str>,
        add: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        let r = ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                vec2(self.label, 20.0),
                Layout::left_to_right(Align::Center).with_main_wrap(true),
                |ui| {
                    ui.set_width(self.label);
                    ui.label(label);
                    if let Some(note) = note {
                        info(ui, note);
                    }
                },
            );
            add(ui)
        });
        ui.add_space(4.0);
        r.inner
    }

    /// A checkbox across the tab, with its note after it.
    pub fn check(&self, ui: &mut egui::Ui, value: &mut bool, text: &str, note: Option<&str>) {
        ui.horizontal(|ui| {
            let width = self.label + self.field - 12.0;
            let text_w = ui
                .painter()
                .layout_no_wrap(
                    text.to_owned(),
                    egui::TextStyle::Body.resolve(ui.style()),
                    theme::text(),
                )
                .size()
                .x;
            let boxed = (text_w + 2.0 * ui.spacing().icon_width).min(width);
            ui.allocate_ui_with_layout(vec2(boxed, 20.0), Layout::top_down(Align::Min), |ui| {
                ui.set_max_width(boxed);
                ui.checkbox(value, text);
            });
            if let Some(note) = note {
                info(ui, note);
            }
        });
        ui.add_space(4.0);
    }

    pub fn slider(&self, ui: &mut egui::Ui, slider: egui::Slider) -> egui::Response {
        ui.spacing_mut().slider_width = (self.field - 64.0).max(60.0);
        ui.add(slider)
    }

    pub fn text(&self, ui: &mut egui::Ui, value: &mut String, hint: &str) -> egui::Response {
        ui.add(
            egui::TextEdit::singleline(value)
                .hint_text(hint)
                .desired_width(self.field),
        )
    }

    pub fn secret(&self, ui: &mut egui::Ui, value: &mut String) -> egui::Response {
        ui.add(
            egui::TextEdit::singleline(value)
                .password(true)
                .desired_width(self.field),
        )
    }

    /// A folder field with a Browse button; `hint` shows what an empty field means.
    pub fn folder(&self, ui: &mut egui::Ui, value: &mut String, hint: &str) {
        self.browse(ui, value, hint, || rfd::FileDialog::new().pick_folder());
    }

    pub fn path(&self, ui: &mut egui::Ui, value: &mut String) {
        self.browse(ui, value, t(Tr::PathAutoHint), || {
            rfd::FileDialog::new().pick_file()
        });
    }

    fn browse(
        &self,
        ui: &mut egui::Ui,
        value: &mut String,
        hint: &str,
        pick: impl FnOnce() -> Option<std::path::PathBuf>,
    ) {
        ui.horizontal(|ui| {
            let browse = t(Tr::BrowseButton);
            let font = egui::TextStyle::Button.resolve(ui.style());
            let button = ui
                .painter()
                .layout_no_wrap(browse.to_owned(), font, theme::text())
                .size()
                .x
                + 2.0 * ui.spacing().button_padding.x;
            ui.add(
                egui::TextEdit::singleline(value)
                    .hint_text(hint)
                    .desired_width(self.field - button - ui.spacing().item_spacing.x),
            );
            if ui.button(browse).clicked()
                && let Some(path) = pick()
            {
                *value = path.to_string_lossy().into_owned();
            }
        });
    }
}

/// The "i" sign: its note shows on hover, and a click keeps it open until the next click.
pub fn info(ui: &mut egui::Ui, note: &str) {
    let size = 16.0;
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    let color = if resp.hovered() {
        theme::accent()
    } else {
        theme::muted()
    };
    paint_info(ui.painter(), rect.center(), size / 2.0 - 1.0, color);
    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
    let popup = egui::Popup::from_toggle_button_response(&resp)
        .width(280.0)
        .show(|ui| {
            ui.set_max_width(280.0);
            ui.label(note);
        });
    if popup.is_none() {
        resp.on_hover_ui(|ui| {
            ui.set_max_width(280.0);
            ui.label(note);
        });
    }
}

/// A circle with an "i" in it, drawn rather than taken from a font that may not have it.
pub fn paint_info(painter: &egui::Painter, center: egui::Pos2, radius: f32, color: egui::Color32) {
    painter.circle_stroke(center, radius, Stroke::new(1.3, color));
    let dot = radius * 0.16;
    painter.circle_filled(center + vec2(0.0, -radius * 0.45), dot.max(1.1), color);
    painter.line_segment(
        [
            center + vec2(0.0, -radius * 0.12),
            center + vec2(0.0, radius * 0.55),
        ],
        Stroke::new((radius * 0.2).max(1.4), color),
    );
}

/// A link with the "i" sign before it.
pub fn info_link(ui: &mut egui::Ui, text: &str) -> bool {
    // Its own left-to-right box, so the sign comes first in a right-to-left row too.
    let font = egui::TextStyle::Body.resolve(ui.style());
    let text_w = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font, theme::text())
        .size()
        .x;
    let width = 16.0 + ui.spacing().item_spacing.x + text_w + 1.0;
    let r = ui.allocate_ui_with_layout(
        vec2(width, 18.0),
        Layout::left_to_right(Align::Center),
        |ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
            paint_info(ui.painter(), rect.center(), 7.0, theme::accent());
            ui.link(text)
        },
    );
    let clicked = r.inner.clicked();
    crate::record::place("help_link", r.response.rect);
    clicked
}

fn languages(ui: &mut egui::Ui, form: &Form, draft: &mut Config) {
    let g = &mut draft.general;
    form.row(ui, t(Tr::RowVideoLang), Some(t(Tr::InfoLearn)), |ui| {
        language_choice(ui, "learn-lang", &mut g.learn_lang, true, form.field);
    });
    form.row(ui, t(Tr::RowMyLang), Some(t(Tr::InfoNative)), |ui| {
        language_choice(ui, "native-lang", &mut g.native_lang, false, form.field);
    });
    form.row(
        ui,
        t(Tr::InterfaceLanguage),
        Some(t(Tr::InfoInterface)),
        |ui| {
            egui::ComboBox::from_id_salt("ui-lang")
                .width(form.field)
                .selected_text(ui_lang_name(&g.ui_lang))
                .show_ui(ui, |ui| {
                    for code in ["auto", "en", "ru", "uk", "zh"] {
                        ui.selectable_value(&mut g.ui_lang, code.to_owned(), ui_lang_name(code));
                    }
                });
        },
    );
}

/// Each interface language is named in itself, so it can be found whatever the interface says.
fn ui_lang_name(code: &str) -> &'static str {
    match lang::find(code) {
        // There is only one Chinese interface, so the script needs no mention here.
        Some(_) if code == "zh" => "中文",
        Some(l) if code != "auto" => l.native,
        _ => t(Tr::LangAuto),
    }
}

/// The language picker. A code the list does not have is kept and shown as it is.
pub fn language_choice(
    ui: &mut egui::Ui,
    salt: &str,
    value: &mut String,
    detect: bool,
    width: f32,
) {
    let current = value.trim().to_owned();
    let known = lang::find(&current).map(|l| l.code);
    let picker = egui::ComboBox::from_id_salt(salt)
        .width(width)
        .height(340.0)
        .selected_text(language_text(ui, &current))
        .show_ui(ui, |ui| {
            if detect
                && ui
                    .selectable_label(current.is_empty(), t(Tr::LangDetect))
                    .clicked()
            {
                value.clear();
            }
            if known.is_none() && !current.is_empty() {
                let _ = ui.selectable_label(true, language_text(ui, &current));
            }
            for l in &lang::LANGUAGES {
                if ui
                    .selectable_label(known == Some(l.code), language_text(ui, l.code))
                    .clicked()
                {
                    *value = l.code.to_owned();
                }
            }
        });
    let shown: Option<String> = ui.data(|d| d.get_temp(Id::unique(OPEN_PICKER)));
    if shown.as_deref() == Some(salt) {
        egui::Popup::open_id(ui.ctx(), egui::Popup::default_response_id(&picker.response));
    }
}

/// A self-test sets this to the salt of a language picker to have its list open.
pub const OPEN_PICKER: &str = "selftest-open-picker";

/// "Deutsch — German": the language's own name, then its name in the interface language.
/// A name the fonts cannot draw, or a script that needs shaping, gives way to the second.
pub fn language_text(ui: &egui::Ui, code: &str) -> egui::Atoms<'static> {
    let mut atoms = egui::Atoms::new(());
    if code.is_empty() {
        atoms.push_right(t(Tr::LangDetect));
        return atoms;
    }
    let Some(l) = lang::find(code) else {
        atoms.push_right(code.to_owned());
        if let Some(name) = i18n::language_name(&lang::normalize(code)) {
            atoms.push_right(RichText::new(format!("— {}", t(name))).color(theme::muted()));
        }
        return atoms;
    };
    let ours = i18n::language_name(l.code).map_or(l.english, t);
    // Japanese fonts lack simplified characters, and a name drawn half in one font and half
    // in another looks broken, so "中文 (简体)" takes the Chinese font. "日本語" takes it too:
    // its characters are there, and the Japanese fonts sit higher than the rest of the row.
    let family = if matches!(l.code, "zh" | "ja") {
        theme::subs_family(Some("zh"), l.native)
    } else {
        egui::FontFamily::Proportional
    };
    let font = FontId::new(
        egui::TextStyle::Body.resolve(ui.style()).size,
        family.clone(),
    );
    let drawable = !l.shaped && ui.ctx().fonts_mut(|f| f.has_glyphs(&font, l.native));
    if drawable {
        atoms.push_right(RichText::new(l.native).family(family));
        // "中文 (简体)" needs no "— 中文" after it.
        if !l.native.to_lowercase().starts_with(&ours.to_lowercase()) {
            atoms.push_right(RichText::new(format!("— {ours}")).color(theme::muted()));
        }
    } else {
        atoms.push_right(ours);
    }
    atoms
}

fn subtitles(ui: &mut egui::Ui, form: &Form, draft: &mut Config) {
    let s = &mut draft.subtitles;
    form.row(ui, t(Tr::TextSize), None, |ui| {
        form.slider(ui, egui::Slider::new(&mut s.font_size, 16.0..=60.0));
    });
    form.row(ui, t(Tr::TranslationSize), None, |ui| {
        form.slider(ui, egui::Slider::new(&mut s.secondary_scale, 0.5..=1.2));
    });
    form.row(ui, t(Tr::SubSizeLabel), None, |ui| {
        form.slider(
            ui,
            egui::Slider::new(&mut s.scale, substyle::SCALE_MIN..=substyle::SCALE_MAX)
                .step_by(f64::from(substyle::SCALE_STEP))
                .custom_formatter(|v, _| format!("{:.0}%", v * 100.0))
                .custom_parser(|s| {
                    s.trim()
                        .trim_end_matches('%')
                        .parse::<f64>()
                        .ok()
                        .map(|p| p / 100.0)
                }),
        );
    });
    form.row(ui, t(Tr::SubStyleLabel), None, |ui| {
        let current = SubStyle::from_id(&s.style);
        egui::ComboBox::from_id_salt("sub-style")
            .width(form.field)
            .selected_text(t(current.name()))
            .show_ui(ui, |ui| {
                for style in SubStyle::ALL {
                    ui.selectable_value(&mut s.style, style.id().to_owned(), t(style.name()));
                }
            });
    });
    form.row(ui, t(Tr::SubPositionLabel), None, |ui| {
        let current = Position::from_id(&s.position);
        egui::ComboBox::from_id_salt("sub-position")
            .width(form.field)
            .selected_text(t(current.name()))
            .show_ui(ui, |ui| {
                for place in [Position::Bottom, Position::Top] {
                    ui.selectable_value(&mut s.position, place.id().to_owned(), t(place.name()));
                }
            });
    });
    form.row(ui, t(Tr::SubLayoutLabel), None, |ui| {
        let current = gp_core::config::SubtitleLayout::from_id(&s.layout);
        let layout_name = |layout: gp_core::config::SubtitleLayout| match layout {
            gp_core::config::SubtitleLayout::Rows => t(Tr::SubLayoutRows),
            gp_core::config::SubtitleLayout::Columns => t(Tr::SubLayoutColumns),
        };
        egui::ComboBox::from_id_salt("sub-layout")
            .width(form.field)
            .selected_text(layout_name(current))
            .show_ui(ui, |ui| {
                for layout in gp_core::config::SubtitleLayout::ALL {
                    ui.selectable_value(&mut s.layout, layout.id().to_owned(), layout_name(layout));
                }
            });
    });
    form.row(ui, t(Tr::SubMarginLabel), None, |ui| {
        form.slider(ui, egui::Slider::new(&mut s.margin, 0.0..=150.0));
    });
    ui.add_space(4.0);
    ui.label(
        RichText::new(t(Tr::SubPreviewLabel))
            .color(theme::muted())
            .small(),
    );
    crate::ui::subtitle_preview(ui, s);
}

fn provider_name(id: &str) -> &'static str {
    match id {
        "google" => t(Tr::ProviderGoogle),
        "deepl" => "DeepL",
        "openai" => t(Tr::ProviderOpenAi),
        "anthropic" => "Claude",
        _ => t(Tr::ProviderUnknown),
    }
}

fn translation(ui: &mut egui::Ui, form: &Form, draft: &mut Config) {
    let tr = &mut draft.translation;
    form.row(ui, t(Tr::ServiceRow), None, |ui| {
        egui::ComboBox::from_id_salt("provider")
            .width(form.field)
            .selected_text(provider_name(&tr.provider))
            .show_ui(ui, |ui| {
                for id in ["google", "deepl", "openai", "anthropic"] {
                    ui.selectable_value(&mut tr.provider, id.to_owned(), provider_name(id));
                }
            });
    });
    if matches!(tr.provider.as_str(), "openai" | "anthropic") {
        let hint = if tr.provider == "anthropic" {
            "claude-opus-5-5"
        } else {
            t(Tr::ModelNameHint)
        };
        form.row(ui, t(Tr::ModelRow), None, |ui| {
            form.text(ui, &mut tr.model, hint);
        });
    }
    if tr.provider == "openai" {
        form.row(ui, t(Tr::ServerAddressRow), None, |ui| {
            form.text(ui, &mut tr.base_url, t(Tr::OpenAiAddressHint));
        });
    }
    if tr.provider != "google" {
        form.row(ui, t(Tr::ApiKeyRow), Some(t(Tr::InfoApiKey)), |ui| {
            form.secret(ui, &mut tr.api_key);
        });
    }
    form.check(
        ui,
        &mut draft.subtitles.auto_translate,
        t(Tr::AutoTranslateCheck),
        Some(t(Tr::InfoAutoTranslate)),
    );
}

/// Returns what the setup buttons asked for, and whether "Change…" was pressed.
fn speech(
    ui: &mut egui::Ui,
    form: &Form,
    draft: &mut Config,
    s: &crate::setup_ui::AsrSetup,
) -> (Option<crate::setup_ui::Action>, bool) {
    let action = crate::setup_ui::settings_section(ui, draft, s);
    let change = language_line(
        ui,
        &if draft.general.learn_lang.trim().is_empty() {
            t(Tr::AsrListensAny).to_owned()
        } else {
            tf(
                Tr::AsrListensFor,
                &[&i18n::language_label(&draft.general.learn_lang)],
            )
        },
    );
    ui.add_space(6.0);
    form.check(
        ui,
        &mut draft.asr.enabled,
        t(Tr::AsrAutoCheck),
        Some(t(Tr::InfoAsrAuto)),
    );
    egui::CollapsingHeader::new(RichText::new(t(Tr::Advanced)).color(theme::muted()))
        .id_salt("asr-advanced")
        .show(ui, |ui| {
            form.row(ui, "whisper-server", None, |ui| {
                form.path(ui, &mut draft.asr.server_path)
            });
            form.row(ui, t(Tr::ModelFileRow), None, |ui| {
                form.path(ui, &mut draft.asr.model_path)
            });
        });
    (action, change)
}

/// A muted line about the language a feature uses, with a link to the Languages tab.
pub fn language_line(ui: &mut egui::Ui, text: &str) -> bool {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(text).color(theme::muted()));
        ui.link(t(Tr::ChangeLanguage)).clicked()
    })
    .inner
}

fn words(ui: &mut egui::Ui, form: &Form, draft: &mut Config) {
    ui.label(RichText::new("Anki").strong());
    form.row(ui, t(Tr::DeckRow), None, |ui| {
        form.text(ui, &mut draft.anki.deck, "Glossplay");
    });
    form.row(ui, t(Tr::AnkiAddressRow), Some(t(Tr::InfoAnki)), |ui| {
        form.text(ui, &mut draft.anki.url, "http://127.0.0.1:8765");
    });
    crate::anki_ui::section(ui, form, draft);

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    ui.label(RichText::new(t(Tr::SectionCustomDict)).strong());
    form.row(ui, t(Tr::CustomDictAddressRow), None, |ui| {
        form.text(
            ui,
            &mut draft.custom_dict.url,
            "http://127.0.0.1:8000/api/words",
        );
    });
    form.row(ui, t(Tr::CustomDictTokenRow), None, |ui| {
        form.text(ui, &mut draft.custom_dict.token, "");
    });
    crate::custom_dict_ui::section(ui, form, draft);
}

fn player(ui: &mut egui::Ui, form: &Form, draft: &mut Config) {
    form.row(ui, t(Tr::SeekSecondsRow), Some(t(Tr::InfoSeekSeconds)), |ui| {
        form.slider(
            ui,
            egui::Slider::new(&mut draft.player.seek_seconds, 1.0..=60.0).step_by(1.0),
        );
    });
    form.row(ui, t(Tr::HwdecRow), Some(t(Tr::InfoHwdec)), |ui| {
        form.text(ui, &mut draft.player.hwdec, "auto");
    });
    form.row(ui, "ffmpeg", Some(t(Tr::InfoFfmpeg)), |ui| {
        form.path(ui, &mut draft.player.ffmpeg_path)
    });
    form.row(
        ui,
        t(Tr::FramesFolderRow),
        Some(t(Tr::InfoFramesFolder)),
        |ui| {
            let default = crate::frame::default_folder();
            form.folder(ui, &mut draft.player.frames_dir, &default.to_string_lossy());
        },
    );
    form.row(
        ui,
        t(Tr::ComponentsFolderRow),
        Some(t(Tr::InfoComponentsFolder)),
        |ui| {
            let default = crate::components::default_root();
            form.folder(
                ui,
                &mut draft.general.components_dir,
                &default.to_string_lossy(),
            );
        },
    );
    if let Some(free) = crate::components::free_space(&match draft.general.components_dir.trim() {
        "" => crate::components::default_root(),
        dir => std::path::PathBuf::from(dir),
    }) {
        ui.label(
            RichText::new(tf(Tr::FreeOnDisk, &[&crate::setup_text::size(free)]))
                .color(theme::muted())
                .small(),
        );
    }
    form.row(ui, t(Tr::InterfaceScaleRow), Some(t(Tr::InfoScale)), |ui| {
        form.slider(
            ui,
            egui::Slider::new(&mut draft.general.ui_scale, 0.75..=2.0),
        );
    });
    form.row(ui, t(Tr::ThemeRow), None, |ui| {
        let choices = [
            ("dark", Tr::ThemeDark),
            ("light", Tr::ThemeLight),
            ("system", Tr::ThemeSystem),
        ];
        let shown = choices
            .iter()
            .find(|(id, _)| *id == draft.general.theme)
            .map_or(Tr::ThemeDark, |(_, name)| *name);
        egui::ComboBox::from_id_salt("theme")
            .width(form.field)
            .selected_text(t(shown))
            .show_ui(ui, |ui| {
                for (id, name) in choices {
                    ui.selectable_value(&mut draft.general.theme, id.to_owned(), t(name));
                }
            });
    });
    if crate::shell::SUPPORTED {
        form.check(
            ui,
            &mut draft.general.shell_integration,
            t(Tr::ShellCheck),
            None,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tab_is_found_by_its_name() {
        for tab in SettingsTab::ALL {
            assert_eq!(SettingsTab::parse(tab.id()), Some(tab));
        }
        assert_eq!(SettingsTab::parse("top"), None);
    }

    #[test]
    fn subtitle_layout_names_are_available() {
        for layout in gp_core::config::SubtitleLayout::ALL {
            let name = match layout {
                gp_core::config::SubtitleLayout::Rows => t(Tr::SubLayoutRows),
                gp_core::config::SubtitleLayout::Columns => t(Tr::SubLayoutColumns),
            };
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn seek_seconds_labels_are_translated() {
        assert!(!t(Tr::SeekSecondsRow).is_empty());
        assert!(!t(Tr::InfoSeekSeconds).is_empty());
    }
}
