//! Background work. The UI thread never touches the network or the disk: it sends a job,
//! keeps drawing, and picks up a `Msg` when the job is done.

use crate::i18n::{Tr, t, tf};
use crate::session::SentenceKey;
use eframe::egui;
use gp_ai::AiError;
use gp_ai::anki::{AnkiClient, AnkiNote};
use gp_ai::asr::{AsrEngine, AsrOutput, WhisperServer};
use gp_ai::translate::{TranslateRequest, Translator, translate_resilient};
use gp_core::asr_plan;
use gp_core::{Cue, Ms};
use gp_media::tools::{self, WhisperProcess};
use gp_store::Store;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub enum Msg {
    TrackLoaded {
        generation: u64,
        track: usize,
        result: Result<Vec<Cue>, String>,
    },
    Translated {
        generation: u64,
        track: usize,
        epoch: u64,
        sentences: Vec<SentenceKey>,
        result: Result<Vec<String>, String>,
        for_panel: bool,
    },
    AsrCues {
        generation: u64,
        /// The whole transcript so far: a new window may also change cues next to it.
        cues: Vec<Cue>,
        window: usize,
        language: Option<String>,
    },
    AsrStatus {
        generation: u64,
        text: String,
        running: bool,
        /// Recognition has stopped for good on an error the user should see.
        alert: bool,
    },
    WordTranslated {
        request: u64,
        result: Result<String, String>,
    },
    WordExplained {
        request: u64,
        result: Result<String, String>,
    },
    YtDlp(Result<crate::ytdlp::Outcome, String>),
    Anki {
        vocab_id: i64,
        result: Result<i64, String>,
    },
    CustomDict {
        vocab_id: i64,
        result: Result<(), String>,
    },
    Component {
        id: String,
        event: crate::components::Event,
    },
    SearchDone {
        request: u64,
        result: Result<Vec<gp_ai::torznab::Release>, String>,
    },
    FilesListed {
        request: u64,
        result: Result<gp_ai::torrserver::Torrent, String>,
    },
    /// The result of the Check button: one line per server.
    SourcesChecked(Vec<Result<String, String>>),
    FrameSaved {
        request: u64,
        result: Result<std::path::PathBuf, String>,
    },
    /// A frame read for the clipboard: width, height and RGBA rows from the top.
    FrameCopied(Result<(u32, u32, Vec<u8>), String>),
}

pub struct TranslateJob {
    pub generation: u64,
    pub track: usize,
    pub epoch: u64,
    pub sentences: Vec<SentenceKey>,
    /// Sentences of the original just before the first one asked, for LLM providers.
    pub context: Vec<String>,
    pub src: Option<String>,
    pub dst: String,
    pub translator: Arc<dyn Translator>,
    /// Asked for the transcript in the sidebar rather than for the lines under the video.
    pub for_panel: bool,
}

pub struct TranslateOutcome {
    pub results: Vec<String>,
    /// How many times the provider was actually called; zero means everything came from cache.
    pub requests: usize,
}

const RETRY_DELAYS: [u64; 3] = [1, 4, 15];
pub const CONTEXT_SENTENCES: usize = 8;

