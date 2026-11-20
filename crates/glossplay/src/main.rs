#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod anki_ui;
mod app;
mod asr_setup;
mod buffering;
mod components;
mod context_menu;
mod control;
mod custom_dict_ui;
mod dub;
mod frame;
mod help_ui;
mod i18n;
mod icon;
mod info;
mod jobs;
mod keys;
mod online;
mod paths;
mod player;
mod record;
mod search_ui;
mod selftest;
mod session;
mod settings_ui;
mod setup_text;
mod setup_ui;
mod shell;
mod substyle;
mod theme;
mod tts_setup;
mod tts_ui;
mod ui;
mod video;
mod ytdlp;

use crate::i18n::{Tr, t};
use eframe::egui;
use std::sync::Mutex;
use std::sync::atomic::Ordering;

const HELP: &str = "\
Glossplay — a video player for language learners

Usage: glossplay [FILE or URL] [options]

  --translate-file FILE --to LANG   translate a subtitle file twice and print how many
                                    requests each pass made (the second should make none)
  --selftest                        play FILE without interaction, print a report, exit
      --seconds N                   how long to play (default 6)
      --at SECONDS                  start position
      --subs A,B                    languages of the primary and secondary track;
                                    B = mt shows the machine translation instead
      --autopause                   check auto-pause and repeat on the next line
      --asr                         run speech recognition
      --anki                        save the opened word and send it to Anki
      --strict                      also fail on software decoding or many dropped frames
      --export-srt FILE.srt         write the primary subtitle track to a file, and the
                                    translation shown under it to FILE.tr.srt
      --screenshot FILE.png         save the window as an image at the end
      --controls --sidebar --word N --paused   what to show in the screenshot
      --theme dark|light|system     interface theme for this run
      --sub-style NAME --sub-scale N --sub-position bottom|top --sub-margin N
                                    subtitle look for this run (style: plate, outline, bold,
                                    yellow, light; scale 0.6 to 2.0)
      --empty                       (with --selftest) no file: the start screen; with
                                    --settings the first tab of the settings window
      --words-tab                   show the saved words in the sidebar
      --ui-lang auto|en|ru|uk|zh    interface language (a self-test is in English unless

                                    told otherwise)
      --setup-dialog --settings     open the speech recognition setup or the settings
      --setup-install               install speech recognition from the setup dialog (as
                                    chosen in config.toml) and transcribe the file after it
  --install-asr                     download and install speech recognition, then exit
      --asr-variant auto|nvidia|cpu   engine build (default auto: NVIDIA if present)
      --asr-model NAME              large-v3-turbo, large-v3-turbo-q5_0, small or base
                                    (default: the one that suits the engine)
  --asr-status                      show what is installed and what this computer would get
      --dub                         (with --selftest) speak the translation over the video,
                                    pause once in the middle, turn it off at the end
      --dub-save DIR                copy every voice-over clip played into DIR
      --tts-dialog --settings-voice --subs-menu   what to show in the screenshot
      --context-menu                (with --selftest) the menu of a right click on the picture
      --context-submenu NAME        the same with a submenu open: speed, audio, subtitles,
                                    video or frame
      --context-click NAME          and click its item once: play, rotate, fill, 16:9, 4:3,
                                    save_frame, save_frame_subtitles, copy_time, video_info
      --video-info                  (with --selftest) the Video info window
      --search-dialog QUERY         (with --selftest) search your sources and show the results
      --settings-sources            (with --selftest) Settings at Sources, after pressing Check
      --settings-tab ID             (with --selftest) Settings at a tab: languages, subtitles,
                                    translation, speech_recognition, voice_over, words,
                                    sources or player
      --help-window                 (with --selftest) the How to use window
      --open-picker learn-lang|native-lang   (with --settings-tab languages) open that list
      --keys LIST                   (with --selftest) press keys as a layout sends them, e.g.
                                    ru:s=repeat_line,focus:url,ru:ctrl+v; prints what each did
      --clipboard TEXT              what Ctrl+V pastes in --keys (the real clipboard is not used)
  --search QUERY                    search the server set in [sources], print the first results
  --stream-url LINK                 add a magnet link or info hash to TorrServer and print its
                                    files and stream URLs (the torrent stays on the server)
  --install-tts                     download and install the voice-over engine, then exit
      --tts-variant vulkan|nvidia   engine build (default vulkan: any graphics card)
  --tts-status                      show what is installed for the voice-over
  --shell-register                  add Glossplay to Explorer's \"Open with\" and context menu
                                    (current user only; it does not become the default program)
  --shell-unregister                remove those entries
  --update-ytdlp                 download or update yt-dlp and print its version
  --size WxH                        window size
  --mcp [--quiet]                   take commands from an AI agent over MCP on stdin and stdout
                                    (docs/mcp.md); --quiet keeps both players silent
  --version, --help";

