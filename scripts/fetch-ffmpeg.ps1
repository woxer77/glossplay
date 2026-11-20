# Downloads the pinned ffmpeg.exe build (see ffmpeg-version.ps1), checks its SHA-256 and puts
# it in an "ffmpeg" folder next to the built program, where Glossplay looks for it.
# Set GLOSSPLAY_CACHE_DIR to keep the downloaded archive somewhere other than %TEMP%.
param(
    [string[]]$Destination
)
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
. (Join-Path $PSScriptRoot 'ffmpeg-version.ps1')

if (-not $Destination) {
    $target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $PSScriptRoot '..\target' }
    $Destination = @((Join-Path $target 'debug'), (Join-Path $target 'release'))
}

$cacheRoot = if ($env:GLOSSPLAY_CACHE_DIR) { $env:GLOSSPLAY_CACHE_DIR } else { [IO.Path]::GetTempPath() }
$work = Join-Path $cacheRoot "glossplay-ffmpeg-$FfmpegVersion"
New-Item -ItemType Directory -Force $work | Out-Null
$archive = Join-Path $work $FfmpegArchiveName

function Test-Sha256($path, $expected) {
    (Get-FileHash $path -Algorithm SHA256).Hash -eq $expected
}

# A cached archive that fails the check is downloaded again rather than trusted.
if ((Test-Path $archive) -and -not (Test-Sha256 $archive $FfmpegArchiveSha256)) {
    Remove-Item $archive -Force
}
if (-not (Test-Path $archive)) {
    Write-Host "Downloading $FfmpegArchiveName"
    Invoke-WebRequest $FfmpegArchiveUrl -OutFile $archive
}
$actual = (Get-FileHash $archive -Algorithm SHA256).Hash
if ($actual -ne $FfmpegArchiveSha256) {
    Remove-Item $archive -Force
    throw "SHA-256 of $FfmpegArchiveName does not match the pinned value. Expected $FfmpegArchiveSha256, got $actual. Nothing was installed."
}

# Unpack into a fresh folder so a stale exe from an earlier run cannot pass the check below.
$unpacked = Join-Path $work ('unpacked-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force $unpacked | Out-Null
try {
    $member = "ffmpeg-$FfmpegVersion-essentials_build/bin/ffmpeg.exe"
    $7z = @('7z.exe', 'C:\Program Files\7-Zip\7z.exe', "$env:ProgramFiles\7-Zip\7z.exe") | Where-Object { (Get-Command $_ -ErrorAction SilentlyContinue) -or (Test-Path $_) } | Select-Object -First 1
    if ($7z) {
        New-Item -ItemType Directory -Force (Split-Path (Join-Path $unpacked $member) -Parent) | Out-Null
        & $7z e $archive "-o$unpacked\ffmpeg-$FfmpegVersion-essentials_build\bin" ffmpeg.exe -r -y
    } else {
        & "$env:SystemRoot\System32\tar.exe" -xf $archive -C $unpacked $member
    }
    if ($LASTEXITCODE -ne 0) { throw 'Could not unpack the archive.' }
    $exe = Join-Path $unpacked $member
    $actual = (Get-FileHash $exe -Algorithm SHA256).Hash
    if ($actual -ne $FfmpegExeSha256) {
        throw "SHA-256 of ffmpeg.exe does not match the pinned value. Expected $FfmpegExeSha256, got $actual. Nothing was installed."
    }
    foreach ($dir in $Destination) {
        $into = Join-Path $dir 'ffmpeg'
        New-Item -ItemType Directory -Force $into | Out-Null
        Copy-Item $exe $into -Force
        Write-Host "ffmpeg.exe -> $into"
    }
}
finally {
    Remove-Item $unpacked -Recurse -Force -ErrorAction SilentlyContinue
}
