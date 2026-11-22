use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub player: Player,
    pub subtitles: Subtitles,
    pub translation: Translation,
    pub asr: Asr,
    pub tts: Tts,
    pub training: Training,
    pub anki: Anki,
    pub custom_dict: CustomDict,
    pub sources: Sources,
}

impl Config {
    /// Missing fields take their defaults; unknown ones are ignored.
    pub fn from_toml(s: &str) -> Result<Config, toml::de::Error> {
        toml::from_str(s)
    }

    /// "line 3: invalid type..." for the first problem, short enough for a toast.
    pub fn describe_error(text: &str, e: &toml::de::Error) -> String {
        match e.span() {
            Some(span) => {
                let line = text[..span.start.min(text.len())].matches('\n').count() + 1;
                format!("line {line}: {}", e.message())
            }
            None => e.message().to_owned(),
        }
    }

    pub fn to_toml(&self) -> String {
        toml::to_string_pretty(self).expect("config has only serializable fields")
    }

    /// `theirs` with every setting `ours` changed since `base`: the file as someone else left
    /// it, plus what was changed here since both were the same.
    pub fn merge(base: &Config, ours: &Config, theirs: &Config) -> Config {
        fn value(c: &Config) -> toml::Table {
            toml::Table::try_from(c).expect("config has only serializable fields")
        }
        fn apply(base: &toml::Table, ours: &toml::Table, theirs: &mut toml::Table) {
            for (key, mine) in ours {
                let was = base.get(key);
                match (mine, was, theirs.get_mut(key)) {
                    (
                        toml::Value::Table(m),
                        Some(toml::Value::Table(b)),
                        Some(toml::Value::Table(t)),
                    ) => {
                        apply(b, m, t);
                    }
                    _ if was != Some(mine) => {
                        theirs.insert(key.clone(), mine.clone());
                    }
                    _ => {}
                }
            }
        }
        let mut out = value(theirs);
        apply(&value(base), &value(ours), &mut out);
        out.try_into().unwrap_or_else(|_| ours.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct General {
    /// Empty means "detect".
    pub learn_lang: String,
    pub native_lang: String,
    /// Interface language: "auto" follows the system, otherwise "en", "ru" or "uk".
    pub ui_lang: String,
    pub ui_scale: f32,
    /// Windows only: "Open with" and context-menu entries in Explorer.
    pub shell_integration: bool,
    /// "dark", "light" or "system"; anything else means dark.
    pub theme: String,
    /// Where speech recognition and the voice engine are downloaded; empty means the data folder.
    pub components_dir: String,
}

impl Default for General {
    fn default() -> Self {
        General {
            learn_lang: "en".into(),
            native_lang: "ru".into(),
            ui_lang: "auto".into(),
            ui_scale: 1.0,
            shell_integration: false,
            theme: "dark".into(),
            components_dir: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Player {
    pub hwdec: String,
    pub volume: f64,
    pub mpv_path: String,
    pub ffmpeg_path: String,
    pub ytdl_path: String,
    /// Where saved frames go; empty means "Pictures/Glossplay".
    pub frames_dir: String,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            hwdec: "auto".into(),
            volume: 100.0,
            mpv_path: String::new(),
            ffmpeg_path: String::new(),
            ytdl_path: String::new(),
            frames_dir: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Subtitles {
    /// In points at a window height of 720; scales with the window.
    pub font_size: f32,
    pub secondary_scale: f32,
    /// A look from `glossplay`'s list of subtitle styles; an unknown name means the default one.
    pub style: String,
    /// One multiplier for the original and the translation: 0.6 to 2.0.
    pub scale: f32,
    /// `bottom` or `top`.
    pub position: String,
    /// Extra distance from that edge, in points at a window height of 720.
    pub margin: f32,
    pub show_primary: bool,
    pub show_secondary: bool,
    pub auto_translate: bool,
    /// `rows` (stacked vertically) or `columns` (side-by-side in one horizontal row).
    pub layout: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubtitleLayout {
    #[default]
    Rows,
    Columns,
}

impl SubtitleLayout {
    pub const ALL: [Self; 2] = [Self::Rows, Self::Columns];

    pub fn id(self) -> &'static str {
        match self {
            Self::Rows => "rows",
            Self::Columns => "columns",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "columns" => Self::Columns,
            _ => Self::Rows,
        }
    }
}

impl Default for Subtitles {
    fn default() -> Self {
        Subtitles {
            font_size: 30.0,
            secondary_scale: 0.8,
            style: "plate".into(),
            scale: 1.0,
            position: "bottom".into(),
            margin: 0.0,
            show_primary: true,
            show_secondary: true,
            auto_translate: true,
            layout: "rows".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Translation {
    /// google | deepl | openai | anthropic
    pub provider: String,
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    /// How many lines ahead of the playhead I translate in advance.
    pub lookahead: usize,
}

impl Default for Translation {
    fn default() -> Self {
        Translation {
            provider: "google".into(),
            model: String::new(),
            base_url: String::new(),
            api_key: String::new(),
            lookahead: 40,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Asr {
    pub enabled: bool,
    pub engine: String,
    pub server_path: String,
    pub model_path: String,
    pub chunk_sec: u32,
    /// What the install button sets up: "auto", "nvidia" or "cpu".
    pub install_engine: String,
    /// Whisper model for the install button; empty picks the one that suits the engine.
    pub install_model: String,
}

impl Default for Asr {
    fn default() -> Self {
        Asr {
            enabled: false,
            engine: "whisper-server".into(),
            server_path: String::new(),
            model_path: String::new(),
            chunk_sec: 30,
            install_engine: "auto".into(),
            install_model: String::new(),
        }
    }
}

/// Voice-over: the translation spoken over the original sound.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Tts {
    pub enabled: bool,
    /// Volume of the original sound while the voice-over plays, in percent of the normal one as
    /// heard (see `dub_plan::percent_gain`); the voice volume is in the same percents.
    pub original_volume: f64,
    pub voice_volume: f64,
    /// Whose voice the installed engine speaks in: "speaker", copied from the film's own
    /// speech, or "default", one standard voice. `voice` below names a voice of a server of
    /// the user's own instead.
    pub voice_source: String,
    /// What the install button sets up: "vulkan" (any graphics card) or "nvidia".
    pub install_engine: String,
    /// An OpenAI-style speech server of the user's own; empty runs the installed engine.
    pub base_url: String,
    pub model: String,
    pub voice: String,
}

impl Default for Tts {
    fn default() -> Self {
        Tts {
            enabled: false,
            original_volume: 35.0,
            voice_volume: 100.0,
            voice_source: "speaker".into(),
            install_engine: "vulkan".into(),
            base_url: String::new(),
            model: String::new(),
            voice: String::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Training {
    pub autopause: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Anki {
    pub url: String,
    pub deck: String,
}

impl Default for Anki {
    fn default() -> Self {
        Anki {
            url: "http://127.0.0.1:8765".into(),
            deck: "Glossplay".into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CustomDict {
    pub url: String,
    pub token: String,
}

/// The user's own search server (Torznab) and TorrServer. Nothing is built in: search stays
/// hidden until both addresses are filled in.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Sources {
    pub torznab_url: String,
    pub torznab_key: String,
    pub torrserver_url: String,
}

impl Sources {
    pub fn search_ready(&self) -> bool {
        !self.torznab_url.trim().is_empty() && !self.torrserver_url.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_save_keeps_what_was_edited_in_the_file_meanwhile() {
        let base = Config::default();
        // Changed in the player: the volume and the auto-pause.
        let mut ours = base.clone();
        ours.player.volume = 40.0;
        ours.training.autopause = !base.training.autopause;
        // Changed in the file by hand while the player was open: Whisper's paths, and the
        // volume too.
        let mut theirs = base.clone();
        theirs.asr.model_path = r"E:\models\large.bin".into();
        theirs.asr.server_path = r"E:\whisper\server.exe".into();
        theirs.player.volume = 70.0;
        let m = Config::merge(&base, &ours, &theirs);
        assert_eq!(m.asr.model_path, r"E:\models\large.bin");
        assert_eq!(m.asr.server_path, r"E:\whisper\server.exe");
        assert_eq!(m.training.autopause, ours.training.autopause);
        assert_eq!(m.player.volume, 40.0, "the last change made wins");
        // Nothing changed here: the file stays as it is.
        assert_eq!(Config::merge(&base, &base, &theirs), theirs);
    }

    #[test]
    fn search_needs_both_addresses() {
        let c = Config::from_toml("").unwrap();
        assert_eq!(c.sources, Sources::default());
        assert!(!c.sources.search_ready());
        let only_search =
            Config::from_toml("[sources]\ntorznab_url = \"http://indexer.test/api\"\n").unwrap();
        assert!(!only_search.sources.search_ready());
        let blank = Config::from_toml(
            "[sources]\ntorznab_url = \"http://indexer.test/api\"\ntorrserver_url = \"  \"\n",
        )
        .unwrap();
        assert!(!blank.sources.search_ready());
        let both = Config::from_toml(
            "[sources]\ntorznab_url = \"http://indexer.test/api\"\ntorrserver_url = \"http://ts.test:8090\"\n",
        )
        .unwrap();
        assert!(both.sources.search_ready());
        assert_eq!(both.sources.torznab_key, "");
    }

    #[test]
    fn empty_and_partial_files_fall_back_to_defaults() {
        assert_eq!(Config::from_toml("").unwrap(), Config::default());
        let c = Config::from_toml("[general]\nnative_lang = \"de\"\nfuture_field = 1\n").unwrap();
        assert_eq!(c.general.native_lang, "de");
        assert_eq!(c.general.learn_lang, "en");
        assert_eq!(c.general.ui_lang, "auto");
        assert_eq!(c.general.theme, "dark");
        assert!(!c.general.shell_integration);
        let on = Config::from_toml("[general]\nshell_integration = true\n").unwrap();
        assert!(on.general.shell_integration);
        assert_eq!(c.translation.provider, "google");
        assert!(!c.tts.enabled, "the voice-over stays off until asked for");
        assert_eq!((c.tts.original_volume, c.tts.voice_volume), (35.0, 100.0));
        assert_eq!(c.tts.voice_source, "speaker");
    }

    #[test]
    fn subtitle_look_defaults_keep_the_old_appearance() {
        let old = Config::from_toml(
            "[subtitles]
font_size = 36.0
",
        )
        .unwrap();
        assert_eq!(old.subtitles.font_size, 36.0);
        assert_eq!(
            (old.subtitles.style.as_str(), old.subtitles.scale),
            ("plate", 1.0)
        );
        assert_eq!(
            (old.subtitles.position.as_str(), old.subtitles.margin),
            ("bottom", 0.0)
        );
        let odd = Config::from_toml(
            "[subtitles]
style = \"from-the-future\"
",
        )
        .unwrap();
        assert_eq!(odd.subtitles.style, "from-the-future");
    }

    #[test]
    fn roundtrip() {
        let mut c = Config::default();
        c.translation.provider = "openai".into();
        c.training.autopause = true;
        assert_eq!(Config::from_toml(&c.to_toml()).unwrap(), c);
    }

    #[test]
    fn bad_type_is_an_error() {
        assert!(Config::from_toml("[player]\nvolume = \"loud\"\n").is_err());
    }

    #[test]
    fn custom_dict_config_ac1() {
        let toml_str = r#"
[custom_dict]
url = "http://127.0.0.1:9999/api/words"
token = "secret123"
"#;
        let c = Config::from_toml(toml_str).unwrap();
        assert_eq!(c.custom_dict.url, "http://127.0.0.1:9999/api/words");
        assert_eq!(c.custom_dict.token, "secret123");
        let serialized = c.to_toml();
        let back = Config::from_toml(&serialized).unwrap();
        assert_eq!(back.custom_dict, c.custom_dict);
    }

    #[test]
    fn subtitle_layout_config_roundtrip_ac3() {
        let old = Config::from_toml("[subtitles]\nfont_size = 28.0\n").unwrap();
        assert_eq!(old.subtitles.layout, "rows");
        assert_eq!(SubtitleLayout::from_id(&old.subtitles.layout), SubtitleLayout::Rows);

        let custom = Config::from_toml("[subtitles]\nlayout = \"columns\"\n").unwrap();
        assert_eq!(custom.subtitles.layout, "columns");
        assert_eq!(SubtitleLayout::from_id(&custom.subtitles.layout), SubtitleLayout::Columns);

        let back = Config::from_toml(&custom.to_toml()).unwrap();
        assert_eq!(back.subtitles.layout, "columns");
    }
}
