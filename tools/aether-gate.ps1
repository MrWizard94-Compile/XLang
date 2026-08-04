<#
.SYNOPSIS
  Offline Aether quality gate (CONST-GATE-001).

.DESCRIPTION
  Runs pack verify (when found), fmt, clippy -D warnings, core/CLI tests,
  example dual-compare, host-pilot run, and project verify.
  -Mode full also rebuilds seed via bootstrap + forge and checks hash identity.

.PARAMETER Mode
  quick  — day-to-day (default)
  full   — release / TP-2 blocking (includes seed forge identity)

.NOTES
  Rule IDs: CONST-GATE-001, ENG-WARN-001, TEST-BEHAVIOR-001, GOV-INT-001
#>
[CmdletBinding()]
param(
    [ValidateSet("quick", "full")]
    [string]$Mode = "quick",

    [switch]$SkipPack
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $RepoRoot

function Write-Step([string]$Name) {
    Write-Host ""
    Write-Host "=== $Name ===" -ForegroundColor Cyan
}

function Fail([string]$Message) {
    Write-Host "FAIL: $Message" -ForegroundColor Red
    exit 1
}

function Invoke-Checked([string]$Label, [scriptblock]$Block) {
    Write-Step $Label
    & $Block
    if ($LASTEXITCODE -ne 0 -and $null -ne $LASTEXITCODE) {
        Fail "$Label exited $LASTEXITCODE"
    }
}

function Find-PackVerify {
    $candidates = @(
        (Join-Path $RepoRoot "..\..\AGENTS Constitution\tools\verify-pack.ps1"),
        (Join-Path $RepoRoot "AGENTS Constitution\tools\verify-pack.ps1"),
        (Join-Path $RepoRoot "..\AGENTS Constitution\tools\verify-pack.ps1")
    )
    foreach ($c in $candidates) {
        if (Test-Path -LiteralPath $c) {
            return (Resolve-Path -LiteralPath $c).Path
        }
    }
    return $null
}

Write-Host "Aether gate mode=$Mode root=$RepoRoot"

# --- Pack integrity ---
if (-not $SkipPack) {
    $pack = Find-PackVerify
    if ($null -eq $pack) {
        Write-Host "WARN: verify-pack.ps1 not found; skip pack verify (GOV-INT-001 deferred)" -ForegroundColor Yellow
    }
    else {
        Invoke-Checked "Pack verify ($pack)" {
            & pwsh -NoProfile -File $pack
        }
    }
}
else {
    Write-Host "Skipping pack verify (-SkipPack)" -ForegroundColor Yellow
}

# --- Format ---
Invoke-Checked "cargo fmt --check" {
    cargo fmt --all -- --check
}

# --- Clippy ---
Invoke-Checked "cargo clippy -D warnings" {
    cargo clippy -p aether-core -p aether-cli -- -D warnings
}

# --- Tests ---
# quick: lib + semantic integration tests (skip multi-generation seed_self_host rebuild)
# full: entire aether-core suite including seed_self_host
if ($Mode -eq "quick") {
    Invoke-Checked "cargo test aether-core --lib" {
        cargo test -p aether-core --lib
    }
    foreach ($t in @(
            "m4_error_effect_semantics",
            "m5_comptime_semantics",
            "m6_layout_semantics",
            "m7_nursery_semantics"
        )) {
        Invoke-Checked "cargo test aether-core --test $t" {
            cargo test -p aether-core --test $t
        }
    }
}
else {
    Invoke-Checked "cargo test aether-core (includes seed_self_host)" {
        cargo test -p aether-core
    }
}

Invoke-Checked "cargo test aether-cli" {
    cargo test -p aether-cli
}

# --- Dual-compare shipped examples (seed ≡ bootstrap) ---
Write-Step "Example dual-compare (seed ≡ bootstrap)"
$examplesDir = Join-Path $RepoRoot "examples"
$outDir = Join-Path $RepoRoot "target\gate-examples"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$aeFiles = Get-ChildItem -Path $examplesDir -Filter "*.ae" -File
if ($aeFiles.Count -eq 0) {
    Fail "No examples/*.ae found"
}

foreach ($ae in $aeFiles) {
    $stem = $ae.BaseName
    $seedOut = Join-Path $outDir "$stem.seed.aeth"
    $bootOut = Join-Path $outDir "$stem.bootstrap.aeth"

    Write-Host "  dual-compare $($ae.Name)"
    cargo run -q -p aether-cli -- compile $ae.FullName --output $seedOut
    if ($LASTEXITCODE -ne 0) { Fail "seed compile failed: $($ae.Name)" }

    cargo run -q -p aether-cli -- compile $ae.FullName --output $bootOut --bootstrap
    if ($LASTEXITCODE -ne 0) { Fail "bootstrap compile failed: $($ae.Name)" }

    $h1 = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedOut).Hash
    $h2 = (Get-FileHash -Algorithm SHA256 -LiteralPath $bootOut).Hash
    if ($h1 -ne $h2) {
        Fail "byte mismatch for $($ae.Name): seed=$h1 bootstrap=$h2"
    }
}
Write-Host "  all $($aeFiles.Count) top-level examples match seed≡bootstrap"