/// Translates `lines`, asking the provider only for what the cache does not have.
pub fn translate_cached(
    store: &Mutex<Store>,
    translator: &dyn Translator,
    src: Option<&str>,
    dst: &str,
    lines: &[String],
    context: &[String],
) -> Result<TranslateOutcome, AiError> {
    let (provider, src_key) = (translator.id(), src.unwrap_or(""));
    let cached = store
        .lock()
        .unwrap()
        .get_translations(&provider, src_key, dst, lines)
        .unwrap_or_else(|_| vec![None; lines.len()]);
    let missing: Vec<usize> = (0..lines.len()).filter(|&i| cached[i].is_none()).collect();
    let mut results: Vec<String> = cached.into_iter().map(Option::unwrap_or_default).collect();
    let mut requests = 0;
    for batch in missing.chunks(translator.batch_size().max(1)) {
        let batch_lines: Vec<String> = batch.iter().map(|&i| lines[i].clone()).collect();
        // Sentences just before the batch give an LLM the context of the dialogue.
        let first = batch[0];
        let mut ctx: Vec<String> = context.to_vec();
        ctx.extend(lines[..first].iter().cloned());
        let ctx = ctx.split_off(ctx.len().saturating_sub(CONTEXT_SENTENCES));
        let req = TranslateRequest {
            src_lang: src,
            dst_lang: dst,
            lines: &batch_lines,
            context_before: &ctx,
        };
        let mut attempt = 0;
        let translated = loop {
            requests += 1;
            match translate_resilient(translator, &req) {
                Err(e) if e.is_transient() && attempt < RETRY_DELAYS.len() => {
                    std::thread::sleep(Duration::from_secs(RETRY_DELAYS[attempt]));
                    attempt += 1;
                }
                other => break other?,
            }
        };
        let pairs: Vec<(String, String)> = batch_lines
            .into_iter()
            .zip(translated.iter().cloned())
            .collect();
        let _ = store
            .lock()
            .unwrap()
            .put_translations(&provider, src_key, dst, &pairs);
        for (&i, text) in batch.iter().zip(translated) {
            results[i] = text;
        }
    }
    Ok(TranslateOutcome { results, requests })
}

/// A new address and ffmpeg input options for media whose address has expired.
pub type Refresh = Arc<dyn Fn() -> Result<(String, Vec<String>), String> + Send + Sync>;

/// Where an Anki card's sound clip is cut from.
pub struct ClipSource {
    pub ffmpeg: PathBuf,
    /// The file, or the direct address of an online video's sound.
    pub media: String,
    pub input: Vec<String>,
    pub stream: Option<u32>,
    pub out: PathBuf,
}

pub struct AsrParams {
    pub generation: u64,
    /// The file, or the direct address of an online video's sound.
    pub media: String,
    /// ffmpeg options before `-i` that `media` needs.
    pub input: Vec<String>,
    pub refresh: Option<Refresh>,
    pub media_key: String,
    pub audio_index: Option<u32>,
    pub duration: Ms,
    pub lang: Option<String>,
    pub ffmpeg: PathBuf,
    pub server: PathBuf,
    pub model: PathBuf,
    pub vad_model: Option<PathBuf>,
    /// CPU threads for whisper-server; `None` keeps its default.
    pub threads: Option<usize>,
    /// Added to the progress text until the first window is done.
    pub first_run_note: Option<String>,
    /// Windows already transcribed in an earlier run.
    pub done: Vec<bool>,
    pub heard: asr_plan::Transcript,
}

/// Cancels the worker when dropped, so switching files never leaves two of them running.
pub struct AsrHandle {
    pub playhead: Arc<AtomicI64>,
    cancel: Arc<AtomicBool>,
}

impl Drop for AsrHandle {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

pub struct Jobs {
    tx: Sender<Msg>,
    pub rx: Receiver<Msg>,
    ctx: egui::Context,
    pub store: Arc<Mutex<Store>>,
    translate_tx: Sender<TranslateJob>,
    whisper: Arc<Mutex<Option<WhisperProcess>>>,
    /// Set by `shutdown`; an ASR thread still running must not start a new server after it.
    closing: Arc<AtomicBool>,
}

impl Jobs {
    pub fn new(ctx: egui::Context, store: Store) -> Jobs {
        let (tx, rx) = channel();
        let store = Arc::new(Mutex::new(store));
        let (translate_tx, translate_rx) = channel::<TranslateJob>();
        let (worker_tx, worker_ctx, worker_store) = (tx.clone(), ctx.clone(), store.clone());
        // One translation at a time keeps me inside provider rate limits.
        std::thread::spawn(move || {
            for job in translate_rx {
                let lines: Vec<String> = job.sentences.iter().map(|s| s.text.clone()).collect();
                let result = translate_cached(
                    &worker_store,
                    job.translator.as_ref(),
                    job.src.as_deref(),
                    &job.dst,
                    &lines,
                    &job.context,
                )
                .map(|o| o.results)
                .map_err(|e| e.to_string());
                let _ = worker_tx.send(Msg::Translated {
                    generation: job.generation,
                    track: job.track,
                    epoch: job.epoch,
                    sentences: job.sentences,
                    result,
                    for_panel: job.for_panel,
                });
                worker_ctx.request_repaint();
            }
        });
        Jobs {
            tx,
            rx,
            ctx,
            store,
            translate_tx,
            whisper: Arc::default(),
            closing: Arc::default(),
        }
    }

