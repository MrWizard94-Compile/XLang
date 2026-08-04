<#
.SYNOPSIS
  Stage a local Aether technical preview package under dist/.

.DESCRIPTION
  Builds release CLI (unless -SkipBuild), copies binary, seed, schemas,
  examples, notes, verify script, and writes SHA-256SUMS.
  REL-PACKAGE-001 / REL-DETERM-001.

.PARAMETER SkipBuild
  Reuse existing target/release/aether.exe
#>
[CmdletBinding()]
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

$Version = "0.16.0"
$PkgName = "aether-$Version-tp"
$DistRoot = Join-Path $RepoRoot "dist"
$Pkg = Join-Path $DistRoot $PkgName

function Fail([string]$m) {
    Write-Host "FAIL: $m" -ForegroundColor Red
    exit 1
}

if (-not $SkipBuild) {
    Write-Host "=== cargo build --release -p aether-cli ===" -ForegroundColor Cyan
    cargo build --release -p aether-cli
    if ($LASTEXITCODE -ne 0) { Fail "release build failed" }
}

$exeSrc = Join-Path $RepoRoot "target\release\aether.exe"
if (-not (Test-Path -LiteralPath $exeSrc)) {
    Fail "missing $exeSrc — run without -SkipBuild or build first"
}

if (Test-Path -LiteralPath $Pkg) {
    Remove-Item -LiteralPath $Pkg -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $Pkg | Out-Null

Write-Host "=== staging $Pkg ===" -ForegroundColor Cyan
Copy-Item -LiteralPath $exeSrc -Destination (Join-Path $Pkg "aether.exe")

New-Item -ItemType Directory -Force -Path (Join-Path $Pkg "seed") | Out-Null
Copy-Item -LiteralPath (Join-Path $RepoRoot "seed\aether_seed.aeth") `
    -Destination (Join-Path $Pkg "seed\aether_seed.aeth")

New-Item -ItemType Directory -Force -Path (Join-Path $Pkg "schemas") | Out-Null
Copy-Item -Path (Join-Path $RepoRoot "schemas\*") -Destination (Join-Path $Pkg "schemas") -Recurse

New-Item -ItemType Directory -Force -Path (Join-Path $Pkg "examples") | Out-Null
Copy-Item -Path (Join-Path $RepoRoot "examples\*") -Destination (Join-Path $Pkg "examples") -Recurse

foreach ($doc in @(
        "docs\RELEASE_NOTES-TECHNICAL-PREVIEW.md",
        "docs\CHANGELOG-0.12.md",
        "docs\THREAT_MODEL-TECHNICAL-PREVIEW.md",
        "MANIFEST.md"
    )) {
    $src = Join-Path $RepoRoot $doc
    if (-not (Test-Path -LiteralPath $src)) { Fail "missing $doc" }
    Copy-Item -LiteralPath $src -Destination (Join-Path $Pkg (Split-Path $doc -Leaf))
}

$verifySrc = Join-Path $RepoRoot "tools\verify-preview.ps1"
if (-not (Test-Path -LiteralPath $verifySrc)) { Fail "missing tools/verify-preview.ps1" }
Copy-Item -LiteralPath $verifySrc -Destination (Join-Path $Pkg "verify-preview.ps1")

# SHA-256SUMS (paths relative to package root, forward slashes)
Write-Host "=== SHA-256SUMS ===" -ForegroundColor Cyan
$sumPath = Join-Path $Pkg "SHA-256SUMS"
$lines = New-Object System.Collections.Generic.List[string]
Get-ChildItem -LiteralPath $Pkg -Recurse -File |
    Where-Object { $_.Name -ne "SHA-256SUMS" } |
    Sort-Object FullName |
    ForEach-Object {
        $rel = $_.FullName.Substring($Pkg.Length).TrimStart("\", "/")
        $rel = $rel -replace "\\", "/"
        $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash.ToLowerInvariant()
        $lines.Add("$hash  $rel")
    }
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllLines($sumPath, $lines, $utf8NoBom)

Write-Host "Package ready: $Pkg"
Write-Host "Files:" ($lines.Count)
Get-Content -LiteralPath $sumPath | Select-Object -First 5
Write-Host "..."
exit 0
