//! The interface texts: one row per message, English first, then Russian and Ukrainian.
//! The Chinese column lives in `messages_zh.rs`.
//! `{0}`, `{1}`… mark values filled in at run time; a translation may reorder them.

macro_rules! messages {
    ($($key:ident => [$en:expr, $ru:expr, $uk:expr]),* $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum Tr {
            $($key),*
        }

        impl Tr {
            #[cfg(test)]
            pub const ALL: &'static [Tr] = &[$(Tr::$key),*];

            /// English, Russian, Ukrainian, Chinese.
            pub fn texts(self) -> [&'static str; 4] {
                let zh = super::messages_zh::text(self);
                match self {
                    $(Tr::$key => [$en, $ru, $uk, zh]),*
                }
            }
        }
    };
}

messages! {
    // Control bar.
    PlayHint => ["Play (Space)", "Воспроизвести (Пробел)", "Відтворити (Пробіл)"],
    PauseHint => ["Pause (Space)", "Пауза (Пробел)", "Пауза (Пробіл)"],
    PrevLineHint => ["Previous line (A)", "Предыдущая реплика (A)", "Попередня репліка (A)"],
    NextLineHint => ["Next line (D)", "Следующая реплика (D)", "Наступна репліка (D)"],
    RepeatLineHint => ["Repeat this line (S)", "Повторить реплику (S)", "Повторити репліку (S)"],
    FullscreenHint => ["Full screen (F)", "Во весь экран (F)", "На весь екран (F)"],
    SettingsHint => ["Settings", "Настройки", "Налаштування"],
    SidebarHint => [
        "Transcript and saved words (T)",
        "Транскрипт и сохранённые слова (T)",
        "Транскрипт і збережені слова (T)"
    ],
    MuteHint => ["Mute (M)", "Выключить звук (M)", "Вимкнути звук (M)"],
    AutoPause => ["Auto-pause", "Автопауза", "Автопауза"],
    AutoPauseHint => [
        "Pause after every line (P)",
        "Пауза после каждой реплики (P)",
        "Пауза після кожної репліки (P)"
    ],
    AutoPauseOn => [
        "Auto-pause after each line: on",
        "Автопауза после каждой реплики: включена",
        "Автопауза після кожної репліки: увімкнена"
    ],
    AutoPauseOff => ["Auto-pause: off", "Автопауза: выключена", "Автопауза: вимкнена"],
    ReadingSubtitles => ["reading subtitles…", "чтение субтитров…", "читання субтитрів…"],

    // Empty screen.
    Tagline => [
        "Watch in the language you are learning",
        "Смотрите на языке, который изучаете",
        "Дивіться мовою, яку вивчаєте"
    ],
    OpenVideo => ["Open a video…", "Открыть видео…", "Відкрити відео…"],
    DropHint => [
        "or drop a file anywhere in this window, or press Ctrl+V with a copied link or path",
        "или перетащите файл в это окно, или нажмите Ctrl+V со скопированной ссылкой или путём",
        "або перетягніть файл у це вікно, або натисніть Ctrl+V зі скопійованим посиланням чи шляхом"
    ],
    PasteLink => ["Paste a video link", "Вставьте ссылку на видео", "Вставте посилання на відео"],
    PlayButton => ["Play", "Смотреть", "Дивитися"],
    ShortcutsHelp => [
        "Space pause · A / S / D previous, repeat, next line · P auto-pause\nT transcript · F full screen · click a word to look it up",
        "Пробел — пауза · A / S / D — предыдущая, повтор, следующая реплика · P — автопауза\nT — транскрипт · F — во весь экран · щёлкните слово, чтобы посмотреть перевод",
        "Пробіл — пауза · A / S / D — попередня, повтор, наступна репліка · P — автопауза\nT — транскрипт · F — на весь екран · клацніть слово, щоб побачити переклад"
    ],
    SettingsLink => ["Settings", "Настройки", "Налаштування"],

    // Subtitles menu.
    SubtitlesMenu => ["Subtitles", "Субтитры", "Субтитри"],
    Learning => ["I am learning", "Я изучаю", "Я вивчаю"],
    StateReading => ["reading…", "чтение…", "читання…"],
    StateAsYouWatch => ["as you watch", "по мере просмотра", "під час перегляду"],
    StateFailed => ["failed", "ошибка", "помилка"],
    StateImage => ["image", "картинка", "зображення"],
    NoneChoice => ["None", "Нет", "Немає"],
    TranslationLabel => ["Translation", "Перевод", "Переклад"],
    TranslateAuto => [
        "Translate automatically ({0})",
        "Переводить автоматически ({0})",
        "Перекладати автоматично ({0})"
    ],
    ProviderNotSetUp => ["not set up", "не настроен", "не налаштовано"],
    OffChoice => ["Off", "Выключен", "Вимкнено"],
    StopTranscribing => [
        "Stop transcribing speech",
        "Остановить распознавание речи",
        "Зупинити розпізнавання мовлення"
    ],
    TranscribeSpeech => [
        "Transcribe speech (Whisper)",
        "Распознать речь (Whisper)",
        "Розпізнати мовлення (Whisper)"
    ],
    AddSubFile => ["Add a subtitle file…", "Добавить файл субтитров…", "Додати файл субтитрів…"],
    SaveTranscriptSrt => [
        "Save transcript as .srt…",
        "Сохранить транскрипт как .srt…",
        "Зберегти транскрипт як .srt…"
    ],
    SaveTranslationSrt => [
        "Save translation as .srt…",
        "Сохранить перевод как .srt…",
        "Зберегти переклад як .srt…"
    ],
    AudioLabel => ["Audio", "Звук", "Звук"],
    AutoPauseMenu => [
        "Auto-pause after each line (P)",
        "Автопауза после каждой реплики (P)",
        "Автопауза після кожної репліки (P)"
    ],
    SettingsMenu => ["Settings…", "Настройки…", "Налаштування…"],
    SubStyleLabel => ["Style", "Стиль", "Стиль"],
    SubStylePlate => ["Plate", "Плашка", "Плашка"],
    SubStyleOutline => ["Outline", "Контур", "Контур"],
    SubStyleBold => ["Large and bold", "Крупный жирный", "Великий жирний"],
    SubStyleYellow => ["Classic yellow", "Жёлтый классический", "Жовтий класичний"],
    SubStyleLight => ["Light plate", "Светлая плашка", "Світла плашка"],
    SubSizeLabel => ["Size", "Размер", "Розмір"],
    SubSizeMenu => [
        "Size {0}% (- / =)",
        "Размер {0}% (- / =)",
        "Розмір {0}% (- / =)"
    ],
    SubPositionLabel => ["Position", "Положение", "Положення"],
    SubPosBottom => ["Bottom", "Снизу", "Знизу"],
    SubPosTop => ["Top", "Сверху", "Зверху"],
    SubLayoutLabel => ["Layout", "Расположение", "Розташування"],
    SubLayoutRows => ["Stacked (rows)", "Друг под другом (строки)", "Один під одним (рядки)"],
    SubLayoutColumns => ["Side-by-side (columns)", "Рядом в строку (колонки)", "Поруч у рядок (колонки)"],
    SubMarginLabel => ["Distance from the edge", "Отступ от края", "Відступ від краю"],
    SubPreviewLabel => ["Preview", "Предпросмотр", "Попередній перегляд"],
    SubSampleOriginal => [
        "Learn a language by watching",
        "Learn a language by watching",
        "Learn a language by watching"
    ],
    SubSampleTranslation => [
        "Learn while you watch",
        "Учитесь, пока смотрите",
        "Вчіться, поки дивитеся"
    ],
    SubSizeToast => ["Subtitle size {0}%", "Размер субтитров {0}%", "Розмір субтитрів {0}%"],
    VolumeToast => ["Volume {0}%", "Громкость {0}%", "Гучність {0}%"],
    ThemeRow => ["Theme", "Тема", "Тема"],
    ThemeDark => ["Dark", "Тёмная", "Темна"],
    ThemeLight => ["Light", "Светлая", "Світла"],
    ThemeSystem => ["Like the system", "Как в системе", "Як у системі"],
    CouldNotSaveSettings => [
        "Could not save settings: {0}",
        "Не удалось сохранить настройки: {0}",
        "Не вдалося зберегти налаштування: {0}"
    ],
    SavedLines => [
        "Saved {0} lines to {1}",
        "Сохранено реплик: {0}, файл: {1}",
        "Збережено реплік: {0}, файл: {1}"
    ],
    CouldNotSaveFile => [
        "Could not save {0}: {1}",
        "Не удалось сохранить {0}: {1}",
        "Не вдалося зберегти {0}: {1}"
    ],

    // Sidebar.
    TabTranscript => ["Transcript", "Транскрипт", "Транскрипт"],
    TabWords => ["Words ({0})", "Слова ({0})", "Слова ({0})"],
    CloseHint => ["Close (T)", "Закрыть (T)", "Закрити (T)"],
    SearchHint => ["🔍 Search", "🔍 Поиск", "🔍 Пошук"],
    OpenToSeeTranscript => [
        "Open a video to see its transcript.",
        "Откройте видео, чтобы увидеть его транскрипт.",
        "Відкрийте відео, щоб побачити його транскрипт."
    ],
    NoTextSubtitles => [
        "No text subtitles yet.",
        "Текстовых субтитров пока нет.",
        "Текстових субтитрів поки немає."
    ],
    WordsEmpty => [
        "Click a word in the subtitles and press Save — it will show up here.",
        "Щёлкните слово в субтитрах и нажмите «Сохранить» — оно появится здесь.",
        "Клацніть слово в субтитрах і натисніть «Зберегти» — воно з’явиться тут."
    ],
    ShowWord => ["▶ Show", "▶ Показать", "▶ Показати"],
    JumpHint => ["Jump to this moment", "Перейти к этому моменту", "Перейти до цього моменту"],
    InAnki => ["✔ In Anki", "✔ В Anki", "✔ В Anki"],
    SendToAnki => ["Send to Anki", "Отправить в Anki", "Надіслати в Anki"],
    InCustomDict => ["✔ In dictionary", "✔ В словаре", "✔ У словнику"],
    SendToCustomDict => ["Send to dictionary", "Отправить в словарь", "Надіслати в словник"],
    CustomDictAddressRow => ["Dictionary API address", "Адрес API словаря", "Адреса API словника"],
    CustomDictTokenRow => ["API Token (optional)", "Токен API (опционально)", "Токен API (опціонально)"],
    CustomDictStatusRow => ["Dictionary connection", "Связь со словарём", "Зв’язок зі словником"],
    CustomDictCheckButton => ["Check", "Проверить", "Перевірити"],
    CustomDictChecking => ["Checking…", "Проверяем…", "Перевіряємо…"],
    CustomDictConnected => ["Connected (HTTP {0})", "Подключено (HTTP {0})", "Підключено (HTTP {0})"],
    CustomDictStatusError => ["Error: {0}", "Ошибка: {0}", "Помилка: {0}"],
    CustomDictNotConfigured => [
        "Dictionary API address is not configured (Settings → Words)",
        "Адрес API словаря не настроен (Настройки → Слова)",
        "Адреса API словника не налаштована (Налаштування → Слова)"
    ],
    CustomDictSending => ["Sending to dictionary…", "Отправляем в словарь…", "Надсилаємо в словник…"],
    CustomDictAdded => ["Added to dictionary", "Добавлено в словарь", "Додано в словник"],
    CustomDictError => ["Dictionary: {0}", "Словарь: {0}", "Словник: {0}"],
    CustomDictErrorNetwork => [
        "Dictionary unreachable: {0}",
        "Словарь недоступен: {0}",
        "Словник недоступний: {0}"
    ],
    CustomDictErrorAuth => [
        "Dictionary: authorization error (401)",
        "Словарь: ошибка авторизации (401)",
        "Словник: помилка авторизації (401)"
    ],
    CustomDictErrorServer => [
        "Dictionary: server error ({0})",
        "Словарь: ошибка сервера ({0})",
        "Словник: помилка сервера ({0})"
    ],
    SectionCustomDict => ["Custom dictionary", "Пользовательский словарь", "Користувацький словник"],
    DeleteWord => ["Delete", "Удалить", "Видалити"],

    // Word card.
    CloseEscHint => ["Close (Esc)", "Закрыть (Esc)", "Закрити (Esc)"],
    Explaining => [
        "Explaining in context…",
        "Объясняем в контексте…",
        "Пояснюємо в контексті…"
    ],
    WordSaved => ["★ Saved", "★ Сохранено", "★ Збережено"],
    SaveWord => ["☆ Save word", "☆ Сохранить слово", "☆ Зберегти слово"],
    CopyWord => ["Copy", "Копировать", "Копіювати"],

    // Settings window.
    SettingsTitle => ["Settings", "Настройки", "Налаштування"],
    SectionLanguages => ["Languages", "Языки", "Мови"],
    InterfaceLanguage => ["Interface language", "Язык интерфейса", "Мова інтерфейсу"],
    LangAuto => ["Automatic", "Автоматически", "Автоматично"],
    SectionSubtitles => ["Subtitles", "Субтитры", "Субтитри"],
    TextSize => ["Text size", "Размер текста", "Розмір тексту"],
    TranslationSize => ["Translation size", "Размер перевода", "Розмір перекладу"],
    AutoTranslateCheck => [
        "Translate when there is no track in my language",
        "Переводить, если нет дорожки на моём языке",
        "Перекладати, якщо немає доріжки моєю мовою"
    ],
    SectionTranslation => ["Translation", "Перевод", "Переклад"],
    ServiceRow => ["Service", "Сервис", "Сервіс"],
    ModelRow => ["Model", "Модель", "Модель"],
    ModelNameHint => ["model name", "название модели", "назва моделі"],
    ServerAddressRow => ["Server address", "Адрес сервера", "Адреса сервера"],
    OpenAiAddressHint => [
        "empty = OpenAI; Ollama: http://localhost:11434/v1",
        "пусто = OpenAI; Ollama: http://localhost:11434/v1",
        "порожньо = OpenAI; Ollama: http://localhost:11434/v1"
    ],
    ApiKeyRow => ["API key", "Ключ API", "Ключ API"],
    ProviderGoogle => [
        "Google Translate (no key needed)",
        "Google Translate (ключ не нужен)",
        "Google Translate (ключ не потрібен)"
    ],
    ProviderOpenAi => [
        "OpenAI-compatible (OpenAI, Ollama, LM Studio)",
        "Совместимый с OpenAI (OpenAI, Ollama, LM Studio)",
        "Сумісний з OpenAI (OpenAI, Ollama, LM Studio)"
    ],
    ProviderUnknown => ["Unknown", "Неизвестно", "Невідомо"],
    SectionSpeechRecognition => [
        "Speech recognition",
        "Распознавание речи",
        "Розпізнавання мовлення"
    ],
    ModelFileRow => ["Model file", "Файл модели", "Файл моделі"],
    AsrAutoCheck => [
        "Transcribe automatically when a video has no subtitles",
        "Распознавать речь автоматически, если у видео нет субтитров",
        "Розпізнавати мовлення автоматично, якщо у відео немає субтитрів"
    ],
    DeckRow => ["Deck", "Колода", "Колода"],
    AnkiAddressRow => ["AnkiConnect address", "Адрес AnkiConnect", "Адреса AnkiConnect"],
    AnkiStatusRow => ["Connection to Anki", "Связь с Anki", "Зв’язок з Anki"],
    AnkiCheckButton => ["Check", "Проверить", "Перевірити"],
    AnkiChecking => ["Checking…", "Проверяем…", "Перевіряємо…"],
    AnkiConnected => [
        "Connected: Anki answers (AnkiConnect API version {0}).",
        "Подключено: Anki отвечает (версия API AnkiConnect — {0}).",
        "Підключено: Anki відповідає (версія API AnkiConnect — {0})."
    ],
    AnkiStatusNotRunning => [
        "Anki is not running. Start Anki and keep it open while you send words.",
        "Anki не запущена. Запустите Anki и оставьте её открытой, пока отправляете слова.",
        "Anki не запущена. Запустіть Anki й залишайте її відкритою, поки надсилаєте слова."
    ],
    AnkiStatusNoAddon => [
        "The AnkiConnect add-on was not found. Install it (see \"How to connect Anki\" below), then start Anki.",
        "Дополнение AnkiConnect не найдено. Установите его (см. «Как подключить Anki» ниже) и запустите Anki.",
        "Доповнення AnkiConnect не знайдено. Встановіть його (див. «Як підключити Anki» нижче) і запустіть Anki."
    ],
    AnkiStatusNoAnswer => [
        "Nothing answers at this address. Start Anki with the AnkiConnect add-on (see \"How to connect Anki\" below).",
        "По этому адресу никто не отвечает. Запустите Anki с дополнением AnkiConnect (см. «Как подключить Anki» ниже).",
        "За цією адресою ніхто не відповідає. Запустіть Anki з доповненням AnkiConnect (див. «Як підключити Anki» нижче)."
    ],
    AnkiStatusError => [
        "The address answers, but not like AnkiConnect: {0}",
        "Адрес отвечает, но не как AnkiConnect: {0}",
        "Адреса відповідає, але не як AnkiConnect: {0}"
    ],
    AnkiHowTo => ["How to connect Anki", "Как подключить Anki", "Як підключити Anki"],
    AnkiStep1 => [
        "1. Install Anki, a free flashcard program, and start it:",
        "1. Установите Anki — бесплатную программу с карточками — и запустите её:",
        "1. Встановіть Anki — безкоштовну програму з картками — і запустіть її:"
    ],
    AnkiStep2 => [
        "2. In Anki choose Tools, Add-ons, Get Add-ons, enter the code {0} and press OK.",
        "2. В Anki выберите «Инструменты» → «Дополнения» → «Получить дополнения», введите код {0} и нажмите OK.",
        "2. В Anki виберіть «Інструменти» → «Доповнення» → «Отримати доповнення», введіть код {0} і натисніть OK."
    ],
    AnkiStep3 => [
        "3. Restart Anki and keep it open while you send words from the player.",
        "3. Перезапустите Anki и оставьте её открытой, пока отправляете слова из плеера.",
        "3. Перезапустіть Anki й залишайте її відкритою, поки надсилаєте слова з плеєра."
    ],
    AnkiCopyCode => ["Copy the code", "Скопировать код", "Скопіювати код"],
    AnkiCodeCopied => ["Copied", "Скопировано", "Скопійовано"],
    SectionPlayer => ["Player", "Плеер", "Плеєр"],
    HwdecRow => ["Hardware decoding", "Аппаратное декодирование", "Апаратне декодування"],
    InterfaceScaleRow => ["Interface scale", "Масштаб интерфейса", "Масштаб інтерфейсу"],
    ShellCheck => [
        "Show Glossplay in Explorer's \"Open with\" and context menu",
        "Показывать Glossplay в меню «Открыть с помощью» и в контекстном меню Проводника",
        "Показувати Glossplay в меню «Відкрити за допомогою» та в контекстному меню Провідника"
    ],
    SaveButton => ["Save", "Сохранить", "Зберегти"],
    PathAutoHint => [
        "found automatically if empty",
        "ищется автоматически",
        "шукається автоматично"
    ],
    BrowseButton => ["Browse", "Обзор", "Огляд"],

    // Messages from the app.
    NoFfmpegToast => [
        "ffmpeg was not found — subtitles are shown without word lookup. It is included in the Windows package; otherwise install ffmpeg or set its path in Settings.",
        "ffmpeg не найден — субтитры показываются без поиска слов. В пакете для Windows он есть; иначе установите ffmpeg или укажите путь к нему в настройках.",
        "ffmpeg не знайдено — субтитри показуються без пошуку слів. У пакеті для Windows він є; інакше встановіть ffmpeg або вкажіть шлях до нього в налаштуваннях."
    ],
    ConfigUsingDefaults => [
        "{0}; using default settings",
        "{0}; используются настройки по умолчанию",
        "{0}; використовуються налаштування за замовчуванням"
    ],
    DownloadingYtdlp => [
        "Downloading yt-dlp, the video will start in a moment…",
        "Загружаем yt-dlp, видео начнётся через мгновение…",
        "Завантажуємо yt-dlp, відео почнеться за мить…"
    ],
    FfmpegRequired => [
        "ffmpeg is required for transcription",
        "Для распознавания речи нужен ffmpeg",
        "Для розпізнавання мовлення потрібен ffmpeg"
    ],
    StreamSoundUnreadable => [
        "Speech recognition cannot read the sound of this link",
        "Распознавание речи не может прочитать звук по этой ссылке",
        "Розпізнавання мовлення не може прочитати звук за цим посиланням"
    ],
    NoDurationYet => [
        "The file has no known duration yet — try again in a moment",
        "Длительность файла пока неизвестна — попробуйте через мгновение",
        "Тривалість файлу поки невідома — спробуйте за мить"
    ],
    AsrTrackLabel => [
        "Speech recognition",
        "Распознавание речи",
        "Розпізнавання мовлення"
    ],
    AutoSubsLabel => ["{0} (auto)", "{0} (авто)", "{0} (авто)"],
    AutoTranslatedSubsLabel => [
        "{0} (auto-translated)",
        "{0} (автоперевод)",
        "{0} (автопереклад)"
    ],
    LangEnglish => ["English", "Английский", "Англійська"],
    LangRussian => ["Russian", "Русский", "Російська"],
    LangUkrainian => ["Ukrainian", "Украинский", "Українська"],
    LangGerman => ["German", "Немецкий", "Німецька"],
    LangFrench => ["French", "Французский", "Французька"],
    LangSpanish => ["Spanish", "Испанский", "Іспанська"],
    LangItalian => ["Italian", "Итальянский", "Італійська"],
    LangPortuguese => ["Portuguese", "Португальский", "Португальська"],
    LangPolish => ["Polish", "Польский", "Польська"],
    LangTurkish => ["Turkish", "Турецкий", "Турецька"],
    LangJapanese => ["Japanese", "Японский", "Японська"],
    LangKorean => ["Korean", "Корейский", "Корейська"],
    LangChinese => ["Chinese", "Китайский", "Китайська"],
    LangDutch => ["Dutch", "Нидерландский", "Нідерландська"],
    LangArabic => ["Arabic", "Арабский", "Арабська"],
    LangHindi => ["Hindi", "Хинди", "Гінді"],
    LangIndonesian => ["Indonesian", "Индонезийский", "Індонезійська"],
    LangVietnamese => ["Vietnamese", "Вьетнамский", "В’єтнамська"],
    LangCzech => ["Czech", "Чешский", "Чеська"],
    LangSwedish => ["Swedish", "Шведский", "Шведська"],
    CouldNotReadSubsShown => [
        "Could not read the subtitles \"{0}\": {1} — they are shown without word lookup.",
        "Не удалось прочитать субтитры «{0}»: {1} — они показываются без поиска слов.",
        "Не вдалося прочитати субтитри «{0}»: {1} — вони показуються без пошуку слів."
    ],
    CouldNotLoadSubs => [
        "Could not load the subtitles \"{0}\": {1}",
        "Не удалось загрузить субтитры «{0}»: {1}",
        "Не вдалося завантажити субтитри «{0}»: {1}"
    ],
    TranslationFailed => [
        "Translation failed: {0}",
        "Не удалось перевести: {0}",
        "Не вдалося перекласти: {0}"
    ],
    YtdlpUpdated => ["yt-dlp updated to {0}", "yt-dlp обновлён до {0}", "yt-dlp оновлено до {0}"],
    AddedToAnki => ["Added to Anki", "Добавлено в Anki", "Додано в Anki"],
    AnkiError => ["Anki: {0}", "Anki: {0}", "Anki: {0}"],
    TranslationNotSetUp => [
        "Translation is not set up: {0} (Settings → Translation)",
        "Перевод не настроен: {0} (Настройки → Перевод)",
        "Переклад не налаштовано: {0} (Налаштування → Переклад)"
    ],
    SendingToAnki => ["Sending to Anki…", "Отправляем в Anki…", "Надсилаємо в Anki…"],
    ExplorerIntegration => [
        "Explorer integration: {0}",
        "Интеграция с Проводником: {0}",
        "Інтеграція з Провідником: {0}"
    ],
    OldConfigKept => [
        "The old config.toml did not parse; I kept it as {0}",
        "Старый config.toml не удалось прочитать; я сохранил его как {0}",
        "Старий config.toml не вдалося прочитати; я зберіг його як {0}"
    ],
    CouldNotKeepConfig => [
        "Could not keep a copy of the old config.toml: {0}",
        "Не удалось сохранить копию старого config.toml: {0}",
        "Не вдалося зберегти копію старого config.toml: {0}"
    ],
    OpenDialogTitle => [
        "Open video or subtitles",
        "Открыть видео или субтитры",
        "Відкрити відео або субтитри"
    ],
    CouldNotPlay => [
        "Could not play this file",
        "Не удалось воспроизвести этот файл",
        "Не вдалося відтворити цей файл"
    ],

    // Messages from background jobs.
    FfmpegNotFound => ["ffmpeg was not found", "ffmpeg не найден", "ffmpeg не знайдено"],
    AnkiNotRunning => [
        "Anki does not answer: start it with the AnkiConnect add-on (Settings, \"Words and Anki\", \"How to connect Anki\")",
        "Anki не отвечает: запустите её с дополнением AnkiConnect (Настройки → «Слова и Anki» → «Как подключить Anki»)",
        "Anki не відповідає: запустіть її з доповненням AnkiConnect (Налаштування → «Слова та Anki» → «Як підключити Anki»)"
    ],
    AlreadyInDeck => [
        "this word is already in the deck",
        "это слово уже есть в колоде",
        "це слово вже є в колоді"
    ],
    AsrStopped => [
        "Speech recognition stopped: {0}",
        "Распознавание речи остановлено: {0}",
        "Розпізнавання мовлення зупинено: {0}"
    ],
    AsrStarting => [
        "Starting speech recognition…",
        "Запускаем распознавание речи…",
        "Запускаємо розпізнавання мовлення…"
    ],
    CannotStartWhisper => [
        "cannot start whisper-server: {0}",
        "не удалось запустить whisper-server: {0}",
        "не вдалося запустити whisper-server: {0}"
    ],
    WhisperQuitLoading => [
        "whisper-server quit while loading; check the model path",
        "whisper-server завершился во время загрузки; проверьте путь к модели",
        "whisper-server завершився під час завантаження; перевірте шлях до моделі"
    ],
    WhisperNotReady => [
        "whisper-server did not become ready; check the model path",
        "whisper-server так и не запустился; проверьте путь к модели",
        "whisper-server так і не запустився; перевірте шлях до моделі"
    ],
    TranscriptReady => ["Transcript ready", "Транскрипт готов", "Транскрипт готовий"],
    Transcribing => [
        "Transcribing {0}%{1}",
        "Распознаём речь: {0}%{1}",
        "Розпізнаємо мовлення: {0}%{1}"
    ],
    CouldNotReadAudio => [
        "could not read the audio: {0}",
        "не удалось прочитать звук: {0}",
        "не вдалося прочитати звук: {0}"
    ],
    WhisperStopped => [
        "whisper-server stopped answering",
        "whisper-server перестал отвечать",
        "whisper-server перестав відповідати"
    ],
    WhisperFailed => [
        "whisper-server failed: {0}",
        "сбой whisper-server: {0}",
        "збій whisper-server: {0}"
    ],
    WhisperQuitTimes => [
        "whisper-server quit unexpectedly {0} times",
        "whisper-server неожиданно завершался {0} раз",
        "whisper-server несподівано завершувався {0} разів"
    ],
    YtdlpNoGithub => [
        "Could not reach GitHub to download yt-dlp: {0}",
        "Не удалось подключиться к GitHub, чтобы скачать yt-dlp: {0}",
        "Не вдалося підключитися до GitHub, щоб завантажити yt-dlp: {0}"
    ],
    YtdlpDownloadFailed => [
        "Could not download yt-dlp: {0}",
        "Не удалось скачать yt-dlp: {0}",
        "Не вдалося завантажити yt-dlp: {0}"
    ],
    YtdlpBadChecksum => [
        "The downloaded yt-dlp does not match its checksum; not installing it",
        "Скачанный yt-dlp не совпадает с контрольной суммой; устанавливать его не буду",
        "Завантажений yt-dlp не збігається з контрольною сумою; встановлювати його не буду"
    ],
    YtdlpChecksumsFailed => [
        "Could not download the yt-dlp checksums: {0}",
        "Не удалось скачать контрольные суммы yt-dlp: {0}",
        "Не вдалося завантажити контрольні суми yt-dlp: {0}"
    ],
    YtdlpNoChecksum => [
        "The checksum list has no entry for {0}",
        "В списке контрольных сумм нет записи для {0}",
        "У списку контрольних сум немає запису для {0}"
    ],

    // Startup error window.
    StartupFailed => [
        "Glossplay could not start",
        "Не удалось запустить Glossplay",
        "Не вдалося запустити Glossplay"
    ],
    StartupMpvHint => [
        "Put the mpv library (libmpv) next to the program, or set player.mpv_path in config.toml:",
        "Положите библиотеку mpv (libmpv) рядом с программой или укажите player.mpv_path в config.toml:",
        "Покладіть бібліотеку mpv (libmpv) поруч із програмою або вкажіть player.mpv_path у config.toml:"
    ],

    // Explorer menu entry.
    ShellPlayWith => [
        "Play with Glossplay",
        "Воспроизвести в Glossplay",
        "Відтворити в Glossplay"
    ],

    // Installer errors and units.
    UnitMb => ["{0} MB", "{0} МБ", "{0} МБ"],
    UnitGb => ["{0} GB", "{0} ГБ", "{0} ГБ"],
    PerSecond => ["{0}/s", "{0}/с", "{0}/с"],
    Cancelled => [
        "Cancelled. What was downloaded is kept and the next install continues from there.",
        "Отменено. Скачанное сохранено, и следующая установка продолжится с этого места.",
        "Скасовано. Завантажене збережено, і наступне встановлення продовжиться з цього місця."
    ],
    NotInstalled => [
        "This component is not installed",
        "Этот компонент не установлен",
        "Цей компонент не встановлено"
    ],
    AlreadyInstalling => [
        "This component is already being installed",
        "Этот компонент уже устанавливается",
        "Цей компонент уже встановлюється"
    ],
    UnsafePath => [
        "The archive contains a file that would land outside the install folder; not installing it",
        "В архиве есть файл, который оказался бы за пределами папки установки; устанавливать не буду",
        "В архіві є файл, який опинився б поза папкою встановлення; встановлювати не буду"
    ],
    DiskFull => [
        "The disk is full: could not write {0}",
        "Диск заполнен: не удалось записать {0}",
        "Диск заповнено: не вдалося записати {0}"
    ],
    CannotWrite => [
        "Could not write {0}: {1}",
        "Не удалось записать {0}: {1}",
        "Не вдалося записати {0}: {1}"
    ],
    CannotRead => [
        "Could not read {0}: {1}",
        "Не удалось прочитать {0}: {1}",
        "Не вдалося прочитати {0}: {1}"
    ],
    CannotRemove => [
        "Could not remove the component: {0}. Close Glossplay and any whisper-server and try again.",
        "Не удалось удалить компонент: {0}. Закройте Glossplay и все whisper-server и повторите попытку.",
        "Не вдалося видалити компонент: {0}. Закрийте Glossplay і всі whisper-server та повторіть спробу."
    ],
    InUse => [
        "Could not replace {0}: {1}. Another Glossplay window may be using it; close it and try again.",
        "Не удалось заменить {0}: {1}. Возможно, файл использует другое окно Glossplay; закройте его и повторите попытку.",
        "Не вдалося замінити {0}: {1}. Можливо, файл використовує інше вікно Glossplay; закрийте його та повторіть спробу."
    ],
    NotEnoughSpace => [
        "Not enough disk space: the install needs {0} free on the drive of {1}, and there is {2}.",
        "Недостаточно места на диске: для установки нужно {0} свободного места на диске, где находится {1}, а доступно {2}.",
        "Недостатньо місця на диску: для встановлення потрібно {0} вільного місця на диску, де розташовано {1}, а доступно {2}."
    ],
    DownloadFailed => [
        "Could not download the {0}: {1}. Try again later; the download will continue where it stopped.",
        "Не удалось скачать «{0}»: {1}. Повторите позже; загрузка продолжится с того места, где остановилась.",
        "Не вдалося завантажити «{0}»: {1}. Повторіть пізніше; завантаження продовжиться з того місця, де зупинилося."
    ],
    DamagedDownload => [
        "The downloaded {0} is damaged (its checksum does not match), so it was deleted. Try again.",
        "Скачанный файл «{0}» повреждён (контрольная сумма не совпадает), поэтому он удалён. Попробуйте ещё раз.",
        "Завантажений файл «{0}» пошкоджено (контрольна сума не збігається), тому його видалено. Спробуйте ще раз."
    ],
    BadArchive => [
        "Could not unpack {0}: {1}",
        "Не удалось распаковать {0}: {1}",
        "Не вдалося розпакувати {0}: {1}"
    ],
    ArchiveEmpty => [
        "{0} has none of the expected files",
        "В {0} нет ни одного из нужных файлов",
        "У {0} немає жодного з потрібних файлів"
    ],
    ArchiveLacks => [
        "{0} lacks {1}",
        "В {0} не хватает: {1}",
        "У {0} бракує: {1}"
    ],

    // Speech recognition setup.
    ComponentName => [
        "Speech recognition (Whisper)",
        "Распознавание речи (Whisper)",
        "Розпізнавання мовлення (Whisper)"
    ],
    LicenceNote => [
        "whisper.cpp and the Whisper models are MIT-licensed (ggml-org, OpenAI); the Silero voice detector is MIT-licensed. The NVIDIA build includes NVIDIA CUDA libraries, redistributed under the NVIDIA CUDA licence.",
        "whisper.cpp и модели Whisper распространяются по лицензии MIT (ggml-org, OpenAI); детектор речи Silero — тоже по лицензии MIT. Сборка для NVIDIA включает библиотеки NVIDIA CUDA, которые распространяются по лицензии NVIDIA CUDA.",
        "whisper.cpp і моделі Whisper поширюються за ліцензією MIT (ggml-org, OpenAI); детектор мовлення Silero — теж за ліцензією MIT. Збірка для NVIDIA містить бібліотеки NVIDIA CUDA, які поширюються за ліцензією NVIDIA CUDA."
    ],
    EngineLabelNvidia => [
        "whisper.cpp engine for NVIDIA graphics cards",
        "Движок whisper.cpp для видеокарт NVIDIA",
        "Рушій whisper.cpp для відеокарт NVIDIA"
    ],
    EngineLabelCpu => [
        "whisper.cpp engine for the processor",
        "Движок whisper.cpp для процессора",
        "Рушій whisper.cpp для процесора"
    ],
    VadLabel => ["Silero voice detector", "Детектор речи Silero", "Детектор мовлення Silero"],
    ModelLabel => ["Whisper model {0}", "Модель Whisper {0}", "Модель Whisper {0}"],
    EngineAuto => ["Automatic", "Автоматически", "Автоматично"],
    EngineNvidia => ["NVIDIA graphics card", "Видеокарта NVIDIA", "Відеокарта NVIDIA"],
    EngineCpu => [
        "Processor (no graphics card)",
        "Процессор (без видеокарты)",
        "Процесор (без відеокарти)"
    ],
    EngineAutoIs => ["{0} ({1})", "{0} ({1})", "{0} ({1})"],
    BuildNvidia => ["NVIDIA build", "сборка для NVIDIA", "збірка для NVIDIA"],
    BuildCpu => ["processor build", "сборка для процессора", "збірка для процесора"],
    ModelAuto => ["Recommended", "Рекомендуемая", "Рекомендована"],
    ModelHintTurbo => ["most accurate", "самая точная", "найточніша"],
    ModelHintTurboQ5 => ["accurate, smaller", "точная, меньше размером", "точна, менша за розміром"],
    ModelHintSmall => [
        "good, works on any computer",
        "хорошая, работает на любом компьютере",
        "добра, працює на будь-якому комп’ютері"
    ],
    ModelHintBase => [
        "fastest, makes more mistakes",
        "самая быстрая, чаще ошибается",
        "найшвидша, частіше помиляється"
    ],
    DialogTitle => [
        "Speech recognition",
        "Распознавание речи",
        "Розпізнавання мовлення"
    ],
    DialogIntro => [
        "Glossplay can turn speech into subtitles for videos that have none. It runs Whisper on this computer, so nothing is sent anywhere. It needs a one-time download.",
        "Glossplay может превращать речь в субтитры для видео, у которых их нет. Whisper работает на этом компьютере, поэтому ничего никуда не отправляется. Нужна однократная загрузка.",
        "Glossplay може перетворювати мовлення на субтитри для відео, у яких їх немає. Whisper працює на цьому комп’ютері, тому нічого нікуди не надсилається. Потрібне одноразове завантаження."
    ],
    CheckingHardware => [
        "Checking the graphics card…",
        "Проверяем видеокарту…",
        "Перевіряємо відеокарту…"
    ],
    NoNvidia => [
        "No NVIDIA graphics card found",
        "Видеокарта NVIDIA не найдена",
        "Відеокарту NVIDIA не знайдено"
    ],
    NvidiaForcedWithoutGpu => [
        "No NVIDIA graphics card was found; the NVIDIA build will not start without one.",
        "Видеокарта NVIDIA не найдена; сборка для NVIDIA без неё не запустится.",
        "Відеокарту NVIDIA не знайдено; збірка для NVIDIA без неї не запуститься."
    ],
    CpuSlow => [
        "Without an NVIDIA graphics card recognition is slower: about as fast as the video plays. Give it a head start before you watch.",
        "Без видеокарты NVIDIA распознавание идёт медленнее: примерно со скоростью воспроизведения видео. Запустите его заранее, до просмотра.",
        "Без відеокарти NVIDIA розпізнавання йде повільніше: приблизно зі швидкістю відтворення відео. Запустіть його заздалегідь, до перегляду."
    ],
    OldDriver => [
        "The NVIDIA driver is too old for the NVIDIA build (version 525 or newer is needed), so the processor build is suggested. Update the driver to use the graphics card.",
        "Драйвер NVIDIA слишком старый для сборки под NVIDIA (нужна версия 525 или новее), поэтому предлагается сборка для процессора. Обновите драйвер, чтобы использовать видеокарту.",
        "Драйвер NVIDIA надто старий для збірки під NVIDIA (потрібна версія 525 або новіша), тому пропонується збірка для процесора. Оновіть драйвер, щоб використовувати відеокарту."
    ],
    NoEngineHere => [
        "There is no ready-made whisper.cpp for this system: install whisper.cpp yourself and set the path to whisper-server under Advanced. The model is downloaded here all the same.",
        "Для этой системы нет готовой сборки whisper.cpp: установите whisper.cpp сами и укажите путь к whisper-server в разделе «Дополнительно». Модель при этом загружается здесь же.",
        "Для цієї системи немає готової збірки whisper.cpp: встановіть whisper.cpp самостійно та вкажіть шлях до whisper-server у розділі «Додатково». Модель при цьому завантажується тут же."
    ],
    VcRuntimeMissing => [
        "whisper.cpp needs the Microsoft Visual C++ runtime, which is not installed. Get it from https://aka.ms/vs/17/release/vc_redist.x64.exe",
        "Для whisper.cpp нужна среда выполнения Microsoft Visual C++, а она не установлена. Скачайте её: https://aka.ms/vs/17/release/vc_redist.x64.exe",
        "Для whisper.cpp потрібне середовище виконання Microsoft Visual C++, а воно не встановлене. Завантажте його: https://aka.ms/vs/17/release/vc_redist.x64.exe"
    ],
    RowEngine => ["Engine", "Движок", "Рушій"],
    RowModel => ["Model", "Модель", "Модель"],
    RowTotal => ["Download", "Загрузка", "Завантаження"],
    RowPlace => ["Installs to", "Папка установки", "Папка встановлення"],
    EngineOwn => ["your own whisper.cpp", "свой whisper.cpp", "власний whisper.cpp"],
    Install => ["Install", "Установить", "Встановити"],
    Reinstall => ["Reinstall", "Переустановить", "Перевстановити"],
    Remove => ["Remove", "Удалить", "Видалити"],
    Cancel => ["Cancel", "Отмена", "Скасувати"],
    Close => ["Close", "Закрыть", "Закрити"],
    TryAgain => ["Try again", "Попробовать снова", "Спробувати знову"],
    Ready => [
        "Ready. Speech recognition is installed.",
        "Готово. Распознавание речи установлено.",
        "Готово. Розпізнавання мовлення встановлено."
    ],
    ReadyStarted => [
        "Transcribing this video now.",
        "Распознаём речь в этом видео.",
        "Розпізнаємо мовлення в цьому відео."
    ],
    ReadyHow => [
        "Open a video without subtitles and choose Subtitles → Transcribe speech.",
        "Откройте видео без субтитров и выберите «Субтитры → Распознать речь».",
        "Відкрийте відео без субтитрів і виберіть «Субтитри → Розпізнати мовлення»."
    ],
    NotInstalledStatus => ["Not installed", "Не установлено", "Не встановлено"],
    Advanced => [
        "Advanced: use my own whisper.cpp",
        "Дополнительно: использовать свой whisper.cpp",
        "Додатково: використовувати власний whisper.cpp"
    ],
    SetUpLink => [
        "Set up speech recognition for videos without subtitles…",
        "Настроить распознавание речи для видео без субтитров…",
        "Налаштувати розпізнавання мовлення для відео без субтитрів…"
    ],
    ToastReady => [
        "Speech recognition is installed",
        "Распознавание речи установлено",
        "Розпізнавання мовлення встановлено"
    ],
    ToastRemoved => [
        "Speech recognition was removed",
        "Распознавание речи удалено",
        "Розпізнавання мовлення видалено"
    ],
    NotSetUp => [
        "Speech recognition is not set up: choose Subtitles → Transcribe speech to install it",
        "Распознавание речи не настроено: выберите «Субтитры → Распознать речь», чтобы его установить",
        "Розпізнавання мовлення не налаштовано: виберіть «Субтитри → Розпізнати мовлення», щоб його встановити"
    ],
    NoServer => [
        "whisper-server was not found: set its path in Settings → Speech recognition → Advanced",
        "whisper-server не найден: укажите путь к нему в «Настройки → Распознавание речи → Дополнительно»",
        "whisper-server не знайдено: вкажіть шлях до нього в «Налаштування → Розпізнавання мовлення → Додатково»"
    ],
    NoModel => [
        "The Whisper model file was not found: set its path in Settings → Speech recognition → Advanced",
        "Файл модели Whisper не найден: укажите путь к нему в «Настройки → Распознавание речи → Дополнительно»",
        "Файл моделі Whisper не знайдено: вкажіть шлях до нього в «Налаштування → Розпізнавання мовлення → Додатково»"
    ],
    FirstRun => [
        "the first run on this graphics card can take up to a minute",
        "первый запуск на этой видеокарте может занять до минуты",
        "перший запуск на цій відеокарті може тривати до хвилини"
    ],
    SpaceLine => [
        "Takes {0} on disk; needs {1} free while installing",
        "Занимает на диске {0}; во время установки нужно {1} свободного места",
        "Займає на диску {0}; під час встановлення потрібно {1} вільного місця"
    ],
    SpaceLineFree => [
        "Takes {0} on disk; needs {1} free while installing ({2} free now)",
        "Занимает на диске {0}; во время установки нужно {1} свободного места (сейчас свободно {2})",
        "Займає на диску {0}; під час встановлення потрібно {1} вільного місця (зараз вільно {2})"
    ],
    GpuLine => [
        "Graphics card: {0}, {1} GB",
        "Видеокарта: {0}, {1} ГБ",
        "Відеокарта: {0}, {1} ГБ"
    ],
    InstalledStatus => [
        "Installed: {0}, {1} ({2})",
        "Установлено: {0}, {1} ({2})",
        "Встановлено: {0}, {1} ({2})"
    ],
    InstalledModelsOnly => [
        "Installed: model {0} (the engine is your own whisper.cpp)",
        "Установлено: модель {0} (движок — ваш собственный whisper.cpp)",
        "Встановлено: модель {0} (рушій — ваш власний whisper.cpp)"
    ],
    StagePreparing => ["Preparing…", "Подготовка…", "Підготовка…"],
    StageDownloading => ["Downloading {0}", "Загрузка: {0}", "Завантаження: {0}"],
    StageVerifying => ["Checking {0}", "Проверка: {0}", "Перевірка: {0}"],
    StageUnpacking => ["Unpacking {0}", "Распаковка: {0}", "Розпакування: {0}"],
    StageFinishing => ["Finishing…", "Завершение…", "Завершення…"],
    FileLine => ["{0}: {1} of {2}", "{0}: {1} из {2}", "{0}: {1} з {2}"],

    // Voice-over.
    TtsComponentName => ["Voice-over (OmniVoice)", "Озвучка (OmniVoice)", "Озвучення (OmniVoice)"],
    TtsLabelVulkan => [
        "audio.cpp voice engine for any graphics card (Vulkan)",
        "Голосовой движок audio.cpp для любой видеокарты (Vulkan)",
        "Голосовий рушій audio.cpp для будь-якої відеокарти (Vulkan)"
    ],
    TtsLabelNvidia => [
        "audio.cpp voice engine for NVIDIA graphics cards (CUDA)",
        "Голосовой движок audio.cpp для видеокарт NVIDIA (CUDA)",
        "Голосовий рушій audio.cpp для відеокарт NVIDIA (CUDA)"
    ],
    TtsLabelCudaLibs => [
        "NVIDIA CUDA libraries for the voice engine",
        "Библиотеки NVIDIA CUDA для голосового движка",
        "Бібліотеки NVIDIA CUDA для голосового рушія"
    ],
    TtsLabelModel => [
        "OmniVoice voice model (q8_0)",
        "Голосовая модель OmniVoice (q8_0)",
        "Голосова модель OmniVoice (q8_0)"
    ],
    TtsDialogTitle => ["Voice-over", "Озвучка", "Озвучення"],
    TtsDialogIntro => [
        "Glossplay can speak the translation over the video, with the original sound turned down. The voice is made on this computer by OmniVoice, so nothing is sent anywhere. It needs a one-time download and a graphics card.",
        "Glossplay может озвучивать перевод поверх видео, приглушая оригинальный звук. Голос создаётся на этом компьютере моделью OmniVoice, поэтому ничего никуда не отправляется. Нужны однократная загрузка и видеокарта.",
        "Glossplay може озвучувати переклад поверх відео, приглушуючи оригінальний звук. Голос створюється на цьому комп’ютері моделлю OmniVoice, тому нічого нікуди не надсилається. Потрібні одноразове завантаження та відеокарта."
    ],
    TtsSources => [
        "Downloaded from GitHub (0xShug0/audio.cpp, release v0.9.5) and Hugging Face (audio-cpp/audio.cpp-gguf).",
        "Загружается с GitHub (0xShug0/audio.cpp, релиз v0.9.5) и Hugging Face (audio-cpp/audio.cpp-gguf).",
        "Завантажується з GitHub (0xShug0/audio.cpp, реліз v0.9.5) і Hugging Face (audio-cpp/audio.cpp-gguf)."
    ],
    TtsLicenceNc => [
        "Non-commercial use only. The OmniVoice voice model (k2-fsa/OmniVoice) is licensed under CC BY-NC 4.0: neither the model nor what it says may be used for commercial purposes.",
        "Только для некоммерческого использования. Голосовая модель OmniVoice (k2-fsa/OmniVoice) распространяется по лицензии CC BY-NC 4.0: ни саму модель, ни озвученную ею речь нельзя использовать в коммерческих целях.",
        "Лише для некомерційного використання. Голосова модель OmniVoice (k2-fsa/OmniVoice) поширюється за ліцензією CC BY-NC 4.0: ні саму модель, ні озвучене нею мовлення не можна використовувати в комерційних цілях."
    ],
    TtsLicenceBoson => [
        "Built with Higgs Materials licensed from Boson AI USA, Inc. (the audio tokenizer inside the model, under the Boson Higgs Audio 2 Community License).",
        "Built with Higgs Materials licensed from Boson AI USA, Inc. (аудиотокенизатор внутри модели, лицензия Boson Higgs Audio 2 Community License).",
        "Built with Higgs Materials licensed from Boson AI USA, Inc. (аудіотокенізатор усередині моделі, ліцензія Boson Higgs Audio 2 Community License)."
    ],
    TtsNoImpersonation => [
        "Do not use the voice engine to imitate a real person's voice without their consent.",
        "Не используйте голосовой движок, чтобы имитировать голос реального человека без его согласия.",
        "Не використовуйте голосовий рушій, щоб імітувати голос реальної людини без її згоди."
    ],
    TtsLicenceEngine => [
        "The audio.cpp engine is Apache-2.0 (ShugoAI). The NVIDIA build includes NVIDIA CUDA libraries, redistributed under the NVIDIA CUDA licence.",
        "Движок audio.cpp распространяется по лицензии Apache-2.0 (ShugoAI). Сборка для NVIDIA включает библиотеки NVIDIA CUDA, которые распространяются по лицензии NVIDIA CUDA.",
        "Рушій audio.cpp поширюється за ліцензією Apache-2.0 (ShugoAI). Збірка для NVIDIA містить бібліотеки NVIDIA CUDA, які поширюються за ліцензією NVIDIA CUDA."
    ],
    TtsAgree => [
        "I will use the voice-over for non-commercial purposes only and accept these licences",
        "Я буду использовать озвучку только в некоммерческих целях и принимаю эти лицензии",
        "Я використовуватиму озвучення лише в некомерційних цілях і приймаю ці ліцензії"
    ],
    TtsNeedsGpu => [
        "The voice engine needs a graphics card. On the processor alone it is 6–10 times slower than the video plays and cannot keep up.",
        "Голосовому движку нужна видеокарта. На одном процессоре он работает в 6–10 раз медленнее воспроизведения видео и не успевает.",
        "Голосовому рушію потрібна відеокарта. На самому процесорі він працює в 6–10 разів повільніше за відтворення відео й не встигає."
    ],
    TtsNoVulkan => [
        "No Vulkan graphics driver was found on this computer, so the voice engine would run on the processor: 6–10 times slower than the video plays. Install the graphics card driver first.",
        "На этом компьютере не найден графический драйвер Vulkan, поэтому голосовой движок работал бы на процессоре: в 6–10 раз медленнее воспроизведения видео. Сначала установите драйвер видеокарты.",
        "На цьому комп’ютері не знайдено графічного драйвера Vulkan, тому голосовий рушій працював би на процесорі: у 6–10 разів повільніше за відтворення відео. Спершу встановіть драйвер відеокарти."
    ],
    TtsNvidiaWithoutGpu => [
        "No NVIDIA graphics card was found; the NVIDIA build will not start without one.",
        "Видеокарта NVIDIA не найдена; сборка для NVIDIA без неё не запустится.",
        "Відеокарту NVIDIA не знайдено; збірка для NVIDIA без неї не запуститься."
    ],
    TtsEngineVulkan => [
        "Any graphics card (Vulkan)",
        "Любая видеокарта (Vulkan)",
        "Будь-яка відеокарта (Vulkan)"
    ],
    TtsEngineNvidia => [
        "NVIDIA graphics card (CUDA): a little faster, much bigger",
        "Видеокарта NVIDIA (CUDA): чуть быстрее, но гораздо больше",
        "Відеокарта NVIDIA (CUDA): трохи швидше, але набагато більше"
    ],
    TtsBuildVulkan => ["Vulkan build", "сборка Vulkan", "збірка Vulkan"],
    TtsBuildNvidia => ["NVIDIA build", "сборка для NVIDIA", "збірка для NVIDIA"],
    TtsReady => [
        "Ready. The voice-over is installed.",
        "Готово. Озвучка установлена.",
        "Готово. Озвучення встановлено."
    ],
    TtsReadyOn => [
        "The voice-over is on now.",
        "Озвучка сейчас включена.",
        "Озвучення зараз увімкнено."
    ],
    TtsReadyHow => [
        "Turn it on in Subtitles → Voice-over.",
        "Включите её в «Субтитры → Озвучка перевода».",
        "Увімкніть його в «Субтитри → Озвучення перекладу»."
    ],
    TtsToastReady => [
        "The voice-over is installed",
        "Озвучка установлена",
        "Озвучення встановлено"
    ],
    TtsToastRemoved => [
        "The voice-over was removed",
        "Озвучка удалена",
        "Озвучення видалено"
    ],
    TtsNotSetUp => [
        "The voice-over is not set up: choose Subtitles → Voice-over to install it",
        "Озвучка не настроена: выберите «Субтитры → Озвучка перевода», чтобы её установить",
        "Озвучення не налаштовано: виберіть «Субтитри → Озвучення перекладу», щоб його встановити"
    ],
    TtsNoEngineHere => [
        "There is no ready-made voice engine for this system: run an OpenAI-style speech server yourself and set its address under Advanced.",
        "Для этой системы нет готового голосового движка: запустите сервер синтеза речи в стиле OpenAI самостоятельно и укажите его адрес в разделе «Дополнительно».",
        "Для цієї системи немає готового голосового рушія: запустіть сервер синтезу мовлення в стилі OpenAI самостійно та вкажіть його адресу в розділі «Додатково»."
    ],
    TtsNeedsTranslation => [
        "The voice-over speaks the translation: choose one under Subtitles → Translation",
        "Озвучка читает перевод: выберите его в «Субтитры → Перевод»",
        "Озвучення читає переклад: виберіть його в «Субтитри → Переклад»"
    ],
    TtsInstalledStatus => [
        "Installed: {0}, OmniVoice q8_0 ({1})",
        "Установлено: {0}, OmniVoice q8_0 ({1})",
        "Встановлено: {0}, OmniVoice q8_0 ({1})"
    ],
    MenuVoiceOver => [
        "Voice-over (dub the translation)",
        "Озвучка перевода",
        "Озвучення перекладу"
    ],
    MenuVoiceOverHint => [
        "Speak the translation over the video and turn the original sound down",
        "Озвучить перевод поверх видео и приглушить оригинальный звук",
        "Озвучити переклад поверх відео та приглушити оригінальний звук"
    ],
    SliderOriginal => ["Original", "Громкость оригинала", "Гучність оригіналу"],
    SliderVoice => ["Voice-over", "Громкость озвучки", "Гучність озвучення"],
    SectionVoiceOver => ["Voice-over", "Озвучка перевода", "Озвучення перекладу"],
    TtsEnable => [
        "Speak the translation over the video",
        "Озвучивать перевод поверх видео",
        "Озвучувати переклад поверх відео"
    ],
    RowOriginalVolume => [
        "Original sound, %",
        "Оригинальный звук, %",
        "Оригінальний звук, %"
    ],
    RowVoiceVolume => ["Voice, %", "Голос, %", "Голос, %"],
    RowVoiceSource => ["Voice", "Голос", "Голос"],
    VoiceSpeaker => [
        "like the speaker's",
        "как у говорящего",
        "як у мовця"
    ],
    VoiceStandard => ["standard", "стандартный", "стандартний"],
    VoiceSpeakerNote => [
        "The speaker's voice is copied from a few seconds of the video only on this computer, for your own viewing, and is sent nowhere.",
        "Голос говорящего копируется по нескольким секундам видео только на этом компьютере, для личного просмотра, и никуда не отправляется.",
        "Голос мовця копіюється за кількома секундами відео лише на цьому комп'ютері, для особистого перегляду, і нікуди не надсилається."
    ],
    DubVoiceSample => [
        "Voice-over: listening to the speaker's voice…",
        "Озвучка: слушаем голос говорящего…",
        "Озвучення: слухаємо голос мовця…"
    ],
    TtsAdvanced => [
        "Advanced: my own speech server",
        "Дополнительно: свой сервер синтеза речи",
        "Додатково: власний сервер синтезу мовлення"
    ],
    RowServer => ["Server address", "Адрес сервера", "Адреса сервера"],
    HintServer => [
        "empty = the installed engine; http://host:port/v1",
        "пусто = установленный; http://host:port/v1",
        "порожньо = встановлений; http://host:port/v1"

    ],
    RowTtsModel => ["Model", "Модель", "Модель"],
    RowVoice => ["Voice", "Голос", "Голос"],
    HintVoice => [
        "the server's default",
        "по умолчанию на сервере",
        "за замовчуванням на сервері"
    ],
    TtsLanguage => [
        "Spoken in: {0} (\"My language\" on the Languages tab)",
        "Язык озвучки: {0} («Мой язык» на вкладке «Языки»)",
        "Мова озвучення: {0} («Моя мова» на вкладці «Мови»)"
    ],
    DubStarting => [
        "Voice-over: starting the voice engine…",
        "Озвучка: запускаем голосовой движок…",
        "Озвучення: запускаємо голосовий рушій…"
    ],
    DubFirstStart => [
        "the first start on this graphics card takes up to half a minute",
        "первый запуск на этой видеокарте занимает до полуминуты",
        "перший запуск на цій відеокарті триває до півхвилини"
    ],
    DubPreparing => [
        "Voice-over: preparing…",
        "Озвучка: подготовка…",
        "Озвучення: підготовка…"
    ],
    DubWaiting => [
        "Preparing the voice-over",
        "Готовлю озвучку",
        "Готую озвучення"
    ],
    DubOn => ["Voice-over", "Озвучка", "Озвучення"],
    DubStopped => [
        "Voice-over stopped: {0}",
        "Озвучка остановлена: {0}",
        "Озвучення зупинено: {0}"
    ],
    DubLineFailed => [
        "Voice-over: a line could not be spoken ({0})",
        "Озвучка: не удалось озвучить реплику ({0})",
        "Озвучення: не вдалося озвучити репліку ({0})"
    ],
    DubEngineGone => [
        "the voice engine did not start; check the install in Settings",
        "голосовой движок не запустился; проверьте установку в настройках",
        "голосовий рушій не запустився; перевірте встановлення в налаштуваннях"
    ],
    DubEngineQuit => [
        "the voice engine quit unexpectedly {0} times",
        "голосовой движок неожиданно завершался {0} раз",
        "голосовий рушій несподівано завершувався {0} разів"
    ],
    DubCannotStart => [
        "could not start the voice engine: {0}",
        "не удалось запустить голосовой движок: {0}",
        "не вдалося запустити голосовий рушій: {0}"
    ],
    DubNoAudioOut => [
        "could not open a second audio player for the voice",
        "не удалось открыть второй аудиоплеер для голоса",
        "не вдалося відкрити другий аудіоплеєр для голосу"
    ],

    // Search in the user's own sources.
    SearchVideosHint => ["Search videos (Ctrl+F)", "Поиск видео (Ctrl+F)", "Пошук відео (Ctrl+F)"],
    SearchMenu => ["Search…", "Поиск…", "Пошук…"],
    SearchField => ["Search for a video", "Поиск видео", "Пошук відео"],
    SearchButton => ["Search", "Найти", "Знайти"],
    SearchTitle => ["Search your sources", "Поиск по своим источникам", "Пошук у своїх джерелах"],
    ShortcutsSearch => [
        "Ctrl+F search your sources",
        "Ctrl+F — поиск по своим источникам",
        "Ctrl+F — пошук у своїх джерелах"
    ],
    Searching => ["Searching…", "Ищу…", "Шукаю…"],
    NothingFound => ["Nothing found", "Ничего не найдено", "Нічого не знайдено"],
    FoundCount => [
        "Found: {0} · double-click or Enter to play",
        "Найдено: {0} · двойной щелчок или Enter — смотреть",
        "Знайдено: {0} · подвійне клацання або Enter — дивитися"
    ],
    ColTitle => ["Title", "Название", "Назва"],
    ColSize => ["Size", "Размер", "Розмір"],
    ColPeers => ["Seeds / peers", "Сиды / пиры", "Сиди / піри"],
    ColDate => ["Date", "Дата", "Дата"],
    ColSource => ["Source", "Источник", "Джерело"],
    GettingFiles => [
        "Getting the list of files…",
        "Получаю список файлов…",
        "Отримую список файлів…"
    ],
    ChooseFile => [
        "Video files: {0} · double-click or Enter to play",
        "Видеофайлов: {0} · двойной щелчок или Enter — смотреть",
        "Відеофайлів: {0} · подвійне клацання або Enter — дивитися"
    ],
    BackToResults => ["← Results", "← К результатам", "← До результатів"],
    NoVideoFiles => [
        "There are no video files in this torrent",
        "В этой раздаче нет видеофайлов",
        "У цій роздачі немає відеофайлів"
    ],
    SearchUnreachable => [
        "The search server does not answer. Is it running, and is the address right? ({0})",
        "Сервер поиска не отвечает. Он запущен, адрес верный? ({0})",
        "Сервер пошуку не відповідає. Він запущений, адреса правильна? ({0})"
    ],
    SearchBadKey => [
        "The search server did not accept the API key; check it in Settings → Sources",
        "Сервер поиска не принял ключ API; проверьте его в Настройках → Источники",
        "Сервер пошуку не прийняв ключ API; перевірте його в Налаштуваннях → Джерела"
    ],
    SearchWrongAddress => [
        "There is no search server at this address (HTTP {0}); check it in Settings → Sources",
        "По этому адресу нет сервера поиска (HTTP {0}); проверьте его в Настройках → Источники",
        "За цією адресою немає сервера пошуку (HTTP {0}); перевірте її в Налаштуваннях → Джерела"
    ],
    SearchFailed => [
        "The search server answered with an error: {0}",
        "Сервер поиска ответил ошибкой: {0}",
        "Сервер пошуку відповів помилкою: {0}"
    ],
    LinkFailed => [
        "Could not get the torrent from the search server: {0}",
        "Не удалось получить торрент от сервера поиска: {0}",
        "Не вдалося отримати торент від сервера пошуку: {0}"
    ],
    TorrServerUnreachable => [
        "TorrServer does not answer. Is it running, and is the address right? ({0})",
        "TorrServer не отвечает. Он запущен, адрес верный? ({0})",
        "TorrServer не відповідає. Він запущений, адреса правильна? ({0})"
    ],
    NoMetadata => [
        "No peer sent the list of files within 30 seconds: the torrent may have no seeders",
        "За 30 секунд никто не прислал список файлов: возможно, у раздачи нет сидов",
        "За 30 секунд ніхто не надіслав список файлів: можливо, у роздачі немає сидів"
    ],
    TorrServerFailed => [
        "TorrServer answered with an error: {0}",
        "TorrServer ответил ошибкой: {0}",
        "TorrServer відповів помилкою: {0}"
    ],
    SectionSources => ["Sources", "Источники", "Джерела"],
    SourcesNote => [
        "Glossplay contains no sources. Enter the address of your own search server (Jackett, Prowlarr — the Torznab protocol) and of your TorrServer.",
        "Glossplay не содержит источников. Укажите адрес своего сервера поиска (Jackett, Prowlarr — протокол Torznab) и своего TorrServer.",
        "Glossplay не містить джерел. Вкажіть адресу свого сервера пошуку (Jackett, Prowlarr — протокол Torznab) і свого TorrServer."
    ],
    SearchServerRow => ["Search server (Torznab)", "Сервер поиска (Torznab)", "Сервер пошуку (Torznab)"],
    TorrServerRow => ["TorrServer", "TorrServer", "TorrServer"],
    CheckButton => ["Check", "Проверить", "Перевірити"],
    Checking => ["Checking…", "Проверяю…", "Перевіряю…"],
    CheckSearchOk => [
        "Search server: {0} sources found",
        "Сервер поиска: найдено источников — {0}",
        "Сервер пошуку: знайдено джерел — {0}"
    ],
    CheckSearchFailed => ["Search server: {0}", "Сервер поиска: {0}", "Сервер пошуку: {0}"],
    CheckTorrServerOk => ["TorrServer {0}", "TorrServer {0}", "TorrServer {0}"],
    CheckTorrServerFailed => ["TorrServer: {0}", "TorrServer: {0}", "TorrServer: {0}"],
    CheckNotSet => ["the address is empty", "адрес не указан", "адресу не вказано"],
    PasteNothing => [
        "The clipboard holds no link or file path",
        "В буфере обмена нет ссылки или пути к файлу",
        "У буфері обміну немає посилання чи шляху до файлу"
    ],
    PasteNeedsTorrServer => [
        "To play a magnet link, set the TorrServer address in Settings → Sources",
        "Чтобы смотреть magnet-ссылку, укажите адрес TorrServer в Настройках → Источники",
        "Щоб дивитися magnet-посилання, вкажіть адресу TorrServer у Налаштуваннях → Джерела"
    ],

    // Settings tabs: what each is for, and the notes behind the ⓘ signs.
    SectionWords => ["Words and Anki", "Слова и Anki", "Слова та Anki"],
    IntroLanguages => [
        "Subtitles in the language of the video go on top, the translation into your language goes under them.",
        "Сверху — субтитры на языке видео, под ними — перевод на ваш язык.",
        "Зверху — субтитри мовою відео, під ними — переклад вашою мовою."
    ],
    IntroSubtitles => [
        "How the two lines of subtitles look over the video. While watching, {0} and {1} make them smaller and larger.",
        "Как выглядят две строки субтитров поверх видео. Во время просмотра клавиши {0} и {1} уменьшают и увеличивают их.",
        "Як виглядають два рядки субтитрів поверх відео. Під час перегляду клавіші {0} і {1} зменшують і збільшують їх."
    ],
    IntroTranslation => [
        "Translates the subtitles into your language. Google Translate works without a key; DeepL, OpenAI and Claude need an API key.",
        "Переводит субтитры на ваш язык. Google Translate работает без ключа; для DeepL, OpenAI и Claude нужен ключ API.",
        "Перекладає субтитри вашою мовою. Google Translate працює без ключа; для DeepL, OpenAI і Claude потрібен ключ API."
    ],
    IntroSpeech => [
        "Makes subtitles from the speech when a video has none. It runs on this computer after a one-time download. While watching: Subtitles menu → \"{0}\".",
        "Делает субтитры из речи, если у видео их нет. Работает на этом компьютере после однократной загрузки. Во время просмотра: меню «Субтитры» → «{0}».",
        "Робить субтитри з мовлення, якщо у відео їх немає. Працює на цьому комп’ютері після одноразового завантаження. Під час перегляду: меню «Субтитри» → «{0}»."
    ],
    IntroVoice => [
        "Reads the translation aloud over the video and turns the original sound down. While watching: Subtitles menu → \"{0}\".",
        "Читает перевод вслух поверх видео и приглушает оригинальный звук. Во время просмотра: меню «Субтитры» → «{0}».",
        "Читає переклад уголос поверх відео і приглушує оригінальний звук. Під час перегляду: меню «Субтитри» → «{0}»."
    ],
    IntroWords => [
        "Click a word in the subtitles to see its translation. Saved words are in the side panel ({0}); Anki, a flashcard app, can get them as cards.",
        "Щёлкните слово в субтитрах, чтобы увидеть перевод. Сохранённые слова — на боковой панели ({0}); их можно отправить карточками в Anki — программу для запоминания слов.",
        "Клацніть слово в субтитрах, щоб побачити переклад. Збережені слова — на бічній панелі ({0}); їх можна надіслати картками в Anki — програму для запам’ятовування слів."
    ],
    IntroPlayer => [
        "The look of the window and how video is played. The defaults suit most computers.",
        "Вид окна и воспроизведение видео. Значения по умолчанию подходят большинству компьютеров.",
        "Вигляд вікна і відтворення відео. Типові значення підходять більшості комп’ютерів."
    ],
    RowVideoLang => ["Language of the videos", "Язык видео", "Мова відео"],
    RowMyLang => ["My language", "Мой язык", "Моя мова"],
    LangDetect => ["Detect from each video", "Определять по видео", "Визначати за відео"],
    InfoLearn => [
        "The language you are learning. It decides which subtitles go on top and which language speech recognition listens for.",
        "Язык, который вы изучаете. От него зависит, какие субтитры будут сверху и какой язык слушает распознавание речи.",
        "Мова, яку ви вивчаєте. Від неї залежить, які субтитри будуть зверху і яку мову слухає розпізнавання мовлення."
    ],
    InfoNative => [
        "Translation, voice-over and word explanations are in this language.",
        "На этом языке будут перевод, озвучка и объяснения слов.",
        "Цією мовою будуть переклад, озвучення і пояснення слів."
    ],
    InfoInterface => [
        "The language of menus and buttons.",
        "Язык меню и кнопок.",
        "Мова меню і кнопок."
    ],
    InfoAutoTranslate => [
        "When a video has no subtitles in your language, the translation service makes them.",
        "Если у видео нет субтитров на вашем языке, их сделает сервис перевода.",
        "Якщо у відео немає субтитрів вашою мовою, їх зробить сервіс перекладу."
    ],
    InfoApiKey => [
        "A key from the service's website. It stays in the settings file on this computer.",
        "Ключ с сайта сервиса. Он хранится в файле настроек на этом компьютере.",
        "Ключ із сайту сервісу. Він зберігається у файлі налаштувань на цьому комп’ютері."
    ],
    InfoAsrAuto => [
        "Starts speech recognition by itself when a video has no subtitles at all.",
        "Запускает распознавание речи само, если у видео совсем нет субтитров.",
        "Запускає розпізнавання мовлення саме, якщо у відео зовсім немає субтитрів."
    ],
    InfoVolumes => [
        "While the voice speaks: how loud the film's own sound stays and how loud the voice is, in percent. The same two sliders are in the Subtitles menu.",
        "Пока звучит озвучка: насколько громким остаётся звук фильма и насколько громок голос, в процентах. Те же два ползунка есть в меню «Субтитры».",
        "Поки звучить озвучення: наскільки гучним лишається звук фільму і наскільки гучний голос, у відсотках. Ті самі два повзунки є в меню «Субтитри»."
    ],
    InfoAnki => [
        "Anki needs the AnkiConnect add-on and has to be running. Saved words go to this deck.",
        "Нужны запущенная Anki и дополнение AnkiConnect. Сохранённые слова попадут в эту колоду.",
        "Потрібні запущена Anki і доповнення AnkiConnect. Збережені слова потраплять у цю колоду."
    ],
    InfoHwdec => [
        "Lets the graphics card decode the video. Leave it empty or \"auto\"; write \"no\" if the picture has glitches.",
        "Декодирование видео видеокартой. Оставьте пустым или «auto»; напишите «no», если картинка с артефактами.",
        "Декодування відео відеокартою. Залиште порожнім або «auto»; напишіть «no», якщо зображення з артефактами."
    ],
    InfoFfmpeg => [
        "ffmpeg reads the subtitles inside video files and the sound for speech recognition.",
        "ffmpeg читает субтитры внутри видеофайлов и звук для распознавания речи.",
        "ffmpeg читає субтитри всередині відеофайлів і звук для розпізнавання мовлення."
    ],
    InfoScale => [
        "Makes all text and buttons larger or smaller. It applies when you press Save.",
        "Делает весь текст и кнопки крупнее или мельче. Применяется при сохранении.",
        "Робить увесь текст і кнопки більшими або меншими. Застосовується під час збереження."
    ],
    AsrListensFor => [
        "Listens for: {0} (the language of the videos)",
        "Слушает язык: {0} (язык видео)",
        "Слухає мову: {0} (мова відео)"
    ],
    AsrListensAny => [
        "Listens for any language and works out which one it is",
        "Слушает любой язык и сам определяет, какой",
        "Слухає будь-яку мову і сам визначає, яку"
    ],
    ChangeLanguage => ["Change…", "Изменить…", "Змінити…"],
    TtsLangUnknown => [
        "The voice engine may not know this language.",
        "Движок озвучки может не знать этот язык.",
        "Рушій озвучення може не знати цю мову."
    ],
    SettingsOf => ["Settings: {0}", "Настройки: {0}", "Налаштування: {0}"],

    // The "How to use" window.
    HelpTitle => ["How to use", "Как пользоваться", "Як користуватися"],
    HelpHint => ["How to use Glossplay", "Как пользоваться Glossplay", "Як користуватися Glossplay"],
    HelpMenu => ["How to use…", "Как пользоваться…", "Як користуватися…"],
    HelpOpenTitle => ["Open a video", "Откройте видео", "Відкрийте відео"],
    HelpOpenText => [
        "Press \"{0}\", drop a file into the window, or paste a link to a video page into the field. Ctrl+V with a copied link works too.",
        "Нажмите «{0}», перетащите файл в окно или вставьте в поле ссылку на страницу с видео. Ctrl+V со скопированной ссылкой тоже работает.",
        "Натисніть «{0}», перетягніть файл у вікно або вставте в поле посилання на сторінку з відео. Ctrl+V зі скопійованим посиланням теж працює."
    ],
    HelpSubsTitle => ["Two lines of subtitles", "Две строки субтитров", "Два рядки субтитрів"],
    HelpSubsText => [
        "On top: the language of the video. Under it: the translation into your language. Set both languages once in Settings; other tracks are in the \"{0}\" menu.",
        "Сверху — язык видео, под ним — перевод на ваш язык. Оба языка задаются один раз в настройках; другие дорожки — в меню «{0}».",
        "Зверху — мова відео, під нею — переклад вашою мовою. Обидві мови задаються один раз у налаштуваннях; інші доріжки — в меню «{0}»."
    ],
    HelpAsrTitle => ["No subtitles?", "Нет субтитров?", "Немає субтитрів?"],
    HelpAsrText => [
        "\"{0}\" menu → \"{1}\": the player listens to the speech and writes it down. The first time it offers a one-time download.",
        "Меню «{0}» → «{1}»: плеер слушает речь и записывает её. В первый раз он предложит однократную загрузку.",
        "Меню «{0}» → «{1}»: плеєр слухає мовлення і записує його. Першого разу він запропонує одноразове завантаження."
    ],
    HelpVoiceTitle => ["Hear the translation", "Слушайте перевод", "Слухайте переклад"],
    HelpVoiceText => [
        "\"{0}\" menu → \"{1}\": the translation is read aloud over the video. Two sliders appear under it: \"{2}\" is the film's sound, \"{3}\" is the voice.",
        "Меню «{0}» → «{1}»: перевод читается вслух поверх видео. Под этим пунктом появятся два ползунка: «{2}» — звук фильма, «{3}» — голос.",
        "Меню «{0}» → «{1}»: переклад читається вголос поверх відео. Під цим пунктом з’являться два повзунки: «{2}» — звук фільму, «{3}» — голос."
    ],
    HelpWordsTitle => ["Look up words", "Смотрите слова", "Дивіться слова"],
    HelpWordsText => [
        "Click a word in the subtitles to see its translation. \"{0}\" keeps it in your list ({1}), and from there it can go to Anki.",
        "Щёлкните слово в субтитрах, чтобы увидеть перевод. «{0}» добавит его в ваш список ({1}), а оттуда его можно отправить в Anki.",
        "Клацніть слово в субтитрах, щоб побачити переклад. «{0}» додасть його до вашого списку ({1}), а звідти його можна надіслати в Anki."
    ],
    HelpKeysTitle => ["Keys", "Клавиши", "Клавіші"],
    KeySpace => ["Space", "Пробел", "Пробіл"],
    KeyPlayPause => ["Play or pause", "Пауза и продолжение", "Пауза і продовження"],
    KeySeek => ["5 seconds back or forward", "На 5 секунд назад или вперёд", "На 5 секунд назад або вперед"],
    KeyVolume => ["Louder or quieter", "Громче или тише", "Гучніше або тихіше"],
    KeyLines => [
        "Previous line, repeat it, next line",
        "Предыдущая реплика, повтор, следующая",
        "Попередня репліка, повтор, наступна"
    ],
    KeyToggleLines => [
        "Hide or show the original or the translation",
        "Скрыть или показать оригинал или перевод",
        "Сховати або показати оригінал чи переклад"
    ],
    KeyAutoPause => [
        "Pause after every line: on or off",
        "Пауза после каждой реплики: вкл. или выкл.",
        "Пауза після кожної репліки: увімк. або вимк."
    ],
    KeyFullscreen => ["Full screen", "Во весь экран", "На весь екран"],
    KeySidebar => [
        "Transcript and saved words",
        "Транскрипт и сохранённые слова",
        "Транскрипт і збережені слова"
    ],
    KeyMute => ["Sound off or on", "Выключить или включить звук", "Вимкнути або увімкнути звук"],
    KeySpeed => ["Slower or faster", "Медленнее или быстрее", "Повільніше або швидше"],
    KeySubSize => ["Smaller or larger subtitles", "Субтитры мельче или крупнее", "Субтитри менші або більші"],
    KeyOpenFile => ["Open a file", "Открыть файл", "Відкрити файл"],
    KeySearch => ["Search videos", "Поиск видео", "Пошук відео"],
    KeyClose => ["Close a window", "Закрыть окно", "Закрити вікно"],

    // Buffering, the context menu, saved frames and the video info window.
    Buffering => ["Buffering", "Буферизация", "Буферизація"],
    PercentValue => ["{0}%", "{0} %", "{0} %"],
    SpeedKb => ["{0} KB/s", "{0} КБ/с", "{0} КБ/с"],
    SpeedMb => ["{0} MB/s", "{0} МБ/с", "{0} МБ/с"],
    MenuPlay => ["Play", "Воспроизвести", "Відтворити"],
    MenuPause => ["Pause", "Пауза", "Пауза"],
    MenuSpeed => ["Speed", "Скорость", "Швидкість"],
    MenuRepeatLine => ["Repeat the line", "Повторить реплику", "Повторити репліку"],
    MenuOpenClipboard => [
        "Open the copied link",
        "Открыть ссылку из буфера",
        "Відкрити посилання з буфера"
    ],
    KeyCtrlV => ["Ctrl+V", "Ctrl+V", "Ctrl+V"],
    MenuAudioTrack => ["Audio track", "Звуковая дорожка", "Звукова доріжка"],
    MenuVideo => ["Picture", "Изображение", "Зображення"],
    MenuAspect => ["Aspect ratio", "Соотношение сторон", "Співвідношення сторін"],
    MenuAspectAuto => ["As in the file", "Как в файле", "Як у файлі"],
    MenuRotate => ["Rotate 90°", "Повернуть на 90°", "Повернути на 90°"],
    MenuFill => ["Fill the window", "Заполнить окно", "Заповнити вікно"],
    MenuDeinterlace => ["Deinterlace", "Устранить чересстрочность", "Усунути черезрядковість"],
    MenuVideoReset => ["Reset", "Сбросить", "Скинути"],
    MenuFrame => ["Frame", "Снимок кадра", "Знімок кадру"],
    MenuSaveFrame => ["Save the frame", "Сохранить кадр", "Зберегти кадр"],
    MenuSaveFrameSubs => [
        "Save the frame with subtitles",
        "Сохранить кадр с субтитрами",
        "Зберегти кадр із субтитрами"
    ],
    MenuCopyFrame => ["Copy the frame", "Скопировать кадр", "Скопіювати кадр"],
    MenuCopyTime => ["Copy the time ({0})", "Скопировать время ({0})", "Скопіювати час ({0})"],
    MenuCopyLink => [
        "Copy a link to this moment",
        "Скопировать ссылку на этот момент",
        "Скопіювати посилання на цей момент"
    ],
    MenuFullscreen => ["Full screen", "Во весь экран", "На весь екран"],
    MenuOnTop => ["Always on top", "Поверх всех окон", "Поверх усіх вікон"],
    FrameSaved => ["Frame saved: {0}", "Кадр сохранён: {0}", "Кадр збережено: {0}"],
    FrameNotSaved => [
        "Could not save the frame: {0}",
        "Не удалось сохранить кадр: {0}",
        "Не вдалося зберегти кадр: {0}"
    ],
    FrameCopied => ["Frame copied", "Кадр скопирован", "Кадр скопійовано"],
    ShowInFolder => ["Show in folder", "Показать в папке", "Показати в теці"],
    Copied => ["Copied: {0}", "Скопировано: {0}", "Скопійовано: {0}"],
    LinkCopied => ["Link copied", "Ссылка скопирована", "Посилання скопійовано"],
    FramesFolderRow => ["Frames folder", "Папка для кадров", "Тека для кадрів"],
    InfoFramesFolder => [
        "Where \"Save the frame\" of the menu on the picture puts its images. Empty: the Glossplay folder in your Pictures.",
        "Куда пункт «Сохранить кадр» меню на картинке кладёт снимки. Пусто — папка Glossplay в «Изображениях».",
        "Куди пункт «Зберегти кадр» меню на зображенні кладе знімки. Порожньо — тека Glossplay у «Зображеннях»."
    ],
    KeyVideoInfo => ["Video info", "Сведения о видео", "Відомості про відео"],
    InfoWindowTitle => ["Video info", "Сведения о видео", "Відомості про відео"],
    InfoCopyAll => ["Copy all", "Скопировать всё", "Скопіювати все"],
    InfoCopied => ["Video info copied", "Сведения скопированы", "Відомості скопійовано"],
    InfoFile => ["File", "Файл", "Файл"],
    InfoTitleRow => ["Title", "Название", "Назва"],
    InfoSource => ["Source", "Источник", "Джерело"],
    InfoContainer => ["Format", "Формат", "Формат"],
    InfoDuration => ["Duration", "Длительность", "Тривалість"],
    InfoSize => ["Size", "Размер", "Розмір"],
    InfoVideo => ["Video", "Видео", "Відео"],
    InfoCodec => ["Codec", "Кодек", "Кодек"],
    InfoResolution => ["Resolution", "Разрешение", "Роздільна здатність"],
    InfoFps => ["Frame rate", "Частота кадров", "Частота кадрів"],
    InfoBitrate => ["Bitrate", "Битрейт", "Бітрейт"],
    InfoPixelFormat => ["Pixel format", "Формат пикселей", "Формат пікселів"],
    InfoColour => ["Colour", "Цвет", "Колір"],
    InfoSoftware => ["No (software)", "Нет (программное)", "Ні (програмне)"],
    InfoAudio => ["Sound", "Звук", "Звук"],
    InfoTrack => ["Track", "Дорожка", "Доріжка"],
    InfoChannels => ["Channels", "Каналы", "Канали"],
    InfoSampleRate => ["Sample rate", "Частота дискретизации", "Частота дискретизації"],
    InfoSubtitles => ["Subtitles", "Субтитры", "Субтитри"],
    InfoSecondLine => ["Line under it", "Строка под ней", "Рядок під ним"],
    InfoPlayback => ["Playback", "Воспроизведение", "Відтворення"],
    InfoDropped => ["Dropped frames", "Пропущено кадров", "Пропущено кадрів"],
    InfoDecoderDropped => [
        "Dropped by the decoder",
        "Пропущено декодером",
        "Пропущено декодером"
    ],
    InfoMistimed => ["Frames shown late", "Кадров с опозданием", "Кадрів із запізненням"],
    InfoOutputFps => ["Output frame rate", "Частота вывода", "Частота виведення"],
    InfoNetwork => ["Network", "Сеть", "Мережа"],
    InfoCacheState => ["State", "Состояние", "Стан"],
    InfoCacheOk => ["Playing", "Воспроизводится", "Відтворюється"],
    InfoBuffered => ["Buffered ahead", "Загружено вперёд", "Завантажено вперед"],
    InfoBufferedBytes => ["In the buffer", "В буфере", "У буфері"],
    InfoSpeed => ["Download speed", "Скорость загрузки", "Швидкість завантаження"],
    ChangeFolder => ["Change…", "Изменить…", "Змінити…"],
    ComponentsFolderRow => ["Download folder", "Папка для загрузок", "Тека для завантажень"],
    InfoComponentsFolder => [
        "Where speech recognition and the voice-over engine are downloaded with their models; they take gigabytes. Empty: the program's data folder. What is installed already stays where it is and keeps working.",
        "Куда скачиваются распознавание речи и движок озвучки с моделями; они занимают гигабайты. Пусто — папка данных программы. Уже установленное остаётся на месте и работает.",
        "Куди завантажуються розпізнавання мовлення й рушій озвучення з моделями; вони займають гігабайти. Порожньо — тека даних програми. Уже встановлене лишається на місці й працює."
    ],
    FreeOnDisk => ["{0} free on this disk", "Свободно на диске: {0}", "Вільно на диску: {0}"],
    UnitSeconds => ["{0} s", "{0} с", "{0} с"],
    UnitFps => ["{0} fps", "{0} к/с", "{0} к/с"],
    UnitKbps => ["{0} kbit/s", "{0} кбит/с", "{0} кбіт/с"],
    UnitMbps => ["{0} Mbit/s", "{0} Мбит/с", "{0} Мбіт/с"],
    UnitKhz => ["{0} kHz", "{0} кГц", "{0} кГц"],
    UnitKb => ["{0} KB", "{0} КБ", "{0} КБ"],
}
