# Downloads the pinned libmpv-2.dll build (see mpv-version.ps1), checks its SHA-256 and puts
# it next to the built program, where Glossplay looks for it first.
param(
    [string[]]$Destination
)
$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
. (Join-Path $PSScriptRoot 'mpv-version.ps1')

if (-not $Destination) {
    $target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $PSScriptRoot '..\target' }
    $Destination = @((Join-Path $target 'debug'), (Join-Path $target 'release'))
}

$work = Join-Path ([IO.Path]::GetTempPath()) "glossplay-mpv-$MpvBuildTag"
New-Item -ItemType Directory -Force $work | Out-Null
$archive = Join-Path $work $MpvArchiveName

function Test-Sha256($path, $expected) {
    (Get-FileHash $path -Algorithm SHA256).Hash -eq $expected
}

# A cached archive that fails the check is downloaded again rather than trusted.
if ((Test-Path $archive) -and -not (Test-Sha256 $archive $MpvArchiveSha256)) {
    Remove-Item $archive -Force
}
if (-not (Test-Path $archive)) {
    Write-Host "Downloading $MpvArchiveName"
    Invoke-WebRequest $MpvArchiveUrl -OutFile $archive
}
$actual = (Get-FileHash $archive -Algorithm SHA256).Hash
if ($actual -ne $MpvArchiveSha256) {
    Remove-Item $archive -Force
    throw "SHA-256 of $MpvArchiveName does not match the pinned value. Expected $MpvArchiveSha256, got $actual. Nothing was installed."
}

# Unpack into a fresh folder so a stale DLL from an earlier run cannot pass the check below.
$unpacked = Join-Path $work ('unpacked-' + [Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Force $unpacked | Out-Null
try {
    $7z = @('7z.exe', 'C:\Program Files\7-Zip\7z.exe', "$env:ProgramFiles\7-Zip\7z.exe") | Where-Object { (Get-Command $_ -ErrorAction SilentlyContinue) -or (Test-Path $_) } | Select-Object -First 1
    if ($7z) {
        & $7z e $archive "-o$unpacked" libmpv-2.dll -y
    } else {
        & "$env:SystemRoot\System32\tar.exe" -xf $archive -C $unpacked libmpv-2.dll
    }
    if ($LASTEXITCODE -ne 0) { throw 'Could not unpack the archive.' }
    $dll = Join-Path $unpacked 'libmpv-2.dll'
    $actual = (Get-FileHash $dll -Algorithm SHA256).Hash
    if ($actual -ne $MpvDllSha256) {
        throw "SHA-256 of libmpv-2.dll does not match the pinned value. Expected $MpvDllSha256, got $actual. Nothing was installed."
    }
    foreach ($dir in $Destination) {
        New-Item -ItemType Directory -Force $dir | Out-Null
        Copy-Item $dll $dir -Force
        Write-Host "libmpv-2.dll -> $dir"
    }
}
finally {
    Remove-Item $unpacked -Recurse -Force -ErrorAction SilentlyContinue
}
