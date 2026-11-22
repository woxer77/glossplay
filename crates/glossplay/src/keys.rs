//! Player shortcuts. A key that types a character is matched by its place on the keyboard,
//! so S repeats a line with a Russian layout too, where that key types "ы". Arrows, Space
//! and Esc are matched by what they are, so the arrows of the number pad still seek.
//!
//! The same table serves the keyboard, the self-test that replays layouts, and MCP.

use eframe::egui::{Event, Key, Modifiers, PointerButton, pos2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    PlayPause,
    Back,
    Forward,
    VolumeUp,
    VolumeDown,
    PreviousLine,
    RepeatLine,
    NextLine,
    AutoPause,
    Fullscreen,
    Sidebar,
    Mute,
    ToggleOriginal,
    ToggleTranslation,
    Slower,
    Faster,
    SubtitlesSmaller,
    SubtitlesLarger,
    OpenFile,
    Search,
    VideoInfo,
    Escape,
}

impl Command {
    pub fn name(self) -> &'static str {
        match self {
            Command::PlayPause => "play_pause",
            Command::Back => "back_5s",
            Command::Forward => "forward_5s",
            Command::VolumeUp => "volume_up",
            Command::VolumeDown => "volume_down",
            Command::PreviousLine => "previous_line",
            Command::RepeatLine => "repeat_line",
            Command::NextLine => "next_line",
            Command::AutoPause => "auto_pause",
            Command::Fullscreen => "fullscreen",
            Command::Sidebar => "sidebar",
            Command::Mute => "mute",
            Command::ToggleOriginal => "toggle_original",
            Command::ToggleTranslation => "toggle_translation",
            Command::Slower => "slower",
            Command::Faster => "faster",
            Command::SubtitlesSmaller => "subtitles_smaller",
            Command::SubtitlesLarger => "subtitles_larger",
            Command::OpenFile => "open_file",
            Command::Search => "search",
            Command::VideoInfo => "video_info",
            Command::Escape => "escape",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Chord {
    /// No Ctrl and no Alt: Alt+Space is the window menu, and AltGr arrives as Ctrl+Alt.
    Plain,
    Ctrl,
    Any,
}

const BINDINGS: [(Key, Chord, Command); 23] = [
    (Key::Space, Chord::Plain, Command::PlayPause),
    (Key::ArrowLeft, Chord::Plain, Command::Back),
    (Key::ArrowRight, Chord::Plain, Command::Forward),
    (Key::ArrowUp, Chord::Plain, Command::VolumeUp),
    (Key::ArrowDown, Chord::Plain, Command::VolumeDown),
    (Key::A, Chord::Plain, Command::PreviousLine),
    (Key::S, Chord::Plain, Command::RepeatLine),
    (Key::D, Chord::Plain, Command::NextLine),
    (Key::P, Chord::Plain, Command::AutoPause),
    (Key::F, Chord::Plain, Command::Fullscreen),
    (Key::T, Chord::Plain, Command::Sidebar),
    (Key::M, Chord::Plain, Command::Mute),
    (Key::Num1, Chord::Plain, Command::ToggleOriginal),
    (Key::Num2, Chord::Plain, Command::ToggleTranslation),
    (Key::OpenBracket, Chord::Plain, Command::Slower),
    (Key::CloseBracket, Chord::Plain, Command::Faster),
    (Key::Minus, Chord::Plain, Command::SubtitlesSmaller),
    (Key::Equals, Chord::Plain, Command::SubtitlesLarger),
    (Key::Plus, Chord::Plain, Command::SubtitlesLarger),
    (Key::O, Chord::Ctrl, Command::OpenFile),
    (Key::F, Chord::Ctrl, Command::Search),
    (Key::I, Chord::Plain, Command::VideoInfo),
    (Key::Escape, Chord::Any, Command::Escape),
];

/// The keys bound to `command`, written the way people know them: "Space", "A", "Ctrl+O".
pub fn keys_of(command: Command) -> Vec<String> {
    let ctrl = if cfg!(target_os = "macos") {
        "Cmd+"
    } else {
        "Ctrl+"
    };
    BINDINGS
        .iter()
        .filter(|(_, _, c)| *c == command)
        .map(|(key, chord, _)| {
            let name = match key {
                Key::Space => crate::i18n::t(crate::i18n::Tr::KeySpace),
                Key::Escape => "Esc",
                other => other.symbol_or_name(),
            };
            match chord {
                Chord::Ctrl => format!("{ctrl}{name}"),
                _ => name.to_owned(),
            }
        })
        .collect()
}

/// What else holds the keyboard at the moment.
#[derive(Clone, Copy, Default, Debug)]
pub struct Gate {
    /// A widget has the keyboard focus, usually a text field.
    pub typing: bool,
    /// Settings, a setup dialog or the search window is open.
    pub modal: bool,
}

impl Gate {
    /// Esc closes the top window from anywhere. Ctrl chords are not text editing, so they
    /// work from a text field too, but not over a dialog. Everything else is the player's
    /// own and waits until no field or dialog has the keyboard.
    pub fn lets(self, command: Command) -> bool {
        match command {
            Command::Escape => true,
            Command::OpenFile | Command::Search => !self.modal,
            _ => !self.modal && !self.typing,
        }
    }

    pub fn lets_paste(self) -> bool {
        !self.modal && !self.typing
    }

    /// Why a shortcut did nothing, for an agent.
    pub fn refusal(self) -> &'static str {
        if self.modal {
            "a dialog or the search window is open; only escape works over it"
        } else {
            "a text field has the keyboard focus"
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Run(Command),
    /// Ctrl+V outside a text field, with what the clipboard held. Empty when it held no text.
    Paste(String),
}

impl Action {
    pub fn name(&self) -> &'static str {
        match self {
            Action::Run(c) => c.name(),
            Action::Paste(_) => "paste",
        }
    }
}

fn types_a_character(key: Key) -> bool {
    use Key::*;
    matches!(
        key,
        A | B
            | C
            | D
            | E
            | F
            | G
            | H
            | I
            | J
            | K
            | L
            | M
            | N
            | O
            | P
            | Q
            | R
            | S
            | T
            | U
            | V
            | W
            | X
            | Y
            | Z
            | Num0
            | Num1
            | Num2
            | Num3
            | Num4
            | Num5
            | Num6
            | Num7
            | Num8
            | Num9
            | OpenBracket
            | CloseBracket
            | OpenCurlyBracket
            | CloseCurlyBracket
            | Semicolon
            | Colon
            | Quote
            | Comma
            | Period
            | Slash
            | Backslash
            | Pipe
            | Backtick
            | Minus
            | Plus
            | Equals
            | Questionmark
            | Exclamationmark
    )
}

/// The key a press counts as. egui-winit gives `key` from the layout and falls back to the
/// physical key only when the layout's character has no egui key (Cyrillic); a layout with
/// Latin letters elsewhere (Dvorak, AZERTY) would move the shortcuts, so a character key is
/// taken by its place. A named key (arrow, Space, Esc, Home on the number pad) stays itself.
pub fn placed(key: Key, physical: Option<Key>) -> Key {
    match physical {
        Some(p) if types_a_character(key) && types_a_character(p) => p,
        _ => key,
    }
}

fn chord_matches(chord: Chord, m: Modifiers) -> bool {
    match chord {
        Chord::Plain => !m.command && !m.ctrl && !m.alt,
        Chord::Ctrl => m.command && !m.alt,
        Chord::Any => true,
    }
}

pub fn command_for(key: Key, physical: Option<Key>, modifiers: Modifiers) -> Option<Command> {
    let at = placed(key, physical);
    BINDINGS
        .iter()
        .find(|(k, chord, _)| *k == at && chord_matches(*chord, modifiers))
        .map(|(_, _, c)| *c)
}

/// The shortcuts in one frame's input, in the order they came, minus those the gate holds.
pub fn actions(events: &[Event], gate: Gate) -> Vec<Action> {
    let mut out = Vec::new();
    for e in events {
        match e {
            Event::Key {
                key,
                physical_key,
                pressed: true,
                modifiers,
                ..
            } => {
                if let Some(c) = command_for(*key, *physical_key, *modifiers)
                    && gate.lets(c)
                {
                    out.push(Action::Run(c));
                }
            }
            Event::PointerButton {
                button: PointerButton::Extra1,
                pressed: true,
                ..
            } if gate.lets(Command::Back) => {
                out.push(Action::Run(Command::Back));
            }
            Event::PointerButton {
                button: PointerButton::Extra2,
                pressed: true,
                ..
            } if gate.lets(Command::Forward) => {
                out.push(Action::Run(Command::Forward));
            }
            Event::Paste(text) if gate.lets_paste() => out.push(Action::Paste(text.clone())),
            Event::PasteImage(_) if gate.lets_paste() => out.push(Action::Paste(String::new())),
            _ => {}
        }
    }
    out
}

/// Events that come from the keyboard (or the clipboard through it).
pub fn is_keyboard(e: &Event) -> bool {
    matches!(
        e,
        Event::Key { .. }
            | Event::Text(_)
            | Event::Copy
            | Event::Cut
            | Event::Paste(_)
            | Event::PasteImage(_)
            | Event::Ime(_)
            | Event::ModifiersChanged(_)
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pasted {
    Link(String),
    Magnet(String),
    File(String),
    Nothing,
}

/// What a pasted text points to: a web or magnet link, or a file that exists. Explorer's
/// "Copy as path" puts the path in quotes, one line per file; the first one is taken.
pub fn classify(text: &str, is_file: impl Fn(&str) -> bool) -> Pasted {
    let Some(line) = text.lines().map(str::trim).find(|l| !l.is_empty()) else {
        return Pasted::Nothing;
    };
    let line = unquote(line);
    let lower = line.to_ascii_lowercase();
    let one_word = !line.contains(char::is_whitespace);
    if (lower.starts_with("http://") || lower.starts_with("https://")) && one_word {
        return Pasted::Link(line.to_owned());
    }
    if lower.starts_with("magnet:?") && one_word {
        return Pasted::Magnet(line.to_owned());
    }
    if !line.contains("://") && is_file(line) {
        return Pasted::File(line.to_owned());
    }
    Pasted::Nothing
}

fn unquote(s: &str) -> &str {
    for q in ['"', '\''] {
        if let Some(inner) = s.strip_prefix(q).and_then(|r| r.strip_suffix(q)) {
            return inner.trim();
        }
    }
    s
}

/// A keyboard layout the self-test and MCP can press keys in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Layout {
    En,
    Ru,
}

impl Layout {
    pub fn parse(s: &str) -> Option<Layout> {
        match s {
            "en" => Some(Layout::En),
            "ru" => Some(Layout::Ru),
            _ => None,
        }
    }

    /// What the key at the place of `physical` types, unshifted.
    fn char_at(self, physical: Key) -> Option<&'static str> {
        const EN: &str = "abcdefghijklmnopqrstuvwxyz";
        // ЙЦУКЕН at the places of A…Z.
        const RU: [&str; 26] = [
            "ф", "и", "с", "в", "у", "а", "п", "р", "ш", "о", "л", "д", "ь", "т", "щ", "з", "й",
            "к", "ы", "е", "г", "м", "ц", "ч", "н", "я",
        ];
        let letters = [
            Key::A,
            Key::B,
            Key::C,
            Key::D,
            Key::E,
            Key::F,
            Key::G,
            Key::H,
            Key::I,
            Key::J,
            Key::K,
            Key::L,
            Key::M,
            Key::N,
            Key::O,
            Key::P,
            Key::Q,
            Key::R,
            Key::S,
            Key::T,
            Key::U,
            Key::V,
            Key::W,
            Key::X,
            Key::Y,
            Key::Z,
        ];
        if let Some(i) = letters.iter().position(|k| *k == physical) {
            return Some(match self {
                Layout::En => &EN[i..=i],
                Layout::Ru => RU[i],
            });
        }
        let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let numbers = [
            Key::Num0,
            Key::Num1,
            Key::Num2,
            Key::Num3,
            Key::Num4,
            Key::Num5,
            Key::Num6,
            Key::Num7,
            Key::Num8,
            Key::Num9,
        ];
        if let Some(i) = numbers.iter().position(|k| *k == physical) {
            return Some(digits[i]);
        }
        let (en, ru) = match physical {
            Key::OpenBracket => ("[", "х"),
            Key::CloseBracket => ("]", "ъ"),
            Key::Semicolon => (";", "ж"),
            Key::Quote => ("'", "э"),
            Key::Comma => (",", "б"),
            Key::Period => (".", "ю"),
            Key::Slash => ("/", "."),
            Key::Backtick => ("`", "ё"),
            Key::Minus => ("-", "-"),
            Key::Equals => ("=", "="),
            Key::Backslash => ("\\", "\\"),
            _ => return None,
        };
        Some(if self == Layout::En { en } else { ru })
    }
}

/// A key press as written in a test or by an agent: `s`, `[`, `space`, `ctrl+v`, `shift+s`.
#[derive(Clone, Debug, PartialEq)]
pub struct Combo {
    pub physical: Key,
    pub modifiers: Modifiers,
}

/// What egui-winit reports for the platform's main modifier: Cmd on macOS (`mac_cmd` and
/// `command`, no `ctrl`), Ctrl elsewhere (`ctrl` and `command`); see `State::on_modifiers_changed`.
pub fn primary_modifiers(mac: bool) -> Modifiers {
    if mac {
        Modifiers::MAC_CMD | Modifiers::COMMAND
    } else {
        Modifiers::CTRL | Modifiers::COMMAND
    }
}

pub fn parse_combo(s: &str) -> Result<Combo, String> {
    parse_combo_for(s, cfg!(target_os = "macos"))
}

/// `ctrl`, `cmd` and `command` all mean the platform's main modifier; I never produce a
/// real Ctrl on macOS because nothing in the player is bound to it there.
pub fn parse_combo_for(s: &str, mac: bool) -> Result<Combo, String> {
    let s = s.trim().to_lowercase();
    let mut parts: Vec<&str> = s.split('+').collect();
    // "ctrl++" would be a plus key; nothing here is bound to it.
    let name = parts.pop().unwrap_or_default();
    let mut modifiers = Modifiers::NONE;
    for m in parts {
        match m {
            "ctrl" | "control" | "cmd" | "command" => {
                modifiers |= primary_modifiers(mac);
            }
            "shift" => modifiers.shift = true,
            "alt" => modifiers.alt = true,
            other => return Err(format!("unknown modifier \"{other}\"")),
        }
    }
    let physical = match name {
        "space" => Key::Space,
        "left" => Key::ArrowLeft,
        "right" => Key::ArrowRight,
        "up" => Key::ArrowUp,
        "down" => Key::ArrowDown,
        "escape" | "esc" => Key::Escape,
        "enter" => Key::Enter,
        "tab" => Key::Tab,
        "backspace" => Key::Backspace,
        "delete" | "del" => Key::Delete,
        "home" => Key::Home,
        "end" => Key::End,
        other if other.chars().count() == 1 => {
            Key::from_name(other).ok_or_else(|| format!("unknown key \"{other}\""))?
        }
        other => return Err(format!("unknown key \"{other}\"")),
    };
    Ok(Combo {
        physical,
        modifiers,
    })
}

/// A press and the release after it, as egui-winit 0.37 turns the OS events into egui
/// events (`State::on_keyboard_input`): `key` is the layout's key or, failing that, the
/// physical one; Ctrl+X/C/V become Cut/Copy/Paste instead of a key press; text comes as
/// `Text` unless Ctrl is held. `clipboard` stands in for the system clipboard.
pub struct Synth {
    pub press: Vec<Event>,
    pub release: Vec<Event>,
}

pub fn synthesize(layout: Layout, combo: &Combo, clipboard: &str) -> Synth {
    let m = combo.modifiers;
    let typed = layout.char_at(combo.physical);
    let logical = match typed {
        Some(c) => Key::from_name(c),
        None => Some(combo.physical),
    };
    let key = logical.unwrap_or(combo.physical);
    let mut press = Vec::new();
    let mut release = Vec::new();
    if m.any() {
        press.push(Event::ModifiersChanged(m));
    }
    match key {
        Key::X if m.command_only() => press.push(Event::Cut),
        Key::C if m.command_only() => press.push(Event::Copy),
        Key::V if m.command_only() => {
            // An empty clipboard gives no event at all.
            let text = clipboard.replace("\r\n", "\n");
            if !text.is_empty() {
                press.push(Event::Paste(text));
            }
        }
        _ => press.push(Event::Key {
            key,
            physical_key: Some(combo.physical),
            pressed: true,
            repeat: false,
            modifiers: m,
        }),
    }
    let text = match (typed, combo.physical) {
        (Some(c), _) if m.shift => Some(c.to_uppercase()),
        (Some(c), _) => Some(c.to_owned()),
        (None, Key::Space) => Some(" ".to_owned()),
        _ => None,
    };
    if let Some(text) = text
        && !(m.ctrl || m.command || m.mac_cmd)
    {
        press.push(Event::Text(text));
    }
    release.push(Event::Key {
        key,
        physical_key: Some(combo.physical),
        pressed: false,
        repeat: false,
        modifiers: m,
    });
    if m.any() {
        release.push(Event::ModifiersChanged(Modifiers::NONE));
    }
    Synth { press, release }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test runs as on Windows/Linux and as on macOS, whatever the host is.
    const PLATFORMS: [bool; 2] = [false, true];

    fn press(mac: bool, layout: Layout, combo: &str, clipboard: &str) -> Vec<Event> {
        synthesize(layout, &parse_combo_for(combo, mac).unwrap(), clipboard).press
    }

    fn run(mac: bool, layout: Layout, combo: &str, gate: Gate) -> Vec<Action> {
        actions(&press(mac, layout, combo, ""), gate)
    }

    #[test]
    fn the_help_names_the_keys_of_the_table() {
        assert_eq!(keys_of(Command::RepeatLine), ["S"]);
        assert_eq!(keys_of(Command::SubtitlesLarger), ["=", "+"]);
        assert_eq!(keys_of(Command::Escape), ["Esc"]);
        let open = keys_of(Command::OpenFile);
        assert!(open == ["Ctrl+O"] || open == ["Cmd+O"], "{open:?}");
    }

    const FREE: Gate = Gate {
        typing: false,
        modal: false,
    };
    const TYPING: Gate = Gate {
        typing: true,
        modal: false,
    };
    const MODAL: Gate = Gate {
        typing: false,
        modal: true,
    };

    #[test]
    fn a_russian_layout_sends_the_physical_key_and_cyrillic_text() {
        for mac in PLATFORMS {
            let events = press(mac, Layout::Ru, "s", "");
            assert_eq!(
                events,
                [
                    Event::Key {
                        key: Key::S,
                        physical_key: Some(Key::S),
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::NONE,
                    },
                    Event::Text("ы".into()),
                ]
            );
            assert_eq!(
                press(mac, Layout::Ru, "ctrl+v", "C:\\a.mkv"),
                [
                    Event::ModifiersChanged(primary_modifiers(mac)),
                    Event::Paste("C:\\a.mkv".into()),
                ]
            );
        }
    }

    #[test]
    fn every_letter_shortcut_works_the_same_in_both_layouts() {
        for mac in PLATFORMS {
            let cases = [
                ("space", Command::PlayPause),
                ("left", Command::Back),
                ("right", Command::Forward),
                ("up", Command::VolumeUp),
                ("down", Command::VolumeDown),
                ("a", Command::PreviousLine),
                ("s", Command::RepeatLine),
                ("d", Command::NextLine),
                ("p", Command::AutoPause),
                ("f", Command::Fullscreen),
                ("t", Command::Sidebar),
                ("m", Command::Mute),
                ("1", Command::ToggleOriginal),
                ("2", Command::ToggleTranslation),
                ("[", Command::Slower),
                ("]", Command::Faster),
                ("-", Command::SubtitlesSmaller),
                ("=", Command::SubtitlesLarger),
                ("shift+s", Command::RepeatLine),
                ("ctrl+o", Command::OpenFile),
                ("ctrl+f", Command::Search),
                ("i", Command::VideoInfo),
                ("escape", Command::Escape),
            ];
            for layout in [Layout::En, Layout::Ru] {
                for (combo, want) in cases {
                    assert_eq!(
                        run(mac, layout, combo, FREE),
                        [Action::Run(want)],
                        "{layout:?} {combo}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_latin_layout_with_letters_elsewhere_keeps_the_places() {
        // Dvorak: the key at the place of S types "o", the one at ";" types "s".
        let at_s = Event::Key {
            key: Key::O,
            physical_key: Some(Key::S),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        let at_semicolon = Event::Key {
            key: Key::S,
            physical_key: Some(Key::Semicolon),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        assert_eq!(actions(&[at_s], FREE), [Action::Run(Command::RepeatLine)]);
        assert_eq!(actions(&[at_semicolon], FREE), []);
    }

    #[test]
    fn number_pad_arrows_and_an_unknown_place_keep_the_key() {
        // Num Lock off: the place is Numpad4 (egui calls it Num4), the key is an arrow.
        assert_eq!(placed(Key::ArrowLeft, Some(Key::Num4)), Key::ArrowLeft);
        assert_eq!(placed(Key::Num1, Some(Key::Num1)), Key::Num1);
        assert_eq!(placed(Key::S, None), Key::S);
    }

    #[test]
    fn typing_in_a_field_keeps_the_player_shortcuts_off() {
        for mac in PLATFORMS {
            for layout in [Layout::En, Layout::Ru] {
                for combo in ["s", "a", "space", "1", "[", "left", "m"] {
                    assert_eq!(run(mac, layout, combo, TYPING), [], "{layout:?} {combo}");
                }
                assert_eq!(
                    run(mac, layout, "ctrl+f", TYPING),
                    [Action::Run(Command::Search)]
                );
                assert_eq!(
                    run(mac, layout, "ctrl+o", TYPING),
                    [Action::Run(Command::OpenFile)]
                );
                assert_eq!(
                    run(mac, layout, "escape", TYPING),
                    [Action::Run(Command::Escape)]
                );
                assert_eq!(
                    actions(&press(mac, layout, "ctrl+v", "https://x.test/v"), TYPING),
                    []
                );
            }
        }
    }

    #[test]
    fn a_dialog_lets_only_escape_through() {
        for mac in PLATFORMS {
            for combo in ["s", "space", "ctrl+o", "ctrl+f", "f"] {
                assert_eq!(run(mac, Layout::Ru, combo, MODAL), [], "{combo}");
            }
            assert_eq!(
                run(mac, Layout::Ru, "escape", MODAL),
                [Action::Run(Command::Escape)]
            );
            assert_eq!(
                actions(&press(mac, Layout::En, "ctrl+v", "https://x.test/v"), MODAL),
                []
            );
        }
    }

    #[test]
    fn modifiers_keep_letters_from_firing() {
        for mac in PLATFORMS {
            for combo in ["ctrl+s", "alt+s", "ctrl+space", "alt+space", "ctrl+1"] {
                assert_eq!(run(mac, Layout::En, combo, FREE), [], "{combo}");
            }
            // Ctrl+V with nothing on the clipboard reaches nobody, as with egui-winit.
            assert_eq!(run(mac, Layout::Ru, "ctrl+v", FREE), []);
            assert_eq!(
                actions(&press(mac, Layout::Ru, "ctrl+v", "x"), FREE),
                [Action::Paste("x".into())]
            );
            // A release is not a press.
            let release = synthesize(Layout::En, &parse_combo("s").unwrap(), "").release;
            assert_eq!(actions(&release, FREE), []);
        }
    }

    #[test]
    fn pasted_text_is_a_link_a_magnet_a_file_or_nothing() {
        let exists = |p: &str| p == "C:\\Videos\\a b.mkv" || p == "/home/me/v.mp4";
        assert_eq!(
            classify("  https://www.youtube.com/watch?v=x\r\n", exists),
            Pasted::Link("https://www.youtube.com/watch?v=x".into())
        );
        assert_eq!(
            classify("HTTP://Example.test/v.mp4", exists),
            Pasted::Link("HTTP://Example.test/v.mp4".into())
        );
        assert_eq!(
            classify("magnet:?xt=urn:btih:08ada5a7&dn=Sintel", exists),
            Pasted::Magnet("magnet:?xt=urn:btih:08ada5a7&dn=Sintel".into())
        );
        assert_eq!(
            classify("\"C:\\Videos\\a b.mkv\"\r\n\"C:\\Videos\\c.mkv\"", exists),
            Pasted::File("C:\\Videos\\a b.mkv".into())
        );
        assert_eq!(
            classify("C:\\Videos\\a b.mkv", exists),
            Pasted::File("C:\\Videos\\a b.mkv".into())
        );
        assert_eq!(
            classify("'/home/me/v.mp4'", exists),
            Pasted::File("/home/me/v.mp4".into())
        );
        for junk in [
            "",
            "  \n ",
            "hello world",
            "C:\\Videos\\missing.mkv",
            "see https://x.test",
            "https://x.test and more",
            "ftp://x.test/v.mkv",
        ] {
            assert_eq!(classify(junk, exists), Pasted::Nothing, "{junk:?}");
        }
    }

    #[test]
    fn combos_parse_and_bad_ones_say_why() {
        assert_eq!(parse_combo("Ctrl+V").unwrap().physical, Key::V);
        assert!(parse_combo("ctrl+v").unwrap().modifiers.command);
        assert_eq!(parse_combo("[").unwrap().physical, Key::OpenBracket);
        assert_eq!(parse_combo("esc").unwrap().physical, Key::Escape);
        // The keys a field is cleared and edited with, for the recording pointer.
        assert_eq!(parse_combo("backspace").unwrap().physical, Key::Backspace);
        assert_eq!(parse_combo("Delete").unwrap().physical, Key::Delete);
        assert_eq!(parse_combo("home").unwrap().physical, Key::Home);
        assert_eq!(parse_combo("end").unwrap().physical, Key::End);
        let all = synthesize(Layout::En, &parse_combo("ctrl+a").unwrap(), "");
        assert!(
            all.press.iter().any(
                |e| matches!(e, Event::Key { key: Key::A, modifiers, .. } if modifiers.command)
            ),
            "{:?}",
            all.press
        );
        assert!(
            !all.press.iter().any(|e| matches!(e, Event::Text(_))),
            "ctrl+a types nothing"
        );
        assert!(parse_combo("hyper+s").is_err());
        assert!(parse_combo("ы").is_err());
        assert!(parse_combo("f13x").is_err());
    }

    #[test]
    fn the_main_modifier_follows_the_platform() {
        let win = parse_combo_for("ctrl+v", false).unwrap().modifiers;
        assert!(win.ctrl && win.command && !win.mac_cmd);
        let mac = parse_combo_for("ctrl+v", true).unwrap().modifiers;
        assert!(mac.mac_cmd && mac.command && !mac.ctrl);
        for name in ["ctrl", "control", "cmd", "command"] {
            for on_mac in PLATFORMS {
                assert_eq!(
                    parse_combo_for(&format!("{name}+v"), on_mac)
                        .unwrap()
                        .modifiers,
                    primary_modifiers(on_mac),
                    "{name} mac={on_mac}"
                );
            }
        }
    }

    #[test]
    fn on_a_mac_command_runs_the_shortcuts_and_ctrl_does_not() {
        for (combo, want) in [("cmd+o", Command::OpenFile), ("cmd+f", Command::Search)] {
            assert_eq!(run(true, Layout::En, combo, FREE), [Action::Run(want)]);
        }
        assert_eq!(
            actions(&press(true, Layout::Ru, "cmd+v", "x"), FREE),
            [Action::Paste("x".into())]
        );
        // A physical Ctrl on a Mac reaches the player as plain `ctrl`, without `command`.
        let ctrl_o = Event::Key {
            key: Key::O,
            physical_key: Some(Key::O),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::CTRL,
        };
        assert_eq!(actions(&[ctrl_o], FREE), []);
        // And Cmd on Windows/Linux is not a thing: no `command`, no shortcut.
        let cmd_o = Event::Key {
            key: Key::O,
            physical_key: Some(Key::O),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::MAC_CMD,
        };
        assert_eq!(actions(&[cmd_o], FREE), []);
    }

    #[test]
    fn mouse_side_buttons_trigger_seek_commands_ac2_ac3_ac4() {
        let extra1_press = Event::PointerButton {
            pos: pos2(100.0, 100.0),
            button: PointerButton::Extra1,
            pressed: true,
            modifiers: Modifiers::NONE,
        };
        let extra2_press = Event::PointerButton {
            pos: pos2(100.0, 100.0),
            button: PointerButton::Extra2,
            pressed: true,
            modifiers: Modifiers::NONE,
        };
        let extra1_release = Event::PointerButton {
            pos: pos2(100.0, 100.0),
            button: PointerButton::Extra1,
            pressed: false,
            modifiers: Modifiers::NONE,
        };

        // AC-2: Extra1 (Back) produces Command::Back when free
        assert_eq!(
            actions(&[extra1_press.clone()], FREE),
            [Action::Run(Command::Back)]
        );

        // AC-3: Extra2 (Forward) produces Command::Forward when free
        assert_eq!(
            actions(&[extra2_press.clone()], FREE),
            [Action::Run(Command::Forward)]
        );

        // Releases do not produce actions
        assert_eq!(actions(&[extra1_release], FREE), []);

        // AC-4: When typing or modal open, gate filters out the commands
        let typing_gate = Gate {
            typing: true,
            modal: false,
        };
        assert_eq!(actions(&[extra1_press.clone()], typing_gate), []);
        assert_eq!(actions(&[extra2_press.clone()], typing_gate), []);

        let modal_gate = Gate {
            typing: false,
            modal: true,
        };
        assert_eq!(actions(&[extra1_press], modal_gate), []);
        assert_eq!(actions(&[extra2_press], modal_gate), []);
    }
}
