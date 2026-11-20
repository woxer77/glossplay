//! Simplified Chinese texts of the interface. They are kept apart from the three-column table
//! in `messages.rs`; the match below has to name every message, so none can be forgotten.

use super::messages::Tr;

pub fn text(key: Tr) -> &'static str {
    match key {
        // Control bar.
        Tr::PlayHint => "播放（空格）",
        Tr::PauseHint => "暂停（空格）",
        Tr::PrevLineHint => "上一句（A）",
        Tr::NextLineHint => "下一句（D）",
        Tr::RepeatLineHint => "重复这一句（S）",
        Tr::FullscreenHint => "全屏（F）",
        Tr::SettingsHint => "设置",
        Tr::SidebarHint => "字幕文本和已存单词（T）",
        Tr::MuteHint => "静音（M）",
        Tr::AutoPause => "自动暂停",
        Tr::AutoPauseHint => "每句之后暂停（P）",
        Tr::AutoPauseOn => "每句之后自动暂停：开",
        Tr::AutoPauseOff => "自动暂停：关",
        Tr::ReadingSubtitles => "正在读取字幕…",

        // Empty screen.
        Tr::Tagline => "用你正在学的语言看视频",
        Tr::OpenVideo => "打开视频…",
        Tr::DropHint => "或把文件拖到窗口中，或复制链接或路径后按 Ctrl+V",
        Tr::PasteLink => "粘贴视频链接",
        Tr::PlayButton => "播放",
        Tr::ShortcutsHelp => {
            "空格 暂停 · A / S / D 上一句、重复、下一句 · P 自动暂停\nT 字幕文本 · F 全屏 · 点击单词查看释义"
        }
        Tr::SettingsLink => "设置",

        // Subtitles menu.
        Tr::SubtitlesMenu => "字幕",
        Tr::Learning => "我在学",
        Tr::StateReading => "读取中…",
        Tr::StateAsYouWatch => "边看边加载",
        Tr::StateFailed => "失败",
        Tr::StateImage => "图片",
        Tr::NoneChoice => "无",
        Tr::TranslationLabel => "翻译",
        Tr::TranslateAuto => "自动翻译（{0}）",
        Tr::ProviderNotSetUp => "未设置",
        Tr::OffChoice => "关闭",
        Tr::StopTranscribing => "停止语音识别",
        Tr::TranscribeSpeech => "识别语音（Whisper）",
        Tr::AddSubFile => "添加字幕文件…",
        Tr::SaveTranscriptSrt => "将字幕文本保存为 .srt…",
        Tr::SaveTranslationSrt => "将翻译保存为 .srt…",
        Tr::AudioLabel => "音轨",
        Tr::AutoPauseMenu => "每句之后自动暂停（P）",
        Tr::SettingsMenu => "设置…",
        Tr::SubStyleLabel => "样式",
        Tr::SubStylePlate => "底板",
        Tr::SubStyleOutline => "描边",
        Tr::SubStyleBold => "大号粗体",
        Tr::SubStyleYellow => "经典黄色",
        Tr::SubStyleLight => "浅色底板",
        Tr::SubSizeLabel => "大小",
        Tr::SubSizeMenu => "大小 {0}%（- / =）",
        Tr::SubPositionLabel => "位置",
        Tr::SubPosBottom => "底部",
        Tr::SubPosTop => "顶部",
        Tr::SubMarginLabel => "与边缘的距离",
        Tr::SubPreviewLabel => "预览",
        Tr::SubSampleOriginal => "Learn a language by watching",
        Tr::SubSampleTranslation => "边看边学",
        Tr::SubSizeToast => "字幕大小 {0}%",
        Tr::ThemeRow => "主题",
        Tr::ThemeDark => "深色",
        Tr::ThemeLight => "浅色",
        Tr::ThemeSystem => "跟随系统",
        Tr::CouldNotSaveSettings => "无法保存设置：{0}",
        Tr::SavedLines => "已将 {0} 句保存到 {1}",
        Tr::CouldNotSaveFile => "无法保存 {0}：{1}",

        // Sidebar.
        Tr::TabTranscript => "字幕文本",
        Tr::TabWords => "单词（{0}）",
        Tr::CloseHint => "关闭（T）",
        Tr::SearchHint => "🔍 搜索",
        Tr::OpenToSeeTranscript => "打开视频即可看到它的字幕文本。",
        Tr::NoTextSubtitles => "还没有文本字幕。",
        Tr::WordsEmpty => "点击字幕中的单词并按“保存”，它就会出现在这里。",
        Tr::ShowWord => "▶ 显示",
        Tr::JumpHint => "跳到这一时刻",
        Tr::InAnki => "✔ 已在 Anki 中",
        Tr::SendToAnki => "发送到 Anki",
        Tr::InCustomDict => "✔ 已在词典中",
        Tr::SendToCustomDict => "发送到词典",
        Tr::CustomDictAddressRow => "词典 API 地址",
        Tr::CustomDictTokenRow => "API 令牌 (可选)",
        Tr::CustomDictStatusRow => "词典连接",
        Tr::CustomDictCheckButton => "检查",
        Tr::CustomDictChecking => "正在检查…",
        Tr::CustomDictConnected => "已连接 (HTTP {0})",
        Tr::CustomDictStatusError => "错误：{0}",
        Tr::CustomDictNotConfigured => "词典 API 地址未配置（设置 → 单词）",
        Tr::CustomDictSending => "正在发送到词典…",
        Tr::CustomDictAdded => "已添加到词典",
        Tr::CustomDictError => "词典：{0}",
        Tr::CustomDictErrorNetwork => "词典无法访问：{0}",
        Tr::CustomDictErrorAuth => "词典：授权错误 (401)",
        Tr::CustomDictErrorServer => "词典：服务器错误 ({0})",
        Tr::SectionCustomDict => "自定义词典",
        Tr::DeleteWord => "删除",

        // Word card.
        Tr::CloseEscHint => "关闭（Esc）",
        Tr::Explaining => "正在结合上下文解释…",
        Tr::WordSaved => "★ 已保存",
        Tr::SaveWord => "☆ 保存单词",
        Tr::CopyWord => "复制",

        // Settings window.
        Tr::SettingsTitle => "设置",
        Tr::SectionLanguages => "语言",
        Tr::InterfaceLanguage => "界面语言",
        Tr::LangAuto => "自动",
        Tr::SectionSubtitles => "字幕",
        Tr::TextSize => "文字大小",
        Tr::TranslationSize => "翻译大小",
        Tr::AutoTranslateCheck => "没有我的语言的字幕时自动翻译",
        Tr::SectionTranslation => "翻译",
        Tr::ServiceRow => "服务",
        Tr::ModelRow => "模型",
        Tr::ModelNameHint => "模型名称",
        Tr::ServerAddressRow => "服务器地址",
        Tr::OpenAiAddressHint => "留空 = OpenAI；Ollama：http://localhost:11434/v1",
        Tr::ApiKeyRow => "API 密钥",
        Tr::ProviderGoogle => "Google 翻译（无需密钥）",
        Tr::ProviderOpenAi => "兼容 OpenAI（OpenAI、Ollama、LM Studio）",
        Tr::ProviderUnknown => "未知",
        Tr::SectionSpeechRecognition => "语音识别",
        Tr::ModelFileRow => "模型文件",
        Tr::AsrAutoCheck => "视频没有字幕时自动识别语音",
        Tr::DeckRow => "牌组",
        Tr::AnkiAddressRow => "AnkiConnect 地址",
        Tr::AnkiStatusRow => "与 Anki 的连接",
        Tr::AnkiCheckButton => "检查",
        Tr::AnkiChecking => "正在检查…",
        Tr::AnkiConnected => "已连接：Anki 有响应（AnkiConnect API 版本 {0}）。",
        Tr::AnkiStatusNotRunning => "Anki 未运行。请启动 Anki，并在发送单词时保持它打开。",
        Tr::AnkiStatusNoAddon => {
            "未找到 AnkiConnect 插件。请先安装（见下方“如何连接 Anki”），然后启动 Anki。"
        }
        Tr::AnkiStatusNoAnswer => {
            "此地址没有响应。请启动装有 AnkiConnect 插件的 Anki（见下方“如何连接 Anki”）。"
        }
        Tr::AnkiStatusError => "此地址有响应，但不像 AnkiConnect：{0}",
        Tr::AnkiHowTo => "如何连接 Anki",
        Tr::AnkiStep1 => "1. 安装 Anki（一款免费的记忆卡片软件）并启动它：",
        Tr::AnkiStep2 => {
            "2. 在 Anki 中选择“工具”→“附加组件”→“获取附加组件”，输入代码 {0}，然后点击确定。"
        }
        Tr::AnkiStep3 => "3. 重启 Anki，并在从播放器发送单词时保持它打开。",
        Tr::AnkiCopyCode => "复制代码",
        Tr::AnkiCodeCopied => "已复制",
        Tr::SectionPlayer => "播放器",
        Tr::HwdecRow => "硬件解码",
        Tr::InterfaceScaleRow => "界面缩放",
        Tr::ShellCheck => "在资源管理器的“打开方式”和右键菜单中显示 Glossplay",
        Tr::SaveButton => "保存",
        Tr::PathAutoHint => "留空则自动查找",
        Tr::BrowseButton => "浏览",

        // Messages from the app.
        Tr::NoFfmpegToast => {
            "未找到 ffmpeg —— 字幕可以显示，但不能点词查询。Windows 版安装包和压缩包已包含 ffmpeg；否则请安装 ffmpeg 或在设置中填写它的路径。"
        }
        Tr::ConfigUsingDefaults => "{0}；使用默认设置",
        Tr::DownloadingYtdlp => "正在下载 yt-dlp，视频马上开始…",
        Tr::FfmpegRequired => "语音识别需要 ffmpeg",
        Tr::StreamSoundUnreadable => "语音识别无法读取此链接的声音",
        Tr::NoDurationYet => "文件时长尚未知 —— 请稍后再试",
        Tr::AsrTrackLabel => "语音识别",
        Tr::AutoSubsLabel => "{0}（自动）",
        Tr::AutoTranslatedSubsLabel => "{0}（自动翻译）",
        Tr::LangEnglish => "英语",
        Tr::LangRussian => "俄语",
        Tr::LangUkrainian => "乌克兰语",
        Tr::LangGerman => "德语",
        Tr::LangFrench => "法语",
        Tr::LangSpanish => "西班牙语",
        Tr::LangItalian => "意大利语",
        Tr::LangPortuguese => "葡萄牙语",
        Tr::LangPolish => "波兰语",
        Tr::LangTurkish => "土耳其语",
        Tr::LangJapanese => "日语",
        Tr::LangKorean => "韩语",
        Tr::LangChinese => "中文",
        Tr::LangDutch => "荷兰语",
        Tr::LangArabic => "阿拉伯语",
        Tr::LangHindi => "印地语",
        Tr::LangIndonesian => "印尼语",
        Tr::LangVietnamese => "越南语",
        Tr::LangCzech => "捷克语",
        Tr::LangSwedish => "瑞典语",
        Tr::CouldNotReadSubsShown => "无法读取字幕“{0}”：{1} —— 字幕照常显示，但不能点词查询。",
        Tr::CouldNotLoadSubs => "无法加载字幕“{0}”：{1}",
        Tr::TranslationFailed => "翻译失败：{0}",
        Tr::YtdlpUpdated => "yt-dlp 已更新到 {0}",
        Tr::AddedToAnki => "已添加到 Anki",
        Tr::AnkiError => "Anki：{0}",
        Tr::TranslationNotSetUp => "翻译未设置：{0}（设置 → 翻译）",
        Tr::SendingToAnki => "正在发送到 Anki…",
        Tr::ExplorerIntegration => "资源管理器集成：{0}",
        Tr::OldConfigKept => "旧的 config.toml 无法解析；已将其保存为 {0}",
        Tr::CouldNotKeepConfig => "无法保存旧 config.toml 的副本：{0}",
        Tr::OpenDialogTitle => "打开视频或字幕",
        Tr::CouldNotPlay => "无法播放此文件",

        // Messages from background jobs.
        Tr::FfmpegNotFound => "未找到 ffmpeg",
        Tr::AnkiNotRunning => {
            "Anki 没有响应：请启动装有 AnkiConnect 插件的 Anki（设置 → “单词与 Anki” → “如何连接 Anki”）"
        }
        Tr::AlreadyInDeck => "这个单词已在牌组中",
        Tr::AsrStopped => "语音识别已停止：{0}",
        Tr::AsrStarting => "正在启动语音识别…",
        Tr::CannotStartWhisper => "无法启动 whisper-server：{0}",
        Tr::WhisperQuitLoading => "whisper-server 在加载时退出；请检查模型路径",
        Tr::WhisperNotReady => "whisper-server 未能就绪；请检查模型路径",
        Tr::TranscriptReady => "字幕文本已就绪",
        Tr::Transcribing => "正在识别 {0}%{1}",
        Tr::CouldNotReadAudio => "无法读取音频：{0}",
        Tr::WhisperStopped => "whisper-server 不再响应",
        Tr::WhisperFailed => "whisper-server 出错：{0}",
        Tr::WhisperQuitTimes => "whisper-server 意外退出了 {0} 次",
        Tr::YtdlpNoGithub => "无法连接 GitHub 下载 yt-dlp：{0}",
        Tr::YtdlpDownloadFailed => "无法下载 yt-dlp：{0}",
        Tr::YtdlpBadChecksum => "下载的 yt-dlp 与校验和不符；不予安装",
        Tr::YtdlpChecksumsFailed => "无法下载 yt-dlp 的校验和：{0}",
        Tr::YtdlpNoChecksum => "校验和列表中没有 {0}",

        // Startup error window.
        Tr::StartupFailed => "Glossplay 无法启动",
        Tr::StartupMpvHint => {
            "请把 mpv 库（libmpv）放在程序旁边，或在 config.toml 中设置 player.mpv_path："
        }

        // Explorer menu entry.
        Tr::ShellPlayWith => "用 Glossplay 播放",

        // Installer errors and units.
        Tr::UnitMb => "{0} MB",
        Tr::UnitGb => "{0} GB",
        Tr::PerSecond => "{0}/秒",
        Tr::Cancelled => "已取消。已下载的部分会保留，下次安装将从那里继续。",
        Tr::NotInstalled => "此组件未安装",
        Tr::AlreadyInstalling => "此组件正在安装中",
        Tr::UnsafePath => "压缩包中有文件会被解压到安装文件夹之外；不予安装",
        Tr::DiskFull => "磁盘已满：无法写入 {0}",
        Tr::CannotWrite => "无法写入 {0}：{1}",
        Tr::CannotRead => "无法读取 {0}：{1}",
        Tr::CannotRemove => "无法删除该组件：{0}。请关闭 Glossplay 和所有 whisper-server 后重试。",
        Tr::InUse => "无法替换 {0}：{1}。可能有另一个 Glossplay 窗口正在使用它；请关闭后重试。",
        Tr::NotEnoughSpace => {
            "磁盘空间不足：安装需要 {1} 所在的磁盘上有 {0} 可用空间，而现在只有 {2}。"
        }
        Tr::DownloadFailed => "无法下载{0}：{1}。请稍后重试；下载会从中断处继续。",
        Tr::DamagedDownload => "下载的{0}已损坏（校验和不符），已被删除。请重试。",
        Tr::BadArchive => "无法解压 {0}：{1}",
        Tr::ArchiveEmpty => "{0} 中没有所需的文件",
        Tr::ArchiveLacks => "{0} 中缺少 {1}",

        // Speech recognition setup.
        Tr::ComponentName => "语音识别（Whisper）",
        Tr::LicenceNote => {
            "whisper.cpp 和 Whisper 模型采用 MIT 许可（ggml-org、OpenAI）；Silero 语音检测器采用 MIT 许可。NVIDIA 版本包含 NVIDIA CUDA 库，按 NVIDIA CUDA 许可再分发。"
        }
        Tr::EngineLabelNvidia => "适用于 NVIDIA 显卡的 whisper.cpp 引擎",
        Tr::EngineLabelCpu => "适用于处理器的 whisper.cpp 引擎",
        Tr::VadLabel => "Silero 语音检测器",
        Tr::ModelLabel => "Whisper 模型 {0}",
        Tr::EngineAuto => "自动",
        Tr::EngineNvidia => "NVIDIA 显卡",
        Tr::EngineCpu => "处理器（无显卡）",
        Tr::EngineAutoIs => "{0}（{1}）",
        Tr::BuildNvidia => "NVIDIA 版本",
        Tr::BuildCpu => "处理器版本",
        Tr::ModelAuto => "推荐",
        Tr::ModelHintTurbo => "最准确",
        Tr::ModelHintTurboQ5 => "准确，更小",
        Tr::ModelHintSmall => "不错，任何电脑都能用",
        Tr::ModelHintBase => "最快，错误较多",
        Tr::DialogTitle => "语音识别",
        Tr::DialogIntro => {
            "Glossplay 可以为没有字幕的视频把语音转成字幕。Whisper 在这台电脑上运行，不会向任何地方发送内容。需要一次性下载。"
        }
        Tr::CheckingHardware => "正在检查显卡…",
        Tr::NoNvidia => "未找到 NVIDIA 显卡",
        Tr::NvidiaForcedWithoutGpu => "未找到 NVIDIA 显卡；没有它，NVIDIA 版本无法启动。",
        Tr::CpuSlow => {
            "没有 NVIDIA 显卡时识别较慢：大约和视频播放一样快。请在观看前先让它提前识别一段。"
        }
        Tr::OldDriver => {
            "NVIDIA 驱动版本过旧，无法使用 NVIDIA 版本（需要 525 或更新版本），因此建议使用处理器版本。更新驱动后即可使用显卡。"
        }
        Tr::NoEngineHere => {
            "此系统没有现成的 whisper.cpp：请自行安装 whisper.cpp，并在“高级”中填写 whisper-server 的路径。模型仍会在这里下载。"
        }
        Tr::VcRuntimeMissing => {
            "whisper.cpp 需要 Microsoft Visual C++ 运行库，但尚未安装。请从 https://aka.ms/vs/17/release/vc_redist.x64.exe 获取"
        }
        Tr::RowEngine => "引擎",
        Tr::RowModel => "模型",
        Tr::RowTotal => "下载",
        Tr::RowPlace => "安装到",
        Tr::EngineOwn => "你自己的 whisper.cpp",
        Tr::Install => "安装",
        Tr::Reinstall => "重新安装",
        Tr::Remove => "删除",
        Tr::Cancel => "取消",
        Tr::Close => "关闭",
        Tr::TryAgain => "重试",
        Tr::Ready => "完成。语音识别已安装。",
        Tr::ReadyStarted => "现在正在识别这个视频。",
        Tr::ReadyHow => "打开一个没有字幕的视频，然后选择 字幕 → 识别语音。",
        Tr::NotInstalledStatus => "未安装",
        Tr::Advanced => "高级：使用我自己的 whisper.cpp",
        Tr::SetUpLink => "为没有字幕的视频设置语音识别…",
        Tr::ToastReady => "语音识别已安装",
        Tr::ToastRemoved => "语音识别已删除",
        Tr::NotSetUp => "语音识别未设置：选择 字幕 → 识别语音 即可安装",
        Tr::NoServer => "未找到 whisper-server：请在 设置 → 语音识别 → 高级 中填写它的路径",
        Tr::NoModel => "未找到 Whisper 模型文件：请在 设置 → 语音识别 → 高级 中填写它的路径",
        Tr::FirstRun => "在这块显卡上首次运行可能需要一分钟",
        Tr::SpaceLine => "占用磁盘 {0}；安装时需要 {1} 可用空间",
        Tr::SpaceLineFree => "占用磁盘 {0}；安装时需要 {1} 可用空间（当前可用 {2}）",
        Tr::GpuLine => "显卡：{0}，{1} GB",
        Tr::InstalledStatus => "已安装：{0}，{1}（{2}）",
        Tr::InstalledModelsOnly => "已安装：模型 {0}（引擎是你自己的 whisper.cpp）",
        Tr::StagePreparing => "正在准备…",
        Tr::StageDownloading => "正在下载 {0}",
        Tr::StageVerifying => "正在校验 {0}",
        Tr::StageUnpacking => "正在解压 {0}",
        Tr::StageFinishing => "即将完成…",
        Tr::FileLine => "{0}：{1} / {2}",

        // Voice-over.
        Tr::TtsComponentName => "配音（OmniVoice）",
        Tr::TtsLabelVulkan => "适用于任何显卡的 audio.cpp 语音引擎（Vulkan）",
        Tr::TtsLabelNvidia => "适用于 NVIDIA 显卡的 audio.cpp 语音引擎（CUDA）",
        Tr::TtsLabelCudaLibs => "语音引擎所需的 NVIDIA CUDA 库",
        Tr::TtsLabelModel => "OmniVoice 语音模型（q8_0）",
        Tr::TtsDialogTitle => "配音",
        Tr::TtsDialogIntro => {
            "Glossplay 可以在视频上朗读翻译，同时调低原声。语音由 OmniVoice 在这台电脑上生成，不会向任何地方发送内容。需要一次性下载和一块显卡。"
        }
        Tr::TtsSources => {
            "从 GitHub（0xShug0/audio.cpp，v0.9.5 版）和 Hugging Face（audio-cpp/audio.cpp-gguf）下载。"
        }
        Tr::TtsLicenceNc => {
            "仅限非商业用途。OmniVoice 语音模型（k2-fsa/OmniVoice）采用 CC BY-NC 4.0 许可：模型本身及其生成的语音均不得用于商业目的。"
        }
        Tr::TtsLicenceBoson => {
            "Built with Higgs Materials licensed from Boson AI USA, Inc.（模型内部的音频分词器，采用 Boson Higgs Audio 2 Community License）。"
        }
        Tr::TtsNoImpersonation => "未经本人同意，不得用语音引擎模仿真人的声音。",
        Tr::TtsLicenceEngine => {
            "audio.cpp 引擎采用 Apache-2.0 许可（ShugoAI）。NVIDIA 版本包含 NVIDIA CUDA 库，按 NVIDIA CUDA 许可再分发。"
        }
        Tr::TtsAgree => "我只将配音用于非商业目的，并接受这些许可",
        Tr::TtsNeedsGpu => "语音引擎需要显卡。只用处理器时，它比视频播放慢 6–10 倍，跟不上。",
        Tr::TtsNoVulkan => {
            "这台电脑上没有找到 Vulkan 显卡驱动，语音引擎将在处理器上运行：比视频播放慢 6–10 倍。请先安装显卡驱动。"
        }
        Tr::TtsNvidiaWithoutGpu => "未找到 NVIDIA 显卡；没有它，NVIDIA 版本无法启动。",
        Tr::TtsEngineVulkan => "任何显卡（Vulkan）",
        Tr::TtsEngineNvidia => "NVIDIA 显卡（CUDA）：稍快，但大得多",
        Tr::TtsBuildVulkan => "Vulkan 版本",
        Tr::TtsBuildNvidia => "NVIDIA 版本",
        Tr::TtsReady => "完成。配音已安装。",
        Tr::TtsReadyOn => "配音现已开启。",
        Tr::TtsReadyHow => "在 字幕 → 配音 中开启。",
        Tr::TtsToastReady => "配音已安装",
        Tr::TtsToastRemoved => "配音已删除",
        Tr::TtsNotSetUp => "配音未设置：选择 字幕 → 配音 即可安装",
        Tr::TtsNoEngineHere => {
            "此系统没有现成的语音引擎：请自行运行一个 OpenAI 风格的语音服务器，并在“高级”中填写它的地址。"
        }
        Tr::TtsNeedsTranslation => "配音朗读的是翻译：请在 字幕 → 翻译 中选择一个",
        Tr::TtsInstalledStatus => "已安装：{0}，OmniVoice q8_0（{1}）",
        Tr::MenuVoiceOver => "配音（朗读翻译）",
        Tr::MenuVoiceOverHint => "在视频上朗读翻译，并调低原声",
        Tr::SliderOriginal => "原声",
        Tr::SliderVoice => "配音",
        Tr::SectionVoiceOver => "翻译配音",
        Tr::TtsEnable => "在视频上朗读翻译",
        Tr::RowOriginalVolume => "原声，%",
        Tr::RowVoiceVolume => "配音，%",
        Tr::RowVoiceSource => "声音",
        Tr::VoiceSpeaker => "模仿说话人",
        Tr::VoiceStandard => "标准",
        Tr::VoiceSpeakerNote => {
            "说话人的声音只在这台电脑上从视频中的几秒钟复制，仅供你自己观看，不会发送到任何地方。"
        }
        Tr::DubVoiceSample => "配音：正在听取说话人的声音…",
        Tr::TtsAdvanced => "高级：我自己的语音服务器",
        Tr::RowServer => "服务器地址",
        Tr::HintServer => "留空 = 已安装的引擎；http://host:port/v1",
        Tr::RowTtsModel => "模型",
        Tr::RowVoice => "声音",
        Tr::HintVoice => "服务器默认",
        Tr::TtsLanguage => "朗读语言：{0}（“语言”标签页中的“我的语言”）",
        Tr::DubStarting => "配音：正在启动语音引擎…",
        Tr::DubFirstStart => "在这块显卡上首次启动最多需要半分钟",
        Tr::DubPreparing => "配音：正在准备…",
        Tr::DubWaiting => "正在准备配音",
        Tr::DubOn => "配音",
        Tr::DubStopped => "配音已停止：{0}",
        Tr::DubLineFailed => "配音：有一句无法朗读（{0}）",
        Tr::DubEngineGone => "语音引擎没有启动；请在设置中检查安装",
        Tr::DubEngineQuit => "语音引擎意外退出了 {0} 次",
        Tr::DubCannotStart => "无法启动语音引擎：{0}",
        Tr::DubNoAudioOut => "无法为配音打开第二个音频播放器",

        // Search.
        Tr::SearchVideosHint => "搜索视频（Ctrl+F）",
        Tr::SearchMenu => "搜索…",
        Tr::SearchField => "搜索视频",
        Tr::SearchButton => "搜索",
        Tr::SearchTitle => "在你的来源中搜索",
        Tr::ShortcutsSearch => "Ctrl+F 在你的来源中搜索",
        Tr::Searching => "正在搜索…",
        Tr::NothingFound => "什么也没找到",
        Tr::FoundCount => "找到：{0} · 双击或按 Enter 播放",
        Tr::ColTitle => "名称",
        Tr::ColSize => "大小",
        Tr::ColPeers => "做种 / 下载",
        Tr::ColDate => "日期",
        Tr::ColSource => "来源",
        Tr::GettingFiles => "正在获取文件列表…",
        Tr::ChooseFile => "视频文件：{0} · 双击或按 Enter 播放",
        Tr::BackToResults => "← 结果",
        Tr::NoVideoFiles => "这个种子里没有视频文件",
        Tr::SearchUnreachable => "搜索服务器没有响应。它在运行吗？地址正确吗？（{0}）",
        Tr::SearchBadKey => "搜索服务器不接受这个 API 密钥；请在 设置 → 来源 中检查",
        Tr::SearchWrongAddress => "这个地址上没有搜索服务器（HTTP {0}）；请在 设置 → 来源 中检查",
        Tr::SearchFailed => "搜索服务器返回了错误：{0}",
        Tr::LinkFailed => "无法从搜索服务器获取种子：{0}",
        Tr::TorrServerUnreachable => "TorrServer 没有响应。它在运行吗？地址正确吗？（{0}）",
        Tr::NoMetadata => "30 秒内没有节点发来文件列表：这个种子可能没有做种者",
        Tr::TorrServerFailed => "TorrServer 返回了错误：{0}",
        Tr::SectionSources => "来源",
        Tr::SourcesNote => {
            "Glossplay 不含任何来源。请填写你自己的搜索服务器（Jackett、Prowlarr —— Torznab 协议）和 TorrServer 的地址。"
        }
        Tr::SearchServerRow => "搜索服务器（Torznab）",
        Tr::TorrServerRow => "TorrServer",
        Tr::CheckButton => "检查",
        Tr::Checking => "正在检查…",
        Tr::CheckSearchOk => "搜索服务器：找到 {0} 个来源",
        Tr::CheckSearchFailed => "搜索服务器：{0}",
        Tr::CheckTorrServerOk => "TorrServer {0}",
        Tr::CheckTorrServerFailed => "TorrServer：{0}",
        Tr::CheckNotSet => "地址为空",
        Tr::PasteNothing => "剪贴板中没有链接或文件路径",
        Tr::PasteNeedsTorrServer => "要播放 magnet 链接，请在 设置 → 来源 中填写 TorrServer 地址",

        // Settings tabs and How to use.
        Tr::SectionWords => "单词与 Anki",
        Tr::IntroLanguages => "上面是视频语言的字幕，下面是翻译成你的语言的字幕。",
        Tr::IntroSubtitles => "视频上两行字幕的样子。观看时，{0} 和 {1} 可以把字幕变小或变大。",
        Tr::IntroTranslation => {
            "把字幕翻译成你的语言。Google 翻译无需密钥；DeepL、OpenAI 和 Claude 需要 API 密钥。"
        }
        Tr::IntroSpeech => {
            "视频没有字幕时，把语音转成字幕。一次性下载后在这台电脑上运行。观看时：字幕菜单 → “{0}”。"
        }
        Tr::IntroVoice => "在视频上朗读翻译，并调低原声。观看时：字幕菜单 → “{0}”。",
        Tr::IntroWords => {
            "点击字幕中的单词即可看到翻译。已存的单词在侧边栏中（{0}）；单词卡应用 Anki 可以把它们做成卡片。"
        }
        Tr::IntroPlayer => "窗口的外观和视频的播放方式。默认设置适合大多数电脑。",
        Tr::RowVideoLang => "视频语言",
        Tr::RowMyLang => "我的语言",
        Tr::LangDetect => "按每个视频自动判断",
        Tr::InfoLearn => "你正在学的语言。它决定哪条字幕放在上面，以及语音识别听哪种语言。",
        Tr::InfoNative => "翻译、配音和单词解释都使用这种语言。",
        Tr::InfoInterface => "菜单和按钮的语言。",
        Tr::InfoAutoTranslate => "视频没有你的语言的字幕时，由翻译服务生成。",
        Tr::InfoApiKey => "从服务网站获取的密钥。它保存在这台电脑的设置文件中。",
        Tr::InfoAsrAuto => "视频完全没有字幕时，自动开始语音识别。",
        Tr::InfoVolumes => {
            "配音朗读时：影片原声保留多大音量、配音多大音量，以百分比表示。字幕菜单里也有这两个滑块。"
        }
        Tr::InfoAnki => "Anki 需要安装 AnkiConnect 插件并保持运行。已存的单词会进入这个牌组。",
        Tr::InfoHwdec => "让显卡解码视频。留空或填写“auto”；如果画面出现异常，填写“no”。",
        Tr::InfoFfmpeg => "ffmpeg 用来读取视频文件内的字幕，以及语音识别所需的声音。",
        Tr::InfoScale => "放大或缩小所有文字和按钮。点击保存后生效。",
        Tr::AsrListensFor => "识别语言：{0}（视频语言）",
        Tr::AsrListensAny => "识别任何语言，并自动判断是哪一种",
        Tr::ChangeLanguage => "更改…",
        Tr::TtsLangUnknown => "语音引擎可能不认识这种语言。",
        Tr::SettingsOf => "设置：{0}",
        Tr::HelpTitle => "使用说明",
        Tr::HelpHint => "如何使用 Glossplay",
        Tr::HelpMenu => "使用说明…",
        Tr::HelpOpenTitle => "打开视频",
        Tr::HelpOpenText => {
            "按“{0}”，把文件拖进窗口，或把视频页面的链接粘贴到输入框中。复制链接后按 Ctrl+V 也可以。"
        }
        Tr::HelpSubsTitle => "两行字幕",
        Tr::HelpSubsText => {
            "上面：视频的语言。下面：翻译成你的语言。在设置中把两种语言设好一次即可；其他字幕轨在“{0}”菜单中。"
        }
        Tr::HelpAsrTitle => "没有字幕？",
        Tr::HelpAsrText => {
            "“{0}”菜单 → “{1}”：播放器会听取语音并写成字幕。第一次会提示一次性下载。"
        }
        Tr::HelpVoiceTitle => "听翻译",
        Tr::HelpVoiceText => {
            "“{0}”菜单 → “{1}”：翻译会在视频上被朗读出来。下面会出现两个滑块：“{2}”是影片的声音，“{3}”是配音。"
        }
        Tr::HelpWordsTitle => "查单词",
        Tr::HelpWordsText => {
            "点击字幕中的单词即可看到翻译。“{0}”会把它存进你的列表（{1}），之后可以发送到 Anki。"
        }
        Tr::HelpKeysTitle => "快捷键",
        Tr::KeySpace => "空格",
        Tr::KeyPlayPause => "播放或暂停",
        Tr::KeySeek => "后退或前进 5 秒",
        Tr::KeyVolume => "调大或调小音量",
        Tr::KeyLines => "上一句、重复这一句、下一句",
        Tr::KeyToggleLines => "隐藏或显示原文或翻译",
        Tr::KeyAutoPause => "每句之后暂停：开或关",
        Tr::KeyFullscreen => "全屏",
        Tr::KeySidebar => "字幕文本和已存单词",
        Tr::KeyMute => "关闭或打开声音",
        Tr::KeySpeed => "减速或加速",
        Tr::KeySubSize => "缩小或放大字幕",
        Tr::KeyOpenFile => "打开文件",
        Tr::KeySearch => "搜索视频",
        Tr::KeyClose => "关闭窗口",

        // Buffering, the context menu, saved frames and the video info window.
        Tr::Buffering => "缓冲中",
        Tr::PercentValue => "{0}%",
        Tr::SpeedKb => "{0} KB/s",
        Tr::SpeedMb => "{0} MB/s",
        Tr::MenuPlay => "播放",
        Tr::MenuPause => "暂停",
        Tr::MenuSpeed => "速度",
        Tr::MenuRepeatLine => "重复这一句",
        Tr::MenuOpenClipboard => "打开复制的链接",
        Tr::KeyCtrlV => "Ctrl+V",
        Tr::MenuAudioTrack => "音轨",
        Tr::MenuVideo => "画面",
        Tr::MenuAspect => "宽高比",
        Tr::MenuAspectAuto => "与文件相同",
        Tr::MenuRotate => "旋转 90°",
        Tr::MenuFill => "填满窗口",
        Tr::MenuDeinterlace => "去隔行",
        Tr::MenuVideoReset => "重置",
        Tr::MenuFrame => "截取画面",
        Tr::MenuSaveFrame => "保存画面",
        Tr::MenuSaveFrameSubs => "保存带字幕的画面",
        Tr::MenuCopyFrame => "复制画面",
        Tr::MenuCopyTime => "复制时间（{0}）",
        Tr::MenuCopyLink => "复制此时刻的链接",
        Tr::MenuFullscreen => "全屏",
        Tr::MenuOnTop => "窗口置顶",
        Tr::FrameSaved => "画面已保存：{0}",
        Tr::FrameNotSaved => "无法保存画面：{0}",
        Tr::FrameCopied => "画面已复制",
        Tr::ShowInFolder => "在文件夹中显示",
        Tr::Copied => "已复制：{0}",
        Tr::LinkCopied => "链接已复制",
        Tr::FramesFolderRow => "画面保存文件夹",
        Tr::InfoFramesFolder => {
            "画面右键菜单中的“保存画面”把图片存放在这里。留空：“图片”中的 Glossplay 文件夹。"
        }
        Tr::KeyVideoInfo => "视频信息",
        Tr::InfoWindowTitle => "视频信息",
        Tr::InfoCopyAll => "全部复制",
        Tr::InfoCopied => "视频信息已复制",
        Tr::InfoFile => "文件",
        Tr::InfoTitleRow => "标题",
        Tr::InfoSource => "来源",
        Tr::InfoContainer => "格式",
        Tr::InfoDuration => "时长",
        Tr::InfoSize => "大小",
        Tr::InfoVideo => "视频",
        Tr::InfoCodec => "编码",
        Tr::InfoResolution => "分辨率",
        Tr::InfoFps => "帧率",
        Tr::InfoBitrate => "码率",
        Tr::InfoPixelFormat => "像素格式",
        Tr::InfoColour => "色彩",
        Tr::InfoSoftware => "否（软件解码）",
        Tr::InfoAudio => "声音",
        Tr::InfoTrack => "音轨",
        Tr::InfoChannels => "声道",
        Tr::InfoSampleRate => "采样率",
        Tr::InfoSubtitles => "字幕",
        Tr::InfoSecondLine => "下方一行",
        Tr::InfoPlayback => "播放",
        Tr::InfoDropped => "丢弃的帧",
        Tr::InfoDecoderDropped => "解码器丢弃的帧",
        Tr::InfoMistimed => "延迟显示的帧",
        Tr::InfoOutputFps => "输出帧率",
        Tr::InfoNetwork => "网络",
        Tr::InfoCacheState => "状态",
        Tr::InfoCacheOk => "正在播放",
        Tr::InfoBuffered => "已预先缓冲",
        Tr::InfoBufferedBytes => "缓冲区中",
        Tr::InfoSpeed => "下载速度",
        Tr::ChangeFolder => "更改…",
        Tr::ComponentsFolderRow => "下载文件夹",
        Tr::InfoComponentsFolder => {
            "语音识别和配音引擎及其模型下载到这里，它们占用数 GB。留空：程序的数据文件夹。已安装的内容保留在原处，照常使用。"
        }
        Tr::FreeOnDisk => "此磁盘可用空间：{0}",
        Tr::UnitSeconds => "{0} 秒",
        Tr::UnitFps => "{0} 帧/秒",
        Tr::UnitKbps => "{0} kbit/s",
        Tr::UnitMbps => "{0} Mbit/s",
        Tr::UnitKhz => "{0} kHz",
        Tr::UnitKb => "{0} KB",
    }
}
