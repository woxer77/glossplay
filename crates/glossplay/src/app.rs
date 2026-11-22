use crate::i18n::{self, Tr, t, tf};
use crate::jobs::{AsrParams, CONTEXT_SENTENCES, Jobs, Msg, TranslateJob};
use crate::player::{Player, Snapshot, TrackInfo, TrackKind};
use crate::session::{
    Load, Origin, Secondary, Session, SubTrack, normalize_lang, sidecar_subtitles,
};
use crate::video::VideoSurface;
use crate::{paths, selftest, shell, theme};
use eframe::egui;
use gp_ai::translate::{Translator, build_translator};
use gp_core::asr_plan;
use gp_core::config::Config;
use gp_core::training::{TrainAction, Trainer};
use gp_core::{Ms, Token};
use gp_media::mpv::{Event, RenderContext};
use gp_media::tools;
use gp_store::{NewWord, Store, VocabEntry};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

pub enum Async {
    Idle,
    Pending,
    Ready(String),
    Failed(String),
}

pub struct WordPopup {
    pub request: u64,
    pub word: String,
    /// (cue, token index) of all selected words in order
    pub tokens: Vec<(usize, usize)>,
    pub sentence: String,
    pub sentence_translation: String,
    pub cue: (Ms, Ms),
    pub anchor: egui::Pos2,
    pub translation: Async,
    pub explanation: Async,
    pub saved_id: Option<i64>,
    pub resume_on_close: bool,
}

impl WordPopup {
    #[allow(dead_code)]
    pub fn token(&self) -> (usize, usize) {
        self.tokens.first().copied().unwrap_or((0, 0))
    }