fn main() {
    let all: Vec<String> = std::env::args().collect();
    if all.get(1).map(String::as_str) == Some("--watchdog") {
        let arg = |i: usize| all.get(i).map_or("", String::as_str);
        std::process::exit(gp_media::tools::watchdog(arg(2), arg(3)));
    }
    if let Ok(exe) = std::env::current_exe() {
        gp_media::tools::enable_watchdog(exe);
    }
    let mut args = std::env::args().skip(1);
    let mut media = None;
    let mut plan = selftest::Plan {
        seconds: 6.0,
        ..Default::default()
    };
    let mut is_selftest = false;
    let mut size = [1280.0, 720.0];
    let (mut translate_file, mut to_lang) = (None, None);
    let (mut install_asr, mut asr_variant, mut asr_model) = (false, None, None);
    let (mut install_tts, mut tts_variant) = (false, None);
    let mut ui_lang: Option<String> = None;
    while let Some(arg) = args.next() {
        let mut value = || args.next().unwrap_or_default();
        match arg.as_str() {
            "--help" | "-h" => return println!("{HELP}"),
            "--version" | "-V" => return println!("glossplay {}", env!("CARGO_PKG_VERSION")),
            "--selftest" => is_selftest = true,
            "--seconds" => plan.seconds = value().parse().unwrap_or(6.0),
            "--at" => plan.at = value().parse().ok(),
            "--subs" => plan.subs = value().split(',').map(str::to_owned).collect(),
            "--screenshot" => plan.screenshot = Some(value().into()),
            "--empty" => plan.empty = true,
            "--words-tab" => plan.words_tab = true,
            "--ui-lang" => ui_lang = Some(value()),
            "--controls" => plan.show_controls = true,
            "--sidebar" => plan.sidebar = true,
            "--paused" => plan.paused = true,
            "--word" => plan.word = value().parse().ok(),
            "--autopause" => plan.autopause = true,
            "--asr" => plan.asr = true,
            "--anki" => plan.anki = true,
            "--strict" => plan.strict = true,
            "--export-srt" => plan.export_srt = Some(value().into()),
            "--shell-register" => std::process::exit(shell::cli(true)),
            "--shell-unregister" => std::process::exit(shell::cli(false)),
            "--update-ytdlp" => {
                return match ytdlp::ensure(false) {
                    Ok(outcome) => println!("{outcome}\nversion={}", outcome.version()),
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
            }
            "--setup-dialog" => plan.setup_dialog = true,
            "--settings" => plan.settings = true,
            "--setup-install" => plan.setup_install = true,
            "--install-asr" => install_asr = true,
            "--asr-variant" => asr_variant = Some(value()),
            "--asr-model" => asr_model = Some(value()),
            "--asr-status" => std::process::exit(asr_setup::cli_status()),
            "--dub" => plan.dub = true,
            "--dub-save" => plan.dub_save = Some(value().into()),
            "--tts-dialog" => plan.tts_dialog = true,
            "--settings-voice" => plan.settings_voice = true,
            "--settings-tab" => {
                plan.settings_tab = Some(value());
                plan.settings = true;
            }
            "--help-window" => plan.help = true,
            "--open-picker" => plan.open_picker = Some(value()),
            "--subs-menu" => plan.subs_menu = true,
            "--context-menu" => plan.context_menu = true,
            "--context-submenu" => {
                plan.context_menu = true;
                plan.context_submenu = Some(value());
            }
            "--context-click" => {
                plan.context_menu = true;
                plan.context_click = Some(value());
            }
            "--video-info" => plan.video_info = true,
            "--sub-style" => plan.sub_style = Some(value()),
            "--sub-scale" => plan.sub_scale = value().parse().ok(),
            "--sub-position" => plan.sub_position = Some(value()),
            "--sub-margin" => plan.sub_margin = value().parse().ok(),
            "--theme" => plan.theme = Some(value()),
            "--keys" => plan.keys = value().split(',').map(str::to_owned).collect(),
            "--clipboard" => plan.clipboard = value(),
            "--search-dialog" => {
                plan.search_dialog = Some(value());
                plan.empty = true;
            }
            "--settings-sources" => {
                plan.settings_sources = true;
                plan.settings = true;
                plan.empty = true;
            }
            "--search" => std::process::exit(search_ui::cli_search(&value())),
            "--stream-url" => std::process::exit(search_ui::cli_stream_url(&value())),
            "--install-tts" => install_tts = true,
            "--tts-variant" => tts_variant = Some(value()),
            "--tts-status" => std::process::exit(tts_setup::cli_status()),
            "--translate-file" => translate_file = Some(value()),
            "--to" => to_lang = Some(value()),
            "--mcp" | "--quiet" => {}
            "--size" => {
                let v = value();
                if let Some((w, h)) = v.split_once('x') {
                    size = [w.parse().unwrap_or(1280.0), h.parse().unwrap_or(720.0)];
                }
            }
            _ => media = Some(arg),
        }
    }
    if install_tts {
        std::process::exit(tts_setup::cli_install(tts_variant.as_deref().unwrap_or("")));
    }
    if install_asr {
        std::process::exit(asr_setup::cli_install(
            asr_variant.as_deref().unwrap_or("auto"),
            asr_model.as_deref().unwrap_or(""),
        ));
    }
    if let Some(file) = translate_file {
        std::process::exit(translate_file_twice(&file, to_lang.as_deref()));
    }

    if !plan.keys.is_empty() && media.is_none() {
        plan.empty = true;
    }
    control::launch(&mut media);
    if is_selftest {
        // Checks compare texts, so a self-test speaks English unless a language is asked for.
        let wanted = ui_lang.as_deref().unwrap_or("en");
        i18n::set_lang(i18n::resolve(wanted, i18n::system_lang()));
    }
    let launch = app::Launch {
        media,
        selftest: is_selftest.then_some(plan),
    };
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Glossplay")
        .with_inner_size(size)
        .with_min_inner_size([640.0, 360.0])
        .with_drag_and_drop(true);
    if let Some(icon) = icon::window_icon() {
        viewport = viewport.with_icon(icon);
    }
    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    let is_selftest = launch.selftest.is_some();
    let launch = Mutex::new(Some(launch));
    let result = eframe::run_native(
        "Glossplay",
        options,
        Box::new(move |cc| {
            let launch = launch
                .lock()
                .unwrap()
                .take()
                .expect("the app is created once");
            match app::App::new(cc, launch) {
                Ok(app) => Ok(Box::new(app) as Box<dyn eframe::App>),
                Err(message) => {
                    eprintln!("{message}");
                    if is_selftest {
                        // Nobody is there to read a window; report on stderr and stop.
                        std::process::exit(2);
                    }
                    selftest::EXIT_CODE.store(2, Ordering::SeqCst);
                    Ok(Box::new(StartupError { message }))
                }
            }
        }),
    );
    control::finish();
    // `on_exit` has stopped the server already when the window closed normally; this also
    // covers leaving any other way, since `process::exit` runs no destructors.
    gp_media::tools::stop_all();
    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(2);
    }
    std::process::exit(selftest::EXIT_CODE.load(Ordering::SeqCst));
}

