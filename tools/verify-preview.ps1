<#
.SYNOPSIS
  Consumer verification for an Aether technical preview package.

.DESCRIPTION
  Run from the package root (directory containing aether.exe and SHA-256SUMS)
  or from the monorepo (uses dist/aether-0.12.0-tp when present).

  Checks: checksums, version, welcome, host-pilot program exit 48, project verify,
  and negative path-escape project document.
#>
[CmdletBinding()]
param(
    [string]$PackageRoot = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Fail([string]$m) {
    Write-Host "FAIL: $m" -ForegroundColor Red
    exit 1
}

function Resolve-PackageRoot {
    if ($PackageRoot -ne "") {
        return (Resolve-Path -LiteralPath $PackageRoot).Path
    }
    $here = Get-Location
    if (Test-Path -LiteralPath (Join-Path $here "aether.exe")) {
        return (Resolve-Path -LiteralPath $here).Path
    }
    $scriptDir = if ($PSScriptRoot) { $PSScriptRoot } else { $here }
    $candidate = Join-Path $scriptDir "aether.exe"
    if (Test-Path -LiteralPath $candidate) {
        return (Resolve-Path -LiteralPath $scriptDir).Path
    }
    # monorepo tools/ → dist package
    foreach ($name in @("aether-0.18.0-tp", "aether-0.17.0-tp", "aether-0.16.0-tp", "aether-0.15.0-tp", "aether-0.14.0-tp", "aether-0.13.0-tp", "aether-0.12.0-tp")) {
        $fromTools = Join-Path $scriptDir "..\dist\$name"
        if (Test-Path -LiteralPath (Join-Path $fromTools "aether.exe")) {
            return (Resolve-Path -LiteralPath $fromTools).Path
        }
    }
    Fail "Could not locate package root (aether.exe). Pass -PackageRoot."
}

$Pkg = Resolve-PackageRoot
Set-Location $Pkg
Write-Host "Package root: $Pkg"

$exe = Join-Path $Pkg "aether.exe"
if (-not (Test-Path -LiteralPath $exe)) { Fail "missing aether.exe" }

# --- Checksums ---
Write-Host "=== SHA-256SUMS ===" -ForegroundColor Cyan
$sumFile = Join-Path $Pkg "SHA-256SUMS"
if (-not (Test-Path -LiteralPath $sumFile)) { Fail "missing SHA-256SUMS" }
$sumLines = Get-Content -LiteralPath $sumFile | Where-Object { $_.Trim() -ne "" }
foreach ($line in $sumLines) {
    if ($line -notmatch '^(?<hash>[0-9a-fA-F]{64})\s{2}(?<path>.+)$') {
        Fail "malformed checksum line: $line"
    }
    $rel = $Matches.path -replace "/", "\"
    $full = Join-Path $Pkg $rel
    if (-not (Test-Path -LiteralPath $full)) { Fail "missing file for checksum: $rel" }
    $actual = (Get-FileHash -Algorithm SHA256 -LiteralPath $full).Hash
    if ($actual.ToLowerInvariant() -ne $Matches.hash.ToLowerInvariant()) {
        Fail "checksum mismatch for $rel"
    }
}
Write-Host "  $($sumLines.Count) files OK"

# --- Version ---
Write-Host "=== version ===" -ForegroundColor Cyan
$verOut = & $exe version 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { Fail "version failed" }
if ($verOut -notmatch "0\.18\.0") { Fail "expected version 0.18.0 in: $verOut" }
Write-Host "  $verOut".Trim()

$work = Join-Path $Pkg "_verify_work"
if (Test-Path -LiteralPath $work) { Remove-Item -LiteralPath $work -Recurse -Force }
New-Item -ItemType Directory -Force -Path $work | Out-Null

# --- Welcome ---
Write-Host "=== welcome compile + run ===" -ForegroundColor Cyan
$welcomeAe = Join-Path $Pkg "examples\welcome.ae"
$welcomeOut = Join-Path $work "welcome.aeth"
& $exe compile $welcomeAe --output $welcomeOut
if ($LASTEXITCODE -ne 0) { Fail "welcome compile failed" }
$welcomeLog = & $exe run $welcomeOut 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { Fail "welcome run process failed" }
if ($welcomeLog -notmatch "exited with") { Fail "welcome run missing exit line" }
Write-Host "  welcome OK"

# --- Host pilot ---
Write-Host "=== host-pilot (expect program exit 48) ===" -ForegroundColor Cyan
$hostAe = Join-Path $Pkg "examples\host-pilot.ae"
$hostOut = Join-Path $work "host-pilot.aeth"
& $exe compile $hostAe --output $hostOut
if ($LASTEXITCODE -ne 0) { Fail "host-pilot compile failed" }
$hostLog = & $exe run $hostOut 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { Fail "host-pilot run process failed" }
if ($hostLog -notmatch "exited with 48\b") { Fail "host-pilot expected exited with 48" }
Write-Host "  host-pilot OK"

# --- Project verify ---
Write-Host "=== project verify ===" -ForegroundColor Cyan
$proj = Join-Path $Pkg "examples\project\aether.project.json"
$projOut = Join-Path $work "project-out"
New-Item -ItemType Directory -Force -Path $projOut | Out-Null
& $exe project verify $proj --output-dir $projOut
if ($LASTEXITCODE -ne 0) { Fail "project verify failed" }
Write-Host "  project verify OK"

# --- Negative: path escape ---
Write-Host "=== negative path escape ===" -ForegroundColor Cyan
$negDir = Join-Path $work "escape-proj"
New-Item -ItemType Directory -Force -Path $negDir | Out-Null
$negJson = Join-Path $negDir "aether.project.json"
@'
{
  "schema": "aether.project/v1",
  "name": "escape_demo",
  "version": "0.0.1",
  "units": [
    { "path": "../welcome.ae", "role": "main" }
  ]
}
'@ | Set-Content -LiteralPath $negJson -Encoding utf8
$prev = $ErrorActionPreference
$ErrorActionPreference = "Continue"
& $exe project verify $negJson 2>&1 | Out-Null
$negCode = $LASTEXITCODE
$ErrorActionPreference = $prev
if ($negCode -eq 0) { Fail "path escape project should fail closed" }
Write-Host "  path escape rejected (exit $negCode) OK"

Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "PREVIEW VERIFY PASS" -ForegroundColor Green
exit 0