    pub fn contains_token(&self, cue: usize, token: usize) -> bool {
        self.tokens.contains(&(cue, token))
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum SidebarTab {
    Transcript,
    Words,
}

pub struct Launch {
    pub media: Option<String>,
    pub selftest: Option<selftest::Plan>,
}

pub struct App {
    pub cfg: Config,
    pub player: Player,
    pub video: VideoSurface,
    pub jobs: Jobs,
    pub session: Option<Session>,
    generation: u64,
    pub snap: Snapshot,
    prev_pos: Ms,
    pub trainer: Trainer,
    pub translator: Result<Arc<dyn Translator>, String>,
    pub ffmpeg: Option<PathBuf>,
    pub saved_words: HashSet<String>,
    pub vocab: Vec<VocabEntry>,
    pub tokens: HashMap<(usize, usize), Rc<Vec<Token>>>,
    pub popup: Option<WordPopup>,
    next_request: u64,
    pub sidebar_open: bool,
    pub sidebar_tab: SidebarTab,
    pub search: String,
    pub scrolled_to: Option<usize>,
    /// Cues of the transcript drawn in the sidebar in the last frame; empty when it is hidden.
    pub panel_cues: Vec<usize>,
    /// A translation asked for the sidebar has not come back yet.
    panel_busy: bool,
    /// Translation jobs sent, for the self-test report.
    pub translate_jobs: usize,
    pub settings_draft: Option<Config>,
    /// The tab Settings shows; it stays while the program runs.
    pub settings_tab: crate::settings_ui::SettingsTab,
    /// The "How to use" window is open.
    pub help_open: bool,
    /// The theme setting changed; the next frame sets the colours again.
    pub restyle: bool,
    /// The interface scale changed in Settings and is applied in the next frame.
    pub rescale: bool,
    /// The fonts are ordered for a Chinese interface.
    pub fonts_chinese: bool,
    pub url_input: String,
    /// A link opened before yt-dlp finished downloading; played as soon as it is there.
    pending_url: Option<String>,
    pub status: Option<(String, Instant, Duration)>,
    /// The config on disk did not parse; the first save keeps a copy of it.
    config_broken: bool,
    /// The last message shown, kept for the self-test report.
    pub last_toast: String,
    /// The config error stays up until this moment; other toasts wait behind it.
    config_toast_until: Option<Instant>,
    queued_toast: Option<(String, Duration)>,
    pub last_activity: Instant,
    pub fullscreen: bool,
    pub hwdec_current: String,
    pub audio_tracks: Vec<TrackInfo>,
    pub selftest: Option<selftest::State>,
    pub asr_setup: crate::setup_ui::AsrSetup,
    pub tts_setup: crate::tts_ui::TtsSetup,
    pub search_ui: crate::search_ui::SearchUi,
    /// The voice-over, while it is on.
    pub dub: Option<crate::dub::Dub>,
    /// The volume the user set; the player plays quieter while the voice-over is on.
    pub volume: f64,
    /// A self-test run: nothing is heard and no choice is saved. Kept apart from `selftest`,
    /// which is taken out of the app while the test steps.
    pub quiet: bool,
    pub dub_save: Option<std::path::PathBuf>,
    /// What the shortcuts of this frame did, for the self-test.
    pub key_log: Vec<String>,
    /// What mpv says about the cache of a network stream.
    pub cache: crate::buffering::Cache,
    /// The "Buffering" overlay is up.
    pub buffering_shown: bool,
    dub_wait: crate::buffering::Delay,
    /// The film stands still for a voice-over clip, long enough to say so.
    pub dub_wait_shown: bool,
    /// Where the video area was drawn in the last frame.
    pub stage_rect: egui::Rect,
    /// The context menu is open there.
    pub menu_at: Option<egui::Pos2>,
    /// A right click asked for the context menu at this point.
    pub menu_request: Option<egui::Pos2>,
    /// Where the context menu drew its submenus, for the self-test.
    pub menu_rects: Vec<(&'static str, egui::Rect)>,
    pub on_top: bool,
    /// The "Video info" window is open.
    pub info_open: bool,
    pub info_view: Option<crate::info::InfoView>,
    pub window_shot: Option<crate::frame::WindowShot>,
    pub frame_requests: u64,
    /// The last frame saved: its request and the file, or why it failed.
    pub frame_done: Option<(u64, Result<PathBuf, String>)>,
    /// The message on screen names a file that can be shown in its folder.
    pub toast_reveal: Option<PathBuf>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, launch: Launch) -> Result<App, String> {
        let (mut cfg, config_error) = paths::load_config();
        if let Some(plan) = &launch.selftest {
            if let Some(v) = &plan.theme {
                cfg.general.theme = v.clone();
            }
            let subs = &mut cfg.subtitles;
            if let Some(v) = &plan.sub_style {
                subs.style = v.clone();
            }
            if let Some(v) = plan.sub_scale {
                subs.scale = v;
            }
            if let Some(v) = &plan.sub_position {
                subs.position = v.clone();
            }
            if let Some(v) = plan.sub_margin {
                subs.margin = v;
            }
        }
        // A self-test has its language set already; the config must not change it.
        if launch.selftest.is_none() {
            i18n::apply_setting(&cfg.general.ui_lang);
        }
        let fonts_chinese = i18n::lang() == i18n::Lang::Zh;
        theme::install_fonts(&cc.egui_ctx, fonts_chinese);
        theme::apply(&cc.egui_ctx, cfg.general.ui_scale, &cfg.general.theme);
        crate::components::set_root(&cfg.general.components_dir);

        let player = Player::new(&cfg.player).map_err(|e| e.to_string())?;
        let quiet = launch.selftest.is_some();
        let dub_save = launch.selftest.as_ref().and_then(|p| p.dub_save.clone());
        // A recording sends the sound into files, and must keep it to be of any use.
        if quiet && !crate::record::on() {
            player.mpv.set_flag("mute", true);
        }
        let loader = cc
            .get_proc_address
            .clone()
            .ok_or("the OpenGL backend is not available")?;
        let repaint = cc.egui_ctx.clone();
        let render = RenderContext::new(
            player.mpv.clone(),
            loader,
            Box::new(move || repaint.request_repaint()),
        )
        .map_err(|e| e.to_string())?;

        // A damaged database only costs the cache, so I set it aside and start a fresh one.
        let store = Store::open(&paths::db_path()).or_else(|_| {
            let _ = std::fs::rename(
                paths::db_path(),
                paths::db_path().with_extension("db.broken"),
            );
            Store::open(&paths::db_path())
        });
        let store = store
            .or_else(|_| Store::open_in_memory())
            .map_err(|e| e.to_string())?;
        let vocab = store.list_words().unwrap_or_default();
        std::thread::spawn(gp_core::tokens::warm_up);
        let jobs = Jobs::new(cc.egui_ctx.clone(), store);
        if cfg.player.ytdl_path.is_empty() {
            jobs.spawn(|| Msg::YtDlp(crate::ytdlp::ensure(false)));
        }

        // Automated runs with a private data dir must not touch the user's registry.
        let isolated_selftest =
            launch.selftest.is_some() && std::env::var_os(paths::ENV_CONFIG_DIR).is_some();
        if cfg.general.shell_integration && shell::SUPPORTED && !isolated_selftest {
            std::thread::spawn(|| {
                if let Err(e) = shell::refresh_if_stale() {
                    eprintln!("shell integration: {e}");
                }
            });
        }

        let mut app = App {
            translator: make_translator(&cfg),
            ffmpeg: tools::locate("ffmpeg", &cfg.player.ffmpeg_path).ok(),
            trainer: Trainer::new(cfg.training.autopause),
            saved_words: vocab.iter().map(|v| v.word.word.to_lowercase()).collect(),
            vocab,
            player,
            video: VideoSurface::new(render),
            jobs,
            pending_url: None,
            session: None,
            generation: 0,
            snap: Snapshot::default(),
            prev_pos: 0,
            tokens: HashMap::new(),
            popup: None,
            next_request: 0,
            sidebar_open: false,
            sidebar_tab: SidebarTab::Transcript,
            search: String::new(),
            scrolled_to: None,
            panel_cues: Vec::new(),
            panel_busy: false,
            translate_jobs: 0,
            settings_draft: None,
            settings_tab: Default::default(),
            help_open: false,
            restyle: false,
            rescale: false,
            fonts_chinese,
            url_input: String::new(),
            status: None,
            config_broken: config_error.is_some(),
            last_toast: String::new(),
            config_toast_until: None,
            queued_toast: None,
            last_activity: Instant::now(),
            fullscreen: false,
            hwdec_current: String::new(),
            audio_tracks: Vec::new(),
            selftest: launch.selftest.map(selftest::State::new),
            asr_setup: crate::setup_ui::AsrSetup::new(),
            tts_setup: crate::tts_ui::TtsSetup::new(),
            search_ui: crate::search_ui::SearchUi::new(),
            dub: None,
            volume: cfg.player.volume,
            quiet,
            dub_save,
            key_log: Vec::new(),
            cache: Default::default(),
            buffering_shown: false,
            dub_wait: Default::default(),
            dub_wait_shown: false,
            stage_rect: egui::Rect::NOTHING,
            menu_at: None,
            menu_request: None,
            menu_rects: Vec::new(),
            on_top: false,
            info_open: false,
            info_view: None,
            window_shot: None,
            frame_requests: 0,
            frame_done: None,
            toast_reveal: None,
            cfg,
        };
        if app.cfg.tts.enabled && !app.quiet && app.dub_ready() {
            app.set_dub(true);
        }
        if let Some(e) = config_error {
            let e = tf(Tr::ConfigUsingDefaults, &[&e]);
            eprintln!("{e}");
            app.toast_for(e, Duration::from_secs(10));
            app.config_toast_until = Some(Instant::now() + Duration::from_secs(10));
        }
        if let Some(media) = launch.media {
            app.open(&media);
        }
        if !app.cfg.general.shell_integration {
            app.sync_shell_switch();
        }
        Ok(app)
    }

    /// The settings start from the config, except that the Explorer switch shows what the
    /// registry says: the installer registers the entries without touching the config.
    pub fn new_settings_draft(&mut self) -> Config {
        self.sync_shell_switch();
        self.cfg.clone()
    }

    fn sync_shell_switch(&mut self) {
        if shell::SUPPORTED
            && self.selftest.is_none()
            && let Ok(state) = shell::state()
        {
            self.cfg.general.shell_integration = state == shell::State::Here;
        }
    }

    pub fn toast(&mut self, text: impl Into<String>) {
        self.toast_for(text, Duration::from_secs(5));
    }

    pub fn toast_for(&mut self, text: impl Into<String>, shown_for: Duration) {
        let text = text.into();
        if self.config_toast_until.is_some_and(|t| Instant::now() < t) {
            self.queued_toast = Some((text, shown_for));
            return;
        }
        self.last_toast.clone_from(&text);
        self.toast_reveal = None;
        self.status = Some((text, Instant::now(), shown_for));
    }

    pub fn open(&mut self, media: &str) {
        self.open_with(media, None, None);
    }

    /// Opens media under a title of its own, with subtitle files that came with a stream.
    pub fn open_with(
        &mut self,
        media: &str,
        title: Option<String>,
        stream_subs: Option<Vec<crate::session::StreamSub>>,
    ) {
        let is_subtitle = media
            .rsplit('.')
            .next()
            .and_then(gp_core::SubFormat::from_extension)
            .is_some();
        if is_subtitle && self.session.is_some() {
            self.add_external_subtitle(media.to_owned(), None, true);
            return;
        }
        let stream = self.torrserver_stream(media);
        let needs_ytdlp = media.starts_with("http")
            && stream.is_none()
            && self.cfg.player.ytdl_path.is_empty()
            && !crate::ytdlp::binary_path().is_file();
        if needs_ytdlp {
            self.pending_url = Some(media.to_owned());
            if !crate::ytdlp::is_busy() {
                self.jobs.spawn(|| Msg::YtDlp(crate::ytdlp::ensure(true)));
            }
            self.toast(t(Tr::DownloadingYtdlp));
            return;
        }
        // `canonicalize` would add a `\?` prefix on Windows that mpv and ffmpeg do not accept.
        let media = if media.contains("://") {
            media.to_owned()
        } else {
            std::path::absolute(media)
                .map_or_else(|_| media.to_owned(), |p| p.to_string_lossy().into_owned())
        };
        let media = media.as_str();
        self.generation += 1;
        self.tokens.clear();
        self.popup = None;
        self.scrolled_to = None;
        let mut session = Session::new(self.generation, media.to_owned());
        if let Some((hash, index)) = stream {
            // The server address and file name in the URL may change; the torrent file stays.
            session.media_key = gp_core::media_key::torrent_key(&hash, index);
            session.title = stream_title(media);
            session.title_fixed = true;
        }
        if let Some(title) = title.filter(|t| !t.trim().is_empty()) {
            session.title = title;
            session.title_fixed = true;
        }
        session.stream_subs = stream_subs.unwrap_or_default();
        self.session = Some(session);
        self.cache
            .follow(&self.player.mpv, crate::buffering::is_network(media));
        crate::context_menu::reset_picture(&self.player.mpv);
        self.info_view = None;
        self.player.mpv.set("sid", "no");
        self.player.mpv.set_flag("sub-visibility", false);
        if let Err(e) = self.player.load(media) {
            self.toast(e.to_string());
        }
        self.player.set_paused(false);
    }

    fn add_external_subtitle(&mut self, path: String, lang: Option<String>, make_primary: bool) {
        let Some(s) = &mut self.session else { return };
        let name = std::path::Path::new(&path)
            .file_name()
            .map_or(path.clone(), |n| n.to_string_lossy().into_owned());
        let index = s.tracks.len();
        s.tracks.push(SubTrack::new(
            name,
            lang,
            Origin::External { path: path.clone() },
            Load::Loading,
        ));
        if make_primary {
            s.primary = Some(index);
        }
        self.jobs.load_external(s.generation, index, path);
    }

    /// Called once mpv has opened the file and knows its tracks.
    fn discover_tracks(&mut self) {
        let all = self.player.tracks();
        self.audio_tracks = all
            .iter()
            .filter(|t| t.kind == TrackKind::Audio)
            .cloned()
            .collect();
        self.hwdec_current = self
            .player
            .mpv
            .get_string("hwdec-current")
            .unwrap_or_default();
        let Some(s) = &mut self.session else { return };
        if s.tracks_discovered {
            return;
        }
        s.tracks_discovered = true;
        if let Some(title) = self
            .player
            .mpv
            .get_string("media-title")
            .filter(|_| s.is_url() && !s.title_fixed)
        {
            s.title = title;
        }
        for t in all.iter().filter(|t| t.kind == TrackKind::Sub) {
            let lang = t.lang.as_deref().map(normalize_lang);
            let index = s.tracks.len();
            let (origin, load) = match (&t.external_filename, t.ff_index) {
                _ if t.is_bitmap_sub() => (
                    Origin::Embedded {
                        mpv_id: t.id,
                        stream: 0,
                    },
                    Load::Bitmap,
                ),
                (Some(file), _) => (
                    Origin::External {
                        path: crate::session::subtitle_url(file),
                    },
                    Load::NotLoaded,
                ),
                (None, Some(stream)) => (
                    Origin::Embedded {
                        mpv_id: t.id,
                        stream,
                    },
                    Load::NotLoaded,
                ),
                (None, None) => continue,
            };
            let _ = index;
            s.tracks.push(SubTrack::new(t.label(), lang, origin, load));
        }
        // Only a link yt-dlp opened has this; it is gone once the file ends.
        let online = self
            .player
            .mpv
            .get_string(crate::online::HOOK_RESULT)
            .and_then(|json| crate::online::OnlineInfo::from_hook_result(&json));
        if let Some(info) = &online {
            s.media_key = info.media_key();
            let (learn, native) = (&self.cfg.general.learn_lang, &self.cfg.general.native_lang);
            for sub in info.auto_subs(learn, native) {
                let mut track = SubTrack::new(
                    auto_label(&sub),
                    Some(sub.lang.clone()),
                    Origin::External { path: sub.url },
                    Load::NotLoaded,
                );
                track.auto = sub.kind;
                s.tracks.push(track);
            }
        }
        s.online = online;
        for sub in std::mem::take(&mut s.stream_subs) {
            let index = s.tracks.len();
            s.tracks.push(SubTrack::new(
                sub.name,
                sub.lang,
                Origin::External {
                    path: sub.url.clone(),
                },
                Load::Loading,
            ));
            self.jobs.load_external(s.generation, index, sub.url);
        }
        if !s.is_url() {
            for (path, lang) in sidecar_subtitles(&s.media) {
                let index = s.tracks.len();
                let name = std::path::Path::new(&path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                s.tracks.push(SubTrack::new(
                    name,
                    lang,
                    Origin::External { path: path.clone() },
                    Load::Loading,
                ));
                self.jobs.load_external(s.generation, index, path);
            }
        }
        self.choose_default_tracks();
        if let Some((primary, secondary)) = self.session.as_ref().map(|s| (s.primary, s.secondary))
        {
            if let Some(i) = primary {
                self.ensure_loaded(i);
            }
            if let Secondary::Track(i) = secondary {
                self.ensure_loaded(i);
            }
        }
        let no_text = self
            .session
            .as_ref()
            .is_some_and(|s| !s.tracks.iter().any(SubTrack::is_text));
        if no_text && self.cfg.asr.enabled {
            self.start_asr();
        }
    }

    fn choose_default_tracks(&mut self) {
        let Some(s) = &mut self.session else { return };
        let (primary, secondary) = crate::session::default_tracks(
            &s.tracks,
            &self.cfg.general.learn_lang,
            &self.cfg.general.native_lang,
            self.cfg.subtitles.auto_translate,
            self.translator.is_ok(),
        );
        s.primary = primary;
        s.secondary = secondary;
        // Image subtitles are the only kind mpv still draws itself.
        let bitmap = s.tracks.iter().find_map(|t| match (&t.origin, &t.load) {
            (Origin::Embedded { mpv_id, .. }, Load::Bitmap) if primary.is_none() => Some(*mpv_id),
            _ => None,
        });
        self.mpv_draws(bitmap);
    }

    /// mpv renders subtitles itself only for tracks I cannot show as text: image tracks and
    /// tracks that failed to load. `None` turns its renderer off.
    pub fn mpv_draws(&self, track: Option<i64>) {
        match track {
            Some(id) => {
                self.player.mpv.set("sid", &id.to_string());
                self.player.mpv.set_flag("sub-visibility", true);
            }
            None => self.player.mpv.set_flag("sub-visibility", false),
        }
    }

    /// Reads a track the first time it is needed. An online video can offer dozens of
    /// languages, and every embedded track costs a pass over the file.
    pub fn ensure_loaded(&mut self, index: usize) {
        let Some(s) = &mut self.session else { return };
        let Some(t) = s
            .tracks
            .get_mut(index)
            .filter(|t| t.load == Load::NotLoaded)
        else {
            return;
        };
        if matches!(t.origin, Origin::Embedded { .. }) && s.media.contains("://") {
            // ffmpeg would have to download the whole file before giving the first line, and
            // for a torrent stream make the server fetch all of it through a small cache.
            t.live = true;
            t.set_cues(Vec::new());
            self.set_live_slots();
            return;
        }
        t.load = Load::Loading;
        match &t.origin {
            Origin::External { path } => self.jobs.load_external(s.generation, index, path.clone()),
            Origin::Embedded { stream, .. } => self.jobs.load_embedded(
                s.generation,
                index,
                self.ffmpeg.clone(),
                s.media.clone(),
                *stream,
            ),
            Origin::Asr => {}
        }
    }

    pub fn set_secondary(&mut self, choice: Secondary) {
        if let Some(s) = &mut self.session {
            s.secondary = choice;
        }
        if let Secondary::Track(i) = choice {
            self.ensure_loaded(i);
        }
        self.set_live_slots();
    }

    /// The embedded tracks of a stream that are shown go to mpv's two subtitle slots, so it
    /// decodes them as it plays; its own rendering of them stays off.
    pub fn set_live_slots(&self) {
        let Some(s) = &self.session else { return };
        let live_id = |i: Option<usize>| match s.tracks.get(i?) {
            Some(SubTrack {
                live: true,
                origin: Origin::Embedded { mpv_id, .. },
                ..
            }) => Some(*mpv_id),
            _ => None,
        };
        let mpv = &self.player.mpv;
        if let Some(id) = live_id(s.primary) {
            mpv.set("sid", &id.to_string());
            mpv.set_flag("sub-visibility", false);
        }
        let secondary = match s.secondary {
            Secondary::Track(i) => live_id(Some(i)),
            _ => None,
        };
        mpv.set_flag("secondary-sub-visibility", false);
        mpv.set(
            "secondary-sid",
            &secondary.map_or("no".to_owned(), |id| id.to_string()),
        );
    }

    /// Takes the lines mpv is showing in its subtitle slots into their live tracks.
    fn read_live_subs(&mut self) {
        let Some(s) = &mut self.session else { return };
        let secondary = match s.secondary {
            Secondary::Track(i) => Some(i),
            _ => None,
        };
        let mpv = &self.player.mpv;
        for (slot, current, index) in [
            ("sub", "current-tracks/sub/id", s.primary),
            ("secondary-sub", "current-tracks/sub2/id", secondary),
        ] {
            let Some(t) = index.and_then(|i| s.tracks.get_mut(i)).filter(|t| t.live) else {
                continue;
            };
            // Right after a switch the slot may still hold the line of the track before.
            let Origin::Embedded { mpv_id, .. } = t.origin else {
                continue;
            };
            if mpv.get_i64(current) != Some(mpv_id) {
                continue;
            }
            let cue = crate::session::live_cue(
                mpv.get_string(&format!("{slot}-text")).as_deref(),
                mpv.get_f64(&format!("{slot}-start")),
                mpv.get_f64(&format!("{slot}-end")),
            );
            if cue.is_some_and(|c| t.absorb(c)) {
                let track = index.unwrap_or_default();
                self.tokens.retain(|(t, _), _| *t != track);
            }
        }
    }

    pub fn asr_configured(&self) -> Result<(PathBuf, PathBuf), String> {
        crate::asr_setup::resolve_paths(&self.cfg.asr, crate::asr_setup::installed().as_ref())
    }

    pub fn start_asr(&mut self) {
        let (server, model) = match self.asr_configured() {
            Ok(paths) => paths,
            Err(e) => return self.toast(e),
        };
        let Some(ffmpeg) = self.ffmpeg.clone() else {
            return self.toast(t(Tr::FfmpegRequired));
        };
        let duration = self.snap.duration.max(self.player.snapshot().duration);
        let ytdl = self.ytdl_binary();
        let Some(s) = &mut self.session else { return };
        // ffmpeg reads a window of sound from a link mpv opened itself (a file on a server, a
        // TorrServer stream) by seeking in it, as from a file. A page yt-dlp opened is read
        // from its separate sound stream, if the site has one.
        let online = s.online.as_ref().and_then(|o| o.audio.clone());
        if s.online.is_some() && online.is_none() {
            return self.toast(t(Tr::StreamSoundUnreadable));
        }
        if duration <= 0 {
            return self.toast(t(Tr::NoDurationYet));
        }
        let index = match s.tracks.iter().position(|t| t.origin == Origin::Asr) {
            Some(i) => i,
            None => {
                let lang = crate::online::OnlineInfo::speech_lang(
                    s.online.as_ref(),
                    &self.cfg.general.learn_lang,
                );
                s.tracks.push(SubTrack::new(
                    t(Tr::AsrTrackLabel).into(),
                    lang,
                    Origin::Asr,
                    Load::Loading,
                ));
                s.tracks.len() - 1
            }
        };
        s.primary = Some(index);
        if s.secondary == Secondary::Off && self.cfg.subtitles.auto_translate {
            s.secondary = Secondary::Translate;
        }
        self.player.mpv.set_flag("sub-visibility", false);

        let lang =
            crate::online::OnlineInfo::speech_lang(s.online.as_ref(), &self.cfg.general.learn_lang);
        let engine_id = format!(
            "whisper.cpp:{}#{}",
            model.file_stem().unwrap_or_default().to_string_lossy(),
            asr_plan::CACHE_FORMAT
        );
        let cached = self
            .jobs
            .store
            .lock()
            .unwrap()
            .load_transcript(&s.media_key, &engine_id, lang.as_deref().unwrap_or(""))
            .ok()
            .flatten();
        let mut done = vec![false; asr_plan::window_count(duration)];
        let mut heard = asr_plan::Transcript::default();
        if let Some(saved) = cached {
            done = asr_plan::restore_done(duration, saved.covered, &saved.done).0;
            heard = saved.heard;
            heard.retain(|k| done.get(k) == Some(&true));
        }
        s.tracks[index].set_cues(heard.cues());
        s.tracks[index].growing = done.contains(&false).then(|| done.clone());

        // Silero VAD keeps Whisper from inventing text over silence; I use it when the model
        // file sits next to the Whisper model.
        let vad_model = model.parent().and_then(|dir| {
            std::fs::read_dir(dir)
                .ok()?
                .flatten()
                .map(|e| e.path())
                .find(|p| {
                    p.file_name()
                        .is_some_and(|n| n.to_string_lossy().starts_with("ggml-silero"))
                })
        });
        // The sound of an online video is a stream of its own, with nothing else in it.
        let audio_index = self
            .audio_tracks
            .iter()
            .find(|t| t.selected)
            .and_then(|t| t.ff_index)
            .filter(|_| online.is_none());
        let installed =
            crate::asr_setup::installed().filter(|i| i.server.as_ref() == Some(&server));
        let engine = installed.as_ref().and_then(|i| i.engine);
        let threads = (engine == Some(crate::asr_setup::Engine::Cpu))
            .then(|| crate::asr_setup::cpu_threads(crate::asr_setup::physical_cores()));
        let first_run_note = installed
            .filter(|i| i.engine == Some(crate::asr_setup::Engine::Nvidia) && !i.warmed_up)
            .map(|_| format!(" — {}", t(Tr::FirstRun)));
        s.asr_running = true;
        s.asr = Some(
            self.jobs.start_asr(AsrParams {
                generation: s.generation,
                media: online
                    .as_ref()
                    .map_or_else(|| s.media.clone(), |a| a.url.clone()),
                input: online
                    .as_ref()
                    .map(|a| a.ffmpeg_input())
                    .unwrap_or_default(),
                refresh: online.as_ref().map(|_| {
                    let page = s.media.clone();
                    let refresh: crate::jobs::Refresh = Arc::new(move || {
                        crate::online::fresh_audio(&ytdl, &page).map(|a| {
                            let input = a.ffmpeg_input();
                            (a.url, input)
                        })
                    });
                    refresh
                }),
                media_key: s.media_key.clone(),
                audio_index,
                duration,
                lang,
                ffmpeg,
                server,
                model,
                vad_model,
                threads,
                first_run_note,
                done,
                heard,
            }),
        );
    }

    pub fn stop_asr(&mut self) {
        if let Some(s) = &mut self.session {
            s.asr = None;
            s.asr_running = false;
            s.asr_status = String::new();
        }
    }

    fn handle_messages(&mut self) {
        while let Ok(msg) = self.jobs.rx.try_recv() {
            let Some(msg) = self.on_search_msg(msg) else {
                continue;
            };
            let current = self.session.as_ref().map(|s| s.generation);
            // Also for a file closed meanwhile, or the sidebar would never ask again.
            if let Msg::Translated {
                for_panel: true, ..
            } = msg
            {
                self.panel_busy = false;
            }
            match msg {
                Msg::TrackLoaded {
                    generation,
                    track,
                    result,
                } if Some(generation) == current => {
                    let s = self.session.as_mut().unwrap();
                    match result {
                        Ok(cues) => s.tracks[track].set_cues(cues),
                        Err(e) => {
                            let sub = &mut s.tracks[track];
                            sub.load = Load::Failed(e.clone());
                            let mpv_id = match sub.origin {
                                Origin::Embedded { mpv_id, .. } if s.primary == Some(track) => {
                                    Some(mpv_id)
                                }
                                _ => None,
                            };
                            let label = sub.label.clone();
                            let embedded = matches!(sub.origin, Origin::Embedded { .. });
                            let no_ffmpeg = self.ffmpeg.is_none();
                            let text = match (mpv_id, no_ffmpeg) {
                                (Some(_), true) => Some(t(Tr::NoFfmpegToast).to_owned()),
                                (Some(_), false) => {
                                    Some(tf(Tr::CouldNotReadSubsShown, &[&label, &e]))
                                }
                                // Same cause as the message for the main track; one is enough.
                                (None, true) if embedded => None,
                                (None, _) => Some(tf(Tr::CouldNotLoadSubs, &[&label, &e])),
                            };
                            if mpv_id.is_some() {
                                self.mpv_draws(mpv_id);
                            }
                            if let Some(text) = text {
                                self.toast(text);
                            }
                        }
                    }
                    self.tokens.retain(|(t, _), _| *t != track);
                }
                Msg::Translated {
                    generation,
                    track,
                    epoch,
                    sentences,
                    result,
                    ..
                } if Some(generation) == current => {
                    let s = self.session.as_mut().unwrap();
                    let t = &mut s.tracks[track];
                    t.apply_translations(&sentences, result.as_deref().ok(), epoch);
                    if let Err(error) = result {
                        s.translate_failed_at = Some(Instant::now());
                        self.toast(tf(Tr::TranslationFailed, &[&error]));
                    }
                }
                Msg::AsrCues {
                    generation,
                    cues,
                    window,
                    language,
                } if Some(generation) == current => {
                    let s = self.session.as_mut().unwrap();
                    if let Some(t) = s.tracks.iter_mut().find(|t| t.origin == Origin::Asr) {
                        if t.track.lang.is_none() {
                            t.track.lang = language;
                        }
                        if let Some(done) = t.growing.as_mut().and_then(|d| d.get_mut(window)) {
                            *done = true;
                        }
                        t.update_cues(cues);
                    }
                    self.tokens.clear();
                }
                Msg::AsrStatus {
                    generation,
                    text,
                    running,
                    alert,
                } if Some(generation) == current => {
                    let s = self.session.as_mut().unwrap();
                    s.asr_status = text.clone();
                    s.asr_running = running;
                    if !running {
                        // Finished or failed: no more cues will come to continue a sentence.
                        for t in s.tracks.iter_mut().filter(|t| t.origin == Origin::Asr) {
                            t.growing = None;
                        }
                    }
                    if alert {
                        self.toast(text);
                    }
                }
                Msg::WordTranslated { request, result } => {
                    if let Some(p) = self.popup.as_mut().filter(|p| p.request == request) {
                        p.translation = result.map_or_else(Async::Failed, Async::Ready);
                    }
                }
                Msg::WordExplained { request, result } => {
                    if let Some(p) = self.popup.as_mut().filter(|p| p.request == request) {
                        p.explanation = match result {
                            Ok(text) => Async::Ready(text),
                            // Plain translators cannot explain; that is not worth an error message.
                            Err(e) if e.contains("does not support") => Async::Idle,
                            Err(e) => Async::Failed(e),
                        };
                    }
                }
                Msg::YtDlp(result) => {
                    match result {
                        Ok(crate::ytdlp::Outcome::Updated { version }) => {
                            self.toast(tf(Tr::YtdlpUpdated, &[&version]));
                        }
                        Ok(_) => {}
                        // A failed start-up check stays quiet; only a link waiting for yt-dlp needs to hear.
                        Err(e) if self.pending_url.take().is_some() => self.toast(e),
                        Err(_) => {}
                    }
                    let ready = crate::ytdlp::binary_path().is_file();
                    if let Some(url) = self.pending_url.take().filter(|_| ready) {
                        self.open(&url);
                    }
                }
                Msg::Anki { vocab_id, result } => match result {
                    Ok(note_id) => {
                        let _ = self
                            .jobs
                            .store
                            .lock()
                            .unwrap()
                            .set_anki_id(vocab_id, note_id);
                        self.reload_vocab();
                        self.toast(t(Tr::AddedToAnki));
                    }
                    Err(e) => self.toast(tf(Tr::AnkiError, &[&e])),
                },
                Msg::CustomDict { vocab_id, result } => match result {
                    Ok(()) => {
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(0);
                        if let Err(e) = self
                            .jobs
                            .store
                            .lock()
                            .unwrap()
                            .set_custom_dict_synced(vocab_id, now)
                        {
                            eprintln!("glossplay: failed to update custom dict sync status: {e}");
                        }
                        self.reload_vocab();
                        self.toast(t(Tr::CustomDictAdded));
                    }
                    Err(code) => {
                        let text = if let Some(msg) = code.strip_prefix("network: ") {
                            tf(Tr::CustomDictErrorNetwork, &[&msg])
                        } else if code == "auth_401" {
                            t(Tr::CustomDictErrorAuth).to_owned()
                        } else if let Some(status) = code.strip_prefix("status_") {
                            tf(Tr::CustomDictErrorServer, &[&status])
                        } else {
                            tf(Tr::CustomDictError, &[&code])
                        };
                        self.toast(text);
                    }
                },
                Msg::Component { id, event } if id == crate::tts_setup::ID => {
                    self.on_tts_event(event)
                }
                Msg::Component { id, event } => self.on_component_event(&id, event),
                Msg::FrameSaved { request, result } => self.on_frame_saved(request, result),
                Msg::FrameCopied(result) => self.on_frame_copied(result),
                _ => {}
            }
        }
    }

    /// Asks for translations a little ahead of the playhead, so they are ready on time, and
    /// when nothing there has to go now, for the lines shown in the sidebar.
    fn request_translations(&mut self) {
        let Some(s) = &mut self.session else { return };
        if s.secondary != Secondary::Translate {
            return;
        }
        let translator = match &self.translator {
            Ok(t) => t.clone(),
            Err(e) => {
                let has_primary = s
                    .primary
                    .is_some_and(|i| s.tracks.get(i).is_some_and(|t| t.load == Load::Ready));
                if has_primary && !s.translator_warned {
                    s.translator_warned = true;
                    let text = tf(Tr::TranslationNotSetUp, &[&e]);
                    self.toast(text);
                }
                return;
            }
        };
        if s.translate_failed_at
            .is_some_and(|t| t.elapsed() < Duration::from_secs(30))
        {
            return;
        }
        let Some(index) = s.primary.filter(|&i| s.tracks[i].load == Load::Ready) else {
            return;
        };
        let generation = s.generation;
        let t = &mut s.tracks[index];
        let lookahead = self.cfg.translation.lookahead.max(1);
        let (wanted, for_panel) = match ahead_of_playhead(t, self.snap.pos, lookahead) {
            Some(wanted) => (wanted, false),
            // One job for the sidebar at a time: the translator works through jobs in order,
            // and the lines under the video must not wait behind a queue of them.
            None if !self.panel_busy => match in_view(t, &self.panel_cues, lookahead) {
                Some(wanted) => (wanted, true),
                None => return,
            },
            None => return,
        };
        let context = (wanted[0].saturating_sub(CONTEXT_SENTENCES)..wanted[0])
            .map(|g| t.sentence_text(g))
            .collect();
        let sentences = t.request_translations(&wanted);
        let src = t
            .track
            .lang
            .clone()
            .or(Some(self.cfg.general.learn_lang.clone()))
            .filter(|l| !l.is_empty());
        self.jobs.translate(TranslateJob {
            generation,
            track: index,
            epoch: t.epoch,
            sentences,
            context,
            src,
            dst: self.cfg.general.native_lang.clone(),
            translator,
            for_panel,
        });
        self.translate_jobs += 1;
        self.panel_busy |= for_panel;
    }

    pub fn open_word(&mut self, word: String, cue: usize, token: usize, anchor: egui::Pos2) {
        let Some(s) = &self.session else { return };
        let Some(t) = s.primary_track() else { return };
        let c = &t.track.cues[cue];
        self.next_request += 1;
        let was_playing = !self.snap.paused;
        let (translation, explanation) = match &self.translator {
            Ok(tr) => {
                let src = t
                    .track
                    .lang
                    .clone()
                    .or(Some(self.cfg.general.learn_lang.clone()))
                    .filter(|l| !l.is_empty());
                self.jobs.lookup_word(
                    self.next_request,
                    tr.clone(),
                    word.clone(),
                    c.text.replace('\n', " "),
                    src,
                    self.cfg.general.native_lang.clone(),
                );
                (Async::Pending, Async::Pending)
            }
            Err(e) => (Async::Failed(e.clone()), Async::Idle),
        };
        let saved_id = self
            .vocab
            .iter()
            .find(|v| v.word.word.to_lowercase() == word.to_lowercase())
            .map(|v| v.id);
        self.popup = Some(WordPopup {
            request: self.next_request,
            sentence: c.text.replace('\n', " "),
            sentence_translation: s.translation_of(cue).unwrap_or_default(),
            cue: (c.start, c.end),
            tokens: vec![(cue, token)],
            word,
            anchor,
            translation,
            explanation,
            saved_id,
            resume_on_close: was_playing,
        });
        self.player.set_paused(true);
    }

    /// Toggles a word token in the active word popup when Ctrl-clicked, or opens a new popup.
    pub fn toggle_word_token(&mut self, cue: usize, token: usize, anchor: egui::Pos2) {
        let Some(s) = &self.session else { return };
        let Some(t) = s.primary_track() else { return };
        let Some(c) = t.track.cues.get(cue) else { return };

        let track_index = s.primary.unwrap_or(0);
        let lang = t.track.lang.clone();
        let cue_text = c.text.clone();
        let tokens_rc = self.tokens_for(track_index, cue, &cue_text, lang.as_deref());

        // Check if popup is currently open on the same cue
        let same_cue = self.popup.as_ref().is_some_and(|p| {
            p.tokens.first().map(|(c_idx, _)| *c_idx) == Some(cue)
        });

        if same_cue {
            let p = self.popup.as_mut().unwrap();
            let key = (cue, token);
            if let Some(pos) = p.tokens.iter().position(|t| *t == key) {
                p.tokens.remove(pos);
            } else {
                p.tokens.push(key);
            }

            if p.tokens.is_empty() {
                self.close_popup();
                return;
            }

            p.tokens.sort_by_key(|(_, t_idx)| *t_idx);
            let words: Vec<&str> = p.tokens
                .iter()
                .filter_map(|(_, idx)| tokens_rc.get(*idx).map(|tok| &cue_text[tok.range.clone()]))
                .collect();
            let new_word = words.join(" ");
            p.word = new_word.clone();
            p.saved_id = self
                .vocab
                .iter()
                .find(|v| v.word.word.to_lowercase() == new_word.to_lowercase())
                .map(|v| v.id);

            let sentence = p.sentence.clone();
            self.next_request += 1;
            let (translation, explanation) = match &self.translator {
                Ok(tr) => {
                    let src = lang
                        .or(Some(self.cfg.general.learn_lang.clone()))
                        .filter(|l| !l.is_empty());
                    self.jobs.lookup_word(
                        self.next_request,
                        tr.clone(),
                        new_word,
                        sentence,
                        src,
                        self.cfg.general.native_lang.clone(),
                    );
                    (Async::Pending, Async::Pending)
                }
                Err(e) => (Async::Failed(e.clone()), Async::Idle),
            };
            p.request = self.next_request;
            p.translation = translation;
            p.explanation = explanation;
        } else {
            let word = tokens_rc
                .get(token)
                .map(|t| cue_text[t.range.clone()].to_string())
                .unwrap_or_default();
            self.open_word(word, cue, token, anchor);
        }
    }

    pub fn close_popup(&mut self) {
        if let Some(p) = self.popup.take()
            && p.resume_on_close
        {
            self.player.set_paused(false);
        }
    }

    pub fn save_popup_word(&mut self) -> Option<i64> {
        let (p, s) = (self.popup.as_ref()?, self.session.as_ref()?);
        let text = |a: &Async| match a {
            Async::Ready(t) => t.clone(),
            _ => String::new(),
        };
        let lang = s
            .primary_track()
            .and_then(|t| t.track.lang.clone())
            .unwrap_or_default();
        let word = NewWord {
            word: p.word.clone(),
            lang,
            translation: text(&p.translation),
            note: text(&p.explanation),
            sentence: p.sentence.clone(),
            sentence_translation: p.sentence_translation.clone(),
            media_title: s.title.clone(),
            media_path: s.media.clone(),
            pos_ms: self.snap.pos,
            cue_start_ms: p.cue.0,
            cue_end_ms: p.cue.1,
        };
        let id = self.jobs.store.lock().unwrap().save_word(&word).ok()?;
        // Taken now, while the word's moment is on screen, so a later export has a picture.
        let _ = self
            .player
            .screenshot(&paths::media_dir().join(format!("glossplay_{id}.jpg")));
        self.reload_vocab();
        if let Some(p) = &mut self.popup {
            p.saved_id = Some(id);
        }
        Some(id)
    }

    pub fn reload_vocab(&mut self) {
        self.vocab = self
            .jobs
            .store
            .lock()
            .unwrap()
            .list_words()
            .unwrap_or_default();
        self.saved_words = self
            .vocab
            .iter()
            .map(|v| v.word.word.to_lowercase())
            .collect();
    }

    pub fn delete_word(&mut self, id: i64) {
        let _ = self.jobs.store.lock().unwrap().delete_word(id);
        self.reload_vocab();
    }

    pub fn export_to_anki(&mut self, vocab_id: i64) {
        let Some(entry) = self.vocab.iter().find(|v| v.id == vocab_id).cloned() else {
            return;
        };
        let dir = paths::media_dir();
        // A screenshot only makes sense while the saved moment is still on screen.
        let same_media = self
            .session
            .as_ref()
            .is_some_and(|s| s.media == entry.word.media_path);
        let on_screen = same_media && (self.snap.pos - entry.word.pos_ms).abs() < 3_000;
        let image = dir.join(format!("glossplay_{vocab_id}.jpg"));
        let image = (image.is_file() || (on_screen && self.player.screenshot(&image).is_ok()))
            .then_some(image);
        let local = !entry.word.media_path.contains("://")
            || self.torrserver_stream(&entry.word.media_path).is_some();
        // The address of an online video's sound is only known while it is open.
        let online = self
            .session
            .as_ref()
            .and_then(|s| s.online.as_ref()?.audio.clone())
            .filter(|_| same_media);
        let stream = self
            .audio_tracks
            .iter()
            .find(|t| t.selected)
            .and_then(|t| t.ff_index)
            .filter(|_| same_media && online.is_none());
        let audio = self
            .ffmpeg
            .clone()
            .filter(|_| local || online.is_some())
            .map(|ffmpeg| crate::jobs::ClipSource {
                ffmpeg,
                media: online
                    .as_ref()
                    .map_or_else(|| entry.word.media_path.clone(), |a| a.url.clone()),
                input: online
                    .as_ref()
                    .map(|a| a.ffmpeg_input())
                    .unwrap_or_default(),
                stream,
                out: dir.join(format!("glossplay_{vocab_id}.mp3")),
            });
        self.toast(t(Tr::SendingToAnki));
        self.jobs.export_anki(
            vocab_id,
            self.cfg.anki.url.clone(),
            self.cfg.anki.deck.clone(),
            entry.word,
            image,
            audio,
        );
    }

    pub fn export_to_custom_dict(&mut self, vocab_id: i64) {
        let Some(entry) = self.vocab.iter().find(|v| v.id == vocab_id).cloned() else {
            return;
        };
        if self.cfg.custom_dict.url.trim().is_empty() {
            self.toast(t(Tr::CustomDictNotConfigured));
            return;
        }
        self.toast(t(Tr::CustomDictSending));
        let token = if self.cfg.custom_dict.token.trim().is_empty() {
            None
        } else {
            Some(self.cfg.custom_dict.token.clone())
        };
        self.jobs.export_custom_dict(
            vocab_id,
            self.cfg.custom_dict.url.clone(),
            token,
            entry.word,
        );
    }

    pub fn apply_settings(&mut self, new: Config) {
        self.rescale |= new.general.ui_scale != self.cfg.general.ui_scale;
        self.restyle |= new.general.theme != self.cfg.general.theme;
        if self.selftest.is_none() {
            i18n::apply_setting(&new.general.ui_lang);
        }
        let shell_changed = new.general.shell_integration != self.cfg.general.shell_integration;
        let (old_tts, old_lang) = (self.cfg.tts.clone(), self.cfg.general.native_lang.clone());
        self.cfg = new;
        crate::components::set_root(&self.cfg.general.components_dir);
        let tts = &self.cfg.tts;
        let voice_changed = (
            &tts.base_url,
            &tts.model,
            &tts.voice,
            &tts.voice_source,
            &self.cfg.general.native_lang,
        ) != (
            &old_tts.base_url,
            &old_tts.model,
            &old_tts.voice,
            &old_tts.voice_source,
            &old_lang,
        );
        if voice_changed || !tts.enabled {
            self.dub = None;
        }
        if self.cfg.tts.enabled && self.dub.is_none() {
            self.set_dub(true);
            self.cfg.tts.enabled = self.dub.is_some();
        }
        self.apply_volume();
        if shell_changed && shell::SUPPORTED && self.selftest.is_none() {
            let result = if self.cfg.general.shell_integration {
                shell::register()
            } else {
                shell::unregister()
            };
            if let Err(e) = result {
                self.toast(tf(Tr::ExplorerIntegration, &[&e]));
            }
        }
        self.translator = make_translator(&self.cfg);
        self.trainer.autopause = self.cfg.training.autopause;
        self.ffmpeg = tools::locate("ffmpeg", &self.cfg.player.ffmpeg_path).ok();
        if let Some(s) = &mut self.session {
            // A new provider or language makes the old translations stale.
            for t in &mut s.tracks {
                t.reset_translations();
            }
            s.translate_failed_at = None;
            s.translator_warned = false;
        }
        if self.config_broken {
            self.config_broken = false;
            match paths::keep_broken_config() {
                Ok(backup) => self.toast_for(
                    tf(Tr::OldConfigKept, &[&backup.display()]),
                    Duration::from_secs(10),
                ),
                Err(e) => self.toast(tf(Tr::CouldNotKeepConfig, &[&e])),
            }
        }
        if let Err(e) = paths::save_config(&mut self.cfg) {
            self.toast(tf(Tr::CouldNotSaveSettings, &[&e]));
        }
    }

    /// The moment the subtitles on screen belong to; see `Trainer::shown_at`.
    pub fn subtitle_pos(&self) -> Ms {
        self.trainer.shown_at(self.snap.pos, self.snap.paused)
    }

    pub fn train(&mut self, action: Option<TrainAction>) {
        match action {
            Some(TrainAction::Pause) => self.player.set_paused(true),
            Some(TrainAction::SeekTo(ms)) => {
                self.player.seek(ms);
                self.player.set_paused(false);
            }
            None => {}
        }
    }

    pub fn cue_nav(&mut self, which: i32) {
        let Some(t) = self.session.as_ref().and_then(Session::primary_track) else {
            return self.player.seek_relative(which as Ms * 5_000);
        };
        let pos = self.snap.pos;
        let action = match which {
            -1 => self.trainer.prev(&t.track, pos),
            0 => self.trainer.replay(&t.track, pos),
            _ => self.trainer.next(&t.track, pos),
        };
        self.train(action);
    }

    pub fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    pub fn open_dialog(&mut self) {
        let picked = rfd::FileDialog::new()
            .set_title(t(Tr::OpenDialogTitle))
            .pick_file();
        if let Some(path) = picked {
            self.open(&path.to_string_lossy());
        }
    }

    /// Per-frame bookkeeping that is not drawing.
    pub fn tick(&mut self, ctx: &egui::Context) {
        self.video.render.report_swap();
        while let Some(event) = self.player.mpv.poll_event() {
            match event {
                Event::FileLoaded => self.discover_tracks(),
                Event::EndFile { error: true } => self.toast(t(Tr::CouldNotPlay)),
                Event::Property { id, value } => {
                    self.cache.on_property(id, &value);
                }
                _ => {}
            }
        }
        self.handle_messages();
        self.read_live_subs();
        self.prev_pos = self.snap.pos;
        self.snap = self.player.snapshot();
        if self.quiet && !self.snap.muted && !crate::record::on() {
            // Nothing in a self-test may be heard; a click on the mute button cannot undo it.
            self.player.mpv.set_flag("mute", true);
        }
        if let Some(s) = &self.session {
            if let Some(asr) = &s.asr {
                asr.playhead.store(self.snap.pos, Ordering::Relaxed);
            }
            if !self.snap.paused {
                let action = s
                    .primary_track()
                    .and_then(|t| self.trainer.tick(&t.track, self.prev_pos, self.snap.pos));
                self.train(action);
            }
        }
        self.request_translations();
        self.update_dub();
        let waiting = self.cache.waiting(self.snap.paused);
        self.buffering_shown = self.cache.shown(waiting, Instant::now());
        let dub_waiting = self.dub.as_ref().is_some_and(|d| d.holding());
        self.dub_wait_shown = self.dub_wait.shown(dub_waiting, Instant::now());
        if waiting || dub_waiting {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
        if !self.snap.paused && !self.snap.idle {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
        if self
            .status
            .as_ref()
            .is_some_and(|(_, at, shown_for)| at.elapsed() > *shown_for)
        {
            self.status = None;
        }
        if self.status.is_none()
            && self.config_toast_until.is_none_or(|t| Instant::now() >= t)
            && let Some((text, shown_for)) = self.queued_toast.take()
        {
            self.toast_for(text, shown_for);
        }
    }

    pub fn tokens_for(
        &mut self,
        track: usize,
        cue: usize,
        text: &str,
        lang: Option<&str>,
    ) -> Rc<Vec<Token>> {
        self.tokens
            .entry((track, cue))
            .or_insert_with(|| Rc::new(gp_core::tokenize(text, lang)))
            .clone()
    }
}

/// Sentence groups to translate now in the window ahead of the playhead, widened to whole
/// sentences; `None` when nothing there has to go yet.
fn ahead_of_playhead(t: &SubTrack, pos: Ms, lookahead: usize) -> Option<Vec<usize>> {
    let from = t
        .track
        .index_at_or_before(pos)
        .or_else(|| t.track.index_after(pos))?;
    let first = t.group_of(from);
    let (mut wanted, mut wanted_cues, mut covered) = (Vec::new(), 0, 0);
    for g in first..t.groups.len() {
        if covered >= lookahead {
            break;
        }
        covered += t.groups[g].len();
        if t.ready_to_translate(g, pos) {
            wanted.push(g);
            wanted_cues += t.groups[g].len();
        }
    }
    // Waiting until half of the window is missing turns many one-line requests into a few batches.
    let urgent = wanted
        .first()
        .is_some_and(|&g| t.track.cues[t.groups[g].start].start <= pos + 5_000);
    (!wanted.is_empty() && (urgent || wanted_cues >= lookahead / 2)).then_some(wanted)
}

/// Sentence groups of the cues shown in the sidebar that still lack a translation, at most
/// about `limit` cues of them.
fn in_view(t: &SubTrack, cues: &[usize], limit: usize) -> Option<Vec<usize>> {
    let (mut wanted, mut count) = (Vec::new(), 0);
    let mut last = None;
    // The list is from the last frame; speech recognition may have changed the track since.
    for &i in cues.iter().filter(|&&i| i < t.track.cues.len()) {
        let g = t.group_of(i);
        if last == Some(g) {
            continue;
        }
        last = Some(g);
        if t.needs_translation(g) && !t.may_grow(g) {
            wanted.push(g);
            count += t.groups[g].len();
            if count >= limit {
                break;
            }
        }
    }
    (!wanted.is_empty()).then_some(wanted)
}

impl App {
    /// The yt-dlp mpv uses: the one in the settings, or the copy I keep up to date.
    fn ytdl_binary(&self) -> PathBuf {
        if self.cfg.player.ytdl_path.is_empty() {
            crate::ytdlp::binary_path()
        } else {
            PathBuf::from(&self.cfg.player.ytdl_path)
        }
    }
}

/// "English (auto)" in the interface language, or the site's own name of a language I have
/// no name for.
fn auto_label(sub: &crate::online::AutoSub) -> String {
    let name = i18n::language_name(&sub.lang).map_or_else(|| sub.name.clone(), |k| t(k).to_owned());
    let template = match sub.kind {
        crate::online::Auto::Translated => Tr::AutoTranslatedSubsLabel,
        _ => Tr::AutoSubsLabel,
    };
    tf(template, &[&name])
}

/// The file name a TorrServer stream URL carries, without its extension.
fn stream_title(url: &str) -> String {
    let path = url.split('?').next().unwrap_or(url);
    let name = path.rsplit('/').next().unwrap_or(path);
    let name = percent_decode(name);
    match name.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => stem.to_owned(),
        _ => name,
    }
}

pub fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = bytes
            .get(i + 1..i + 3)
            .and_then(|h| std::str::from_utf8(h).ok())
            .and_then(|h| u8::from_str_radix(h, 16).ok());
        match (bytes[i], hex) {
            (b'%', Some(v)) => {
                out.push(v);
                i += 3;
            }
            (b, _) => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn make_translator(cfg: &Config) -> Result<Arc<dyn Translator>, String> {
    build_translator(&cfg.translation)
        .map(Arc::from)
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use gp_core::Cue;

    /// One sentence every 3 s.
    fn track(n: usize) -> SubTrack {
        let mut t = SubTrack::new("en".into(), Some("en".into()), Origin::Asr, Load::Loading);
        t.set_cues(
            (0..n)
                .map(|i| Cue {
                    start: i as Ms * 3_000,
                    end: i as Ms * 3_000 + 2_000,
                    text: format!("Line {i}."),
                })
                .collect(),
        );
        t
    }

    #[test]
    fn a_stream_is_titled_by_its_file_name() {
        assert_eq!(
            stream_title(
                "http://ts.test:8090/stream/Sintel%20%282010%29.mp4?link=abc&index=6&play"
            ),
            "Sintel (2010)"
        );
        assert_eq!(
            stream_title("http://ts.test/stream/video?link=abc"),
            "video"
        );
    }

    #[test]
    fn the_sidebar_asks_for_what_it_shows_after_the_playhead_window() {
        let mut t = track(60);
        let ahead = ahead_of_playhead(&t, 90_000, 10).unwrap();
        assert_eq!(ahead, (30..40).collect::<Vec<_>>());
        t.request_translations(&ahead);
        assert_eq!(ahead_of_playhead(&t, 90_000, 10), None);

        // Rows above the playhead, the way a sidebar scrolled back shows them.
        let shown: Vec<usize> = (5..20).collect();
        let wanted = in_view(&t, &shown, 10).unwrap();
        assert_eq!(
            wanted,
            (5..15).collect::<Vec<_>>(),
            "no more than the limit"
        );
        t.request_translations(&wanted);
        assert_eq!(
            in_view(&t, &shown, 10).unwrap(),
            (15..20).collect::<Vec<_>>()
        );
        t.request_translations(&(15..20).collect::<Vec<_>>());
        assert_eq!(in_view(&t, &shown, 10), None, "nothing asked twice");
        // The list is a frame old and the track may have shrunk meanwhile.
        assert_eq!(in_view(&t, &[58, 59, 60, 61], 10), Some(vec![58, 59]));
        assert_eq!(in_view(&t, &[], 10), None, "the sidebar is closed");
    }

    #[test]
    fn word_popup_multi_token_toggle_and_reset_ac6_ac7() {
        let mut popup = WordPopup {
            request: 1,
            word: "take".into(),
            tokens: vec![(0, 1)],
            sentence: "Please take care of yourself".into(),
            sentence_translation: "".into(),
            cue: (0, 2000),
            anchor: egui::pos2(10.0, 20.0),
            translation: Async::Idle,
            explanation: Async::Idle,
            saved_id: None,
            resume_on_close: false,
        };
        assert!(popup.contains_token(0, 1));
        assert!(!popup.contains_token(0, 2));

        // Add second token out of order (e.g. index 3 then 2)
        popup.tokens.push((0, 3)); // "of"
        popup.tokens.push((0, 2)); // "care"
        popup.tokens.sort_by_key(|(_, idx)| *idx);
        assert_eq!(popup.tokens, vec![(0, 1), (0, 2), (0, 3)]);
        assert!(popup.contains_token(0, 2));
        assert!(popup.contains_token(0, 3));

        // Toggle removal of (0, 2)
        if let Some(pos) = popup.tokens.iter().position(|t| *t == (0, 2)) {
            popup.tokens.remove(pos);
        }
        assert_eq!(popup.tokens, vec![(0, 1), (0, 3)]);
        assert!(!popup.contains_token(0, 2));
    }
}