    /// Stops whisper-server now. `main` leaves through `process::exit`, which runs no
    /// destructors, and the ASR thread holds its own reference to the slot anyway.
    pub fn shutdown(&self) {
        self.closing.store(true, Ordering::SeqCst);
        let process = self
            .whisper
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        drop(process);
    }

    pub fn context(&self) -> egui::Context {
        self.ctx.clone()
    }

    pub fn spawn(&self, work: impl FnOnce() -> Msg + Send + 'static) {
        let (tx, ctx) = (self.tx.clone(), self.ctx.clone());
        std::thread::spawn(move || {
            let _ = tx.send(work());
            ctx.request_repaint();
        });
    }

    /// Like `spawn`, for work that reports progress on the way through `send`.
    pub fn spawn_reporting(&self, work: impl FnOnce(&dyn Fn(Msg)) -> Msg + Send + 'static) {
        let (tx, ctx) = (self.tx.clone(), self.ctx.clone());
        std::thread::spawn(move || {
            let send = |msg: Msg| {
                let _ = tx.send(msg);
                ctx.request_repaint();
            };
            let last = work(&send);
            send(last);
        });
    }

    pub fn translate(&self, job: TranslateJob) {
        let _ = self.translate_tx.send(job);
    }

    pub fn load_embedded(
        &self,
        generation: u64,
        track: usize,
        ffmpeg: Option<PathBuf>,
        media: String,
        stream: u32,
    ) {
        self.spawn(move || {
            let result = ffmpeg
                .ok_or_else(|| t(Tr::FfmpegNotFound).to_owned())
                .and_then(|f| {
                    tools::extract_subtitles(&f, &media, stream).map_err(|e| e.to_string())
                })
                .and_then(|bytes| {
                    gp_core::parse_subtitles(&bytes, Some(gp_core::SubFormat::Srt))
                        .map_err(|e| e.to_string())
                });
            Msg::TrackLoaded {
                generation,
                track,
                result,
            }
        });
    }

    pub fn load_external(&self, generation: u64, track: usize, path: String) {
        self.spawn(move || {
            let hint = path
                .split('?')
                .next()
                .unwrap_or(&path)
                .rsplit('.')
                .next()
                .and_then(gp_core::SubFormat::from_extension);
            let bytes = if path.starts_with("http://") || path.starts_with("https://") {
                gp_ai::fetch(&path).map_err(|e| e.to_string())
            } else {
                std::fs::read(&path).map_err(|e| e.to_string())
            };
            let result =
                bytes.and_then(|b| gp_core::parse_subtitles(&b, hint).map_err(|e| e.to_string()));
            Msg::TrackLoaded {
                generation,
                track,
                result,
            }
        });
    }