# --- Host pilot runtime ---
# CLI process exits 0 on successful VM run; program exit is printed as
# "Aether <ver> exited with <code>" (host-pilot yields 48).
Write-Step "host-pilot compile + run (expect program exit 48)"
$hostSeed = Join-Path $outDir "host-pilot.seed.aeth"
$hostAe = Join-Path $examplesDir "host-pilot.ae"
cargo run -q -p aether-cli -- compile $hostAe --output $hostSeed
if ($LASTEXITCODE -ne 0) { Fail "host-pilot compile failed" }

$hostLog = Join-Path $outDir "host-pilot.run.txt"
cargo run -q -p aether-cli -- run $hostSeed *>&1 | Tee-Object -FilePath $hostLog | Out-Host
if ($LASTEXITCODE -ne 0) { Fail "host-pilot CLI run failed (process exit $LASTEXITCODE)" }
$hostText = Get-Content -LiteralPath $hostLog -Raw
if ($hostText -notmatch "exited with 48\b") {
    Fail "host-pilot expected 'exited with 48' in output"
}
Write-Host "  host-pilot program exit 48 OK"

# --- Project verify (single-unit + multi-unit) ---
Write-Step "project verify examples/project"
$projectFile = Join-Path $examplesDir "project\aether.project.json"
$projectOut = Join-Path $outDir "project-verify"
New-Item -ItemType Directory -Force -Path $projectOut | Out-Null
cargo run -q -p aether-cli -- project verify $projectFile --output-dir $projectOut
if ($LASTEXITCODE -ne 0) { Fail "project verify failed" }
Write-Host "  project verify OK"

Write-Step "project verify examples/project-multi"
$multiFile = Join-Path $examplesDir "project-multi\aether.project.json"
$multiOut = Join-Path $outDir "project-multi-verify"
New-Item -ItemType Directory -Force -Path $multiOut | Out-Null
cargo run -q -p aether-cli -- project verify $multiFile --output-dir $multiOut
if ($LASTEXITCODE -ne 0) { Fail "project-multi verify failed" }
$mappedMain = Join-Path $multiOut "src__main.aeth"
$mappedLib = Join-Path $multiOut "lib__helper.aeth"
if (-not (Test-Path -LiteralPath $mappedMain)) { Fail "missing $mappedMain" }
if (-not (Test-Path -LiteralPath $mappedLib)) { Fail "missing $mappedLib" }
cargo run -q -p aether-cli -- project format $multiFile
if ($LASTEXITCODE -ne 0) { Fail "project-multi format failed" }
Write-Host "  project-multi verify + format OK"

# --- Full: seed forge identity ---
if ($Mode -eq "full") {
    Write-Step "Seed bootstrap + forge hash identity"
    $seedAe = Join-Path $RepoRoot "seed\aether_seed.ae"
    $seedBoot = Join-Path $RepoRoot "target\aether_seed.gate.bootstrap.aeth"
    $seedForged = Join-Path $RepoRoot "target\aether_seed.gate.forged.aeth"
    $seedCheckedIn = Join-Path $RepoRoot "seed\aether_seed.aeth"

    cargo run -q -p aether-cli -- compile $seedAe --output $seedBoot --bootstrap
    if ($LASTEXITCODE -ne 0) { Fail "seed bootstrap compile failed" }

    cargo run -q -p aether-cli -- forge $seedBoot $seedAe --output $seedForged
    if ($LASTEXITCODE -ne 0) { Fail "seed forge failed" }

    $hb = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedBoot).Hash
    $hf = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedForged).Hash
    $hc = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedCheckedIn).Hash

    if ($hb -ne $hf) {
        Fail "bootstrap≠forged: bootstrap=$hb forged=$hf"
    }
    if ($hb -ne $hc) {
        Fail "bootstrap≠checked-in seed/aether_seed.aeth: bootstrap=$hb checked-in=$hc"
    }
    Write-Host "  seed SHA-256: $hb"
    Write-Host "  bootstrap ≡ forged ≡ checked-in OK"
}

Write-Host ""
Write-Host "GATE PASS mode=$Mode" -ForegroundColor Green
exit 0