/// Shown instead of the player when it cannot start, most often because libmpv is missing.
struct StartupError {
    message: String,
}

impl eframe::App for StartupError {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.heading(t(Tr::StartupFailed));
                ui.add_space(12.0);
                ui.label(&self.message);
                ui.add_space(12.0);
                ui.label(t(Tr::StartupMpvHint));
                ui.monospace(paths::config_path().to_string_lossy());
            });
        });
    }
}

fn translate_file_twice(file: &str, to: Option<&str>) -> i32 {
    let (cfg, config_error) = paths::load_config();
    if let Some(e) = config_error {
        eprintln!("{e}; using the default settings");
    }
    let run = || -> Result<(), String> {
        let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
        let hint = file
            .rsplit('.')
            .next()
            .and_then(gp_core::SubFormat::from_extension);
        let cues = gp_core::parse_subtitles(&bytes, hint).map_err(|e| e.to_string())?;
        let lines: Vec<String> = cues.iter().map(|c| c.text.clone()).collect();
        let translator =
            gp_ai::translate::build_translator(&cfg.translation).map_err(|e| e.to_string())?;
        let store =
            Mutex::new(gp_store::Store::open(&paths::db_path()).map_err(|e| e.to_string())?);
        let dst = to.unwrap_or(&cfg.general.native_lang);
        println!("provider={}", translator.id());
        println!("lines={}", lines.len());
        for pass in 1..=2 {
            let out = jobs::translate_cached(&store, translator.as_ref(), None, dst, &lines, &[])
                .map_err(|e| e.to_string())?;
            let filled = out.results.iter().filter(|r| !r.trim().is_empty()).count();
            println!("pass{pass}_requests={}", out.requests);
            println!("pass{pass}_translated={filled}");
            if pass == 1 {
                for (src, dst) in lines.iter().zip(&out.results).take(3) {
                    println!("sample={} => {}", src.replace('\n', " "), dst);
                }
            }
        }
        Ok(())
    };
    match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}