    pub fn lookup_word(
        &self,
        request: u64,
        translator: Arc<dyn Translator>,
        word: String,
        sentence: String,
        src: Option<String>,
        dst: String,
    ) {
        let (store, t) = (self.store.clone(), translator.clone());
        let (w, d) = (word.clone(), dst.clone());
        self.spawn(move || {
            let result = translate_cached(&store, t.as_ref(), src.as_deref(), &d, &[w], &[])
                .map(|mut o| o.results.remove(0))
                .map_err(|e| e.to_string());
            Msg::WordTranslated { request, result }
        });
        self.spawn(move || {
            let result = translator
                .explain(&word, &sentence, &dst)
                .map_err(|e| e.to_string());
            Msg::WordExplained { request, result }
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn export_anki(
        &self,
        vocab_id: i64,
        url: String,
        deck: String,
        word: gp_store::NewWord,
        image: Option<PathBuf>,
        audio_job: Option<ClipSource>,
    ) {
        self.spawn(move || {
            // The clip is optional: a card without audio is better than no card.
            let audio = audio_job.and_then(|c| {
                let len = (word.cue_end_ms - word.cue_start_ms).clamp(500, 20_000);
                tools::extract_clip(
                    &c.ffmpeg,
                    &c.media,
                    &c.input,
                    c.stream,
                    word.cue_start_ms,
                    len,
                    &c.out,
                )
                .ok()
                .map(|_| c.out)
            });
            let anki = AnkiClient::new(&url);
            let source = format!(
                "{} — {}",
                word.media_title,
                crate::theme::clock(word.pos_ms)
            );
            let result = anki
                .ensure_setup(&deck)
                .and_then(|_| {
                    anki.add_note(&AnkiNote {
                        deck: &deck,
                        word: &word.word,
                        translation: &word.translation,
                        sentence: &word.sentence,
                        sentence_translation: &word.sentence_translation,
                        note: &word.note,
                        source: &source,
                        image: image.as_deref().filter(|p| p.is_file()),
                        audio: audio.as_deref(),
                    })
                })
                .map_err(|e| match e {
                    AiError::Network(_) => t(Tr::AnkiNotRunning).to_owned(),
                    AiError::Duplicate => t(Tr::AlreadyInDeck).to_owned(),
                    other => other.to_string(),
                });
            Msg::Anki { vocab_id, result }
        });
    }

    pub fn export_custom_dict(
        &self,
        vocab_id: i64,
        url: String,
        token: Option<String>,
        word: gp_store::NewWord,
    ) {
        self.spawn(move || {
            let client = gp_ai::custom_dict::CustomDictClient::new(&url, token.as_deref());
            let context = if word.sentence.trim().is_empty() {
                None
            } else {
                Some(word.sentence.as_str())
            };
            let translation = if word.translation.trim().is_empty() {
                None
            } else {
                Some(word.translation.as_str())
            };
            let result = client
                .send_word(&word.word, context, translation)
                .map_err(|e| match e {
                    AiError::Network(ref msg) => format!("network: {msg}"),
                    AiError::Http { status: 401, .. } => "auth_401".to_string(),
                    AiError::Http { status, .. } => format!("status_{status}"),
                    other => other.to_string(),
                });
            Msg::CustomDict { vocab_id, result }
        });
    }

    pub fn start_asr(&self, params: AsrParams) -> AsrHandle {
        let handle = AsrHandle {
            playhead: Arc::new(AtomicI64::new(0)),
            cancel: Arc::default(),
        };
        let (playhead, cancel) = (handle.playhead.clone(), handle.cancel.clone());
        let (tx, ctx, store, whisper, closing) = (
            self.tx.clone(),
            self.ctx.clone(),
            self.store.clone(),
            self.whisper.clone(),
            self.closing.clone(),
        );
        std::thread::spawn(move || {
            let generation = params.generation;
            let send = |text: String, running: bool, alert: bool| {
                let _ = tx.send(Msg::AsrStatus {
                    generation,
                    text,
                    running,
                    alert,
                });
                ctx.request_repaint();
            };
            let status = |text: String, running: bool| send(text, running, false);
            let asr = Asr {
                cancel: &cancel,
                closing: &closing,
                whisper: &whisper,
                status: &status,
            };
            if let Err(e) = run_asr(params, &asr, &playhead, &store, &tx, &ctx) {
                send(tf(Tr::AsrStopped, &[&e]), false, true);
            }
        });
        handle
    }
}

/// A window that took the server down this many times in a row is taken for empty.
const WINDOW_CRASHES: usize = 2;
/// Windows skipped that way, one after another with no good answer between them, after which
/// the server itself is blamed.
const KILLER_WINDOWS: usize = 3;

#[derive(Debug, PartialEq)]
enum Verdict {
    Retry,
    /// Treat the window as done and empty; the server is fine otherwise.
    Skip,
    GiveUp(usize),
}

/// Tells a window that kills whisper-server from a server that does not work at all.
#[derive(Default)]
struct CrashLog {
    window: Option<usize>,
    repeats: usize,
    skipped: usize,
}

impl CrashLog {
    fn crashed(&mut self, window: usize) -> Verdict {
        self.repeats = if self.window == Some(window) {
            self.repeats + 1
        } else {
            1
        };
        self.window = Some(window);
        if self.repeats < WINDOW_CRASHES {
            return Verdict::Retry;
        }
        self.window = None;
        self.repeats = 0;
        self.skipped += 1;
        if self.skipped >= KILLER_WINDOWS {
            Verdict::GiveUp(self.skipped * WINDOW_CRASHES)
        } else {
            Verdict::Skip
        }
    }

    fn answered(&mut self) {
        *self = CrashLog::default();
    }
}

/// What a transcription shares with the rest of the program.
struct Asr<'a> {
    cancel: &'a AtomicBool,
    closing: &'a AtomicBool,
    whisper: &'a Mutex<Option<WhisperProcess>>,
    status: &'a dyn Fn(String, bool),
}

impl Asr<'_> {
    /// Asked to stop: another file was opened or the player is closing.
    fn stopped(&self) -> bool {
        self.cancel.load(Ordering::Relaxed) || self.closing.load(Ordering::SeqCst)
    }

    /// Starts whisper-server unless it is running, and waits until it answers. `None` when
    /// asked to stop meanwhile.
    fn server(&self, p: &AsrParams) -> Result<Option<WhisperServer>, String> {
        let model_name = p
            .model
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        let url = {
            let mut slot = self.whisper.lock().unwrap_or_else(|e| e.into_inner());
            // Checked under the lock: `shutdown` sets the flag before it empties the slot.
            if self.closing.load(Ordering::SeqCst) {
                return Ok(None);
            }
            if slot.as_mut().is_none_or(|w| w.has_exited()) {
                (self.status)(t(Tr::AsrStarting).into(), true);
                let process = WhisperProcess::spawn_with_threads(
                    &p.server,
                    &p.model,
                    p.vad_model.as_deref(),
                    p.threads,
                )
                .map_err(|e| tf(Tr::CannotStartWhisper, &[&e]))?;
                *slot = Some(process);
            }
            slot.as_ref().unwrap().url.clone()
        };
        let engine = WhisperServer::new(&url, &model_name);
        let started = Instant::now();
        while !engine.is_ready() {
            if self.stopped() {
                return Ok(None);
            }
            if self.died() {
                return Err(t(Tr::WhisperQuitLoading).into());
            }
            if started.elapsed() > Duration::from_secs(120) {
                return Err(t(Tr::WhisperNotReady).into());
            }
            std::thread::sleep(Duration::from_millis(150));
        }
        Ok(Some(engine))
    }

    /// Whether the server process is gone. A connection error can come a moment before the
    /// process is reported dead, so I give it a second.
    fn died(&self) -> bool {
        let gone = || {
            let mut slot = self.whisper.lock().unwrap_or_else(|e| e.into_inner());
            slot.as_mut().is_none_or(|w| w.has_exited())
        };
        let since = Instant::now();
        loop {
            if gone() {
                return true;
            }
            if since.elapsed() > Duration::from_secs(1) {
                return false;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn run_asr(
    mut p: AsrParams,
    asr: &Asr,
    playhead: &AtomicI64,
    store: &Mutex<Store>,
    tx: &Sender<Msg>,
    ctx: &egui::Context,
) -> Result<(), String> {
    if p.done.iter().all(|d| *d) {
        (asr.status)(t(Tr::TranscriptReady).into(), false);
        return Ok(());
    }
    let Some(mut engine) = asr.server(&p)? else {
        return Ok(());
    };

    let total = p.done.len();
    let lang_key = p.lang.clone().unwrap_or_default();
    let cache_engine = format!("{}#{}", engine.id(), asr_plan::CACHE_FORMAT);
    let save = |p: &AsrParams| {
        let covered = asr_plan::covered_prefix(&p.done, p.duration);
        let _ = store
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .save_transcript(
                &p.media_key,
                &cache_engine,
                &lang_key,
                covered,
                &p.done,
                &p.heard,
            );
    };
    let mut saved = Instant::now();
    let (mut crashes, mut retry) = (CrashLog::default(), None);
    while let Some(k) = retry
        .take()
        .or_else(|| asr_plan::next_window(&p.done, playhead.load(Ordering::Relaxed)))
    {
        if asr.stopped() {
            save(&p);
            return Ok(());
        }
        let finished = p.done.iter().filter(|d| **d).count();
        (asr.status)(
            tf(
                Tr::Transcribing,
                &[
                    &(finished * 100 / total.max(1)),
                    &p.first_run_note.as_deref().unwrap_or(""),
                ],
            ),
            true,
        );
        let (start, len) = asr_plan::audio_range(k);
        let read = |p: &AsrParams| {
            tools::extract_pcm(&p.ffmpeg, &p.media, &p.input, p.audio_index, start, len)
        };
        let pcm = match (read(&p), p.refresh.clone()) {
            (Ok(pcm), _) => pcm,
            // An online video's address expires after some hours; a new one is asked for.
            (Err(e), Some(refresh)) => {
                let (media, input) =
                    refresh().map_err(|r| tf(Tr::CouldNotReadAudio, &[&format!("{e} ({r})")]))?;
                (p.media, p.input) = (media, input);
                read(&p).map_err(|e| tf(Tr::CouldNotReadAudio, &[&e]))?
            }
            (Err(e), None) => return Err(tf(Tr::CouldNotReadAudio, &[&e])),
        };
        let mut language = None;
        let words = if pcm.is_empty() {
            Vec::new()
        } else {
            let out = match engine.transcribe(&pcm, p.lang.as_deref()) {
                Ok(out) => {
                    crashes.answered();
                    out
                }
                // The server was killed under me on purpose; that is not an error to show.
                Err(_) if asr.stopped() => {
                    save(&p);
                    return Ok(());
                }
                Err(e) => {
                    save(&p);
                    if !asr.died() {
                        return Err(match e {
                            AiError::Network(_) => t(Tr::WhisperStopped).into(),
                            other => tf(Tr::WhisperFailed, &[&other]),
                        });
                    }
                    let verdict = crashes.crashed(k);
                    if let Verdict::GiveUp(n) = verdict {
                        return Err(tf(Tr::WhisperQuitTimes, &[&n]));
                    }
                    let Some(fresh) = asr.server(&p)? else {
                        return Ok(());
                    };
                    engine = fresh;
                    if verdict == Verdict::Retry {
                        retry = Some(k);
                        continue;
                    }
                    AsrOutput::default()
                }
            };
            language = out.language;
            let sound = asr_plan::Loudness::new(&pcm, gp_ai::asr::SAMPLE_RATE);
            asr_plan::words_in(start, &out.segments, Some(&sound))
        };
        if p.first_run_note.take().is_some() {
            crate::asr_setup::mark_warmed_up();
        }
        p.done[k] = true;
        p.heard.insert(k, words);
        let _ = tx.send(Msg::AsrCues {
            generation: p.generation,
            cues: p.heard.cues(),
            window: k,
            language,
        });
        ctx.request_repaint();

        // The player may be closed at any moment, but the whole transcript is rewritten on
        // every save, which for a film adds up; once every couple of seconds is enough.
        if saved.elapsed() >= Duration::from_secs(2) {
            save(&p);
            saved = Instant::now();
        }
    }
    save(&p);
    (asr.status)(t(Tr::TranscriptReady).into(), false);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn window_that_kills_the_server_twice_is_skipped() {
        let mut log = CrashLog::default();
        assert_eq!(log.crashed(4), Verdict::Retry);
        assert_eq!(log.crashed(4), Verdict::Skip);
        // The counters start over, so the next window gets its own two tries.
        assert_eq!(log.crashed(5), Verdict::Retry);
        assert_eq!(log.crashed(5), Verdict::Skip);
    }

    #[test]
    fn a_good_answer_clears_the_crash_record() {
        let mut log = CrashLog::default();
        for w in [1, 1, 2, 2] {
            log.crashed(w);
        }
        log.answered();
        assert_eq!(log.crashed(3), Verdict::Retry);
        assert_eq!(log.crashed(3), Verdict::Skip);
        assert_eq!(log.crashed(4), Verdict::Retry);
        assert_eq!(log.crashed(4), Verdict::Skip);
    }

    #[test]
    fn several_different_windows_in_a_row_blame_the_server() {
        let mut log = CrashLog::default();
        let verdicts: Vec<_> = [1, 1, 2, 2, 3, 3].iter().map(|w| log.crashed(*w)).collect();
        assert_eq!(
            verdicts,
            vec![
                Verdict::Retry,
                Verdict::Skip,
                Verdict::Retry,
                Verdict::Skip,
                Verdict::Retry,
                Verdict::GiveUp(6)
            ]
        );
    }

    #[test]
    fn a_crash_on_another_window_restarts_the_count() {
        let mut log = CrashLog::default();
        assert_eq!(log.crashed(1), Verdict::Retry);
        assert_eq!(log.crashed(2), Verdict::Retry);
        assert_eq!(log.crashed(1), Verdict::Retry);
    }

    struct Fake {
        calls: AtomicUsize,
        fail_first: AtomicUsize,
    }

    impl Translator for Fake {
        fn id(&self) -> String {
            "fake".into()
        }
        fn batch_size(&self) -> usize {
            20
        }
        fn translate(&self, req: &TranslateRequest) -> Result<Vec<String>, AiError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            if self.fail_first.load(Ordering::SeqCst) > 0 {
                self.fail_first.fetch_sub(1, Ordering::SeqCst);
                return Err(AiError::Http {
                    status: 400,
                    body: "bad".into(),
                });
            }
            Ok(req.lines.iter().map(|l| format!("ru:{l}")).collect())
        }
    }

    /// Remembers every line it was asked to translate.
    #[derive(Default)]
    struct Recorder {
        lines: Mutex<Vec<String>>,
        context: Mutex<Vec<String>>,
    }

    impl Translator for Recorder {
        fn id(&self) -> String {
            "recorder".into()
        }
        fn batch_size(&self) -> usize {
            20
        }
        fn translate(&self, req: &TranslateRequest) -> Result<Vec<String>, AiError> {
            self.lines.lock().unwrap().extend(req.lines.iter().cloned());
            *self.context.lock().unwrap() = req.context_before.to_vec();
            Ok(req
                .lines
                .iter()
                .map(|l| match l.as_str() {
                    "I told him that he didn't do it, she said." => {
                        "Я сказала ему, что он этого не делал, — сказала она.".to_owned()
                    }
                    other => format!("ru:{other}"),
                })
                .collect())
        }
    }

    #[test]
    fn fragments_of_one_sentence_go_out_as_one_line_and_come_back_cut() {
        use crate::session::{Load, Origin, SubTrack};
        let cue = |start: Ms, text: &str| Cue {
            start,
            end: start + 1_000,
            text: text.into(),
        };
        let mut track = SubTrack::new("en".into(), Some("en".into()), Origin::Asr, Load::Loading);
        track.set_cues(vec![
            cue(0, "Hello."),
            cue(1_100, "I told him that"),
            cue(2_200, "he didn't do it,"),
            cue(3_300, "she said."),
        ]);
        let groups: Vec<usize> = (1..track.groups.len()).collect();
        let context: Vec<String> = vec![track.sentence_text(0)];
        let asked = track.request_translations(&groups);
        let lines: Vec<String> = asked.iter().map(|s| s.text.clone()).collect();

        let store = Mutex::new(Store::open_in_memory().unwrap());
        let recorder = Recorder::default();
        let out = translate_cached(&store, &recorder, Some("en"), "ru", &lines, &context).unwrap();
        assert_eq!(
            *recorder.lines.lock().unwrap(),
            ["I told him that he didn't do it, she said."]
        );
        assert_eq!(*recorder.context.lock().unwrap(), ["Hello."]);

        track.apply_translations(&asked, Some(&out.results), track.epoch);
        let whole = "Я сказала ему, что он этого не делал, — сказала она.";
        let parts: Vec<String> = (1..4)
            .map(|i| match &track.translation[i] {
                crate::session::Tr::Done(p) => p.clone(),
                other => panic!("cue {i}: {other:?}"),
            })
            .collect();
        assert!(parts.iter().all(|p| !p.is_empty()), "{parts:?}");
        assert_eq!(parts.join(" "), whole);

        // The sentence is cached as a whole: asking again costs nothing.
        let again = translate_cached(&store, &recorder, Some("en"), "ru", &lines, &[]).unwrap();
        assert_eq!((again.requests, again.results[0].as_str()), (0, whole));
    }

    fn fifty() -> Vec<String> {
        (0..50).map(|i| format!("line {i}")).collect()
    }

    #[test]
    fn second_pass_makes_no_requests() {
        let store = Mutex::new(Store::open_in_memory().unwrap());
        let fake = Fake {
            calls: AtomicUsize::new(0),
            fail_first: AtomicUsize::new(0),
        };
        let lines = fifty();
        let first = translate_cached(&store, &fake, Some("en"), "ru", &lines, &[]).unwrap();
        assert_eq!(first.results.len(), 50);
        assert!(
            first
                .results
                .iter()
                .enumerate()
                .all(|(i, r)| r == &format!("ru:line {i}"))
        );
        assert_eq!(first.requests, 3, "50 lines in batches of 20");
        let second = translate_cached(&store, &fake, Some("en"), "ru", &lines, &[]).unwrap();
        assert_eq!(second.requests, 0);
        assert_eq!(second.results, first.results);
        assert_eq!(fake.calls.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn only_missing_lines_are_requested_and_errors_are_not_cached() {
        let store = Mutex::new(Store::open_in_memory().unwrap());
        let fake = Fake {
            calls: AtomicUsize::new(0),
            fail_first: AtomicUsize::new(1),
        };
        let lines = fifty();
        assert!(translate_cached(&store, &fake, None, "ru", &lines[..10], &[]).is_err());
        let ok = translate_cached(&store, &fake, None, "ru", &lines[..10], &[]).unwrap();
        assert_eq!(ok.requests, 1);
        let more = translate_cached(&store, &fake, None, "ru", &lines[..15], &[]).unwrap();
        assert_eq!(more.requests, 1, "ten cached, five new in one batch");
        assert_eq!(more.results[14], "ru:line 14");
        let other_lang = translate_cached(&store, &fake, None, "de", &lines[..10], &[]).unwrap();
        assert_eq!(other_lang.requests, 1, "cache is per target language");
    }

    #[test]
    fn custom_dict_export_job_e2e_ac7() {
        use std::io::{BufRead, BufReader, Read, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://127.0.0.1:{}/api/words", addr.port());

        let server_thread = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();
            let mut content_length = 0;
            let mut auth_found = false;
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line.trim().is_empty() {
                    break;
                }
                if line.to_lowercase().starts_with("content-length:") {
                    content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap();
                }
                if line.to_lowercase().starts_with("authorization:")
                    && line.contains("Bearer testtoken123")
                {
                    auth_found = true;
                }
            }
            let mut body = vec![0u8; content_length];
            reader.read_exact(&mut body).unwrap();
            let body_str = String::from_utf8(body).unwrap();

            let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();

            (request_line, auth_found, body_str)
        });

        let store = Store::open_in_memory().unwrap();
        let word_entry = gp_store::NewWord {
            word: "serendipity".into(),
            lang: "en".into(),
            translation: "счастливая случайность".into(),
            sentence: "What a serendipity!".into(),
            ..Default::default()
        };
        let vocab_id = store.save_word(&word_entry).unwrap();

        let ctx = egui::Context::default();
        let jobs = Jobs::new(ctx, store);

        jobs.export_custom_dict(vocab_id, url, Some("testtoken123".into()), word_entry);

        let (request_line, auth_found, body_str) = server_thread.join().unwrap();
        assert!(request_line.starts_with("POST /api/words"));
        assert!(auth_found, "Authorization header must be present");
        assert!(body_str.contains("serendipity"));
        assert!(body_str.contains("счастливая случайность"));

        let msg = jobs.rx.recv_timeout(Duration::from_secs(5)).unwrap();
        match msg {
            Msg::CustomDict {
                vocab_id: id,
                result,
            } => {
                assert_eq!(id, vocab_id);
                assert!(result.is_ok());
            }
            _ => panic!("Expected Msg::CustomDict"),
        }

        jobs.store
            .lock()
            .unwrap()
            .set_custom_dict_synced(vocab_id, 1700000000)
            .unwrap();
        let words = jobs.store.lock().unwrap().list_words().unwrap();
        assert_eq!(words[0].custom_dict_synced_at, Some(1700000000));
    }
}
