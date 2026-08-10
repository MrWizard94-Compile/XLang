<#
.SYNOPSIS
  Offline Aether quality gate (CONST-GATE-001).

.DESCRIPTION
  Runs pack verify (when found), fmt, workspace Clippy with warnings denied,
  tests, example dual-compare, host-pilot run, and project verification.
  -Mode full also rebuilds seed via bootstrap + forge and checks hash identity.
  -Mode release adds a release build, a version-derived local package, consumer
  verification, and a negative package-integrity check.

.PARAMETER Mode
  quick   — day-to-day (default)
  full    — full source and seed proof (includes seed forge identity)
  release — full plus local technical-preview packaging and consumer proof

.NOTES
  Rule IDs: CONST-GATE-001, ENG-WARN-001, TEST-BEHAVIOR-001, GOV-INT-001,
  REL-PACKAGE-001, REL-DETERM-001, SEC-INPUT-001
#>
[CmdletBinding()]
param(
    [ValidateSet("quick", "full", "release")]
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

function Get-CliPackageVersion {
    $manifestPath = Join-Path $RepoRoot "apps\xlang-cli\Cargo.toml"
    if (-not (Test-Path -LiteralPath $manifestPath)) {
        Fail "missing CLI manifest: $manifestPath"
    }
    $manifest = Get-Content -Raw -LiteralPath $manifestPath
    $match = [regex]::Match($manifest, '(?m)^\s*version\s*=\s*"(?<version>[0-9A-Za-z.+-]+)"\s*$')
    if (-not $match.Success) {
        Fail "could not read the CLI package version from $manifestPath"
    }
    return $match.Groups["version"].Value
}

function Assert-ChildPath([string]$Parent, [string]$Candidate, [string]$Label) {
    $parentFull = [System.IO.Path]::GetFullPath($Parent)
    $candidateFull = [System.IO.Path]::GetFullPath($Candidate)
    $separator = [System.IO.Path]::DirectorySeparatorChar
    $prefix = if ($parentFull.EndsWith([string]$separator)) { $parentFull } else { "$parentFull$separator" }
    if (-not $candidateFull.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
        Fail "$Label escapes its required parent: $candidateFull"
    }
    return $candidateFull
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
Invoke-Checked "cargo clippy --workspace --all-targets -D warnings" {
    cargo clippy --workspace --all-targets -- -D warnings
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
    Invoke-Checked "cargo test aether-cli" {
        cargo test -p aether-cli
    }
}
else {
    Invoke-Checked "cargo test --workspace (includes seed_self_host)" {
        cargo test --workspace
    }
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

Write-Step "project build examples/project-modules"
$modFile = Join-Path $examplesDir "project-modules\aether.project.json"
$modOut = Join-Path $outDir "project-modules.aeth"
if (Test-Path -LiteralPath $modFile) {
    cargo run -q -p aether-cli -- project build $modFile --output $modOut
    if ($LASTEXITCODE -ne 0) { Fail "project-modules build failed" }
    cargo run -q -p aether-cli -- run $modOut *>&1 | Tee-Object -FilePath (Join-Path $outDir "modules.run.txt") | Out-Host
    $modText = Get-Content -LiteralPath (Join-Path $outDir "modules.run.txt") -Raw
    if ($modText -notmatch "exited with 42\b") { Fail "project-modules expected exited with 42" }
    Write-Host "  project-modules build+run 42 OK"
} else {
    Write-Host "  skip (no project-modules example)" -ForegroundColor Yellow
}

# --- Full: seed forge identity ---
if ($Mode -ne "quick") {
    Write-Step "Seed bootstrap + product + forge hash identity (ADR-049)"
    $seedAe = Join-Path $RepoRoot "seed\aether_seed.ae"
    $seedBoot = Join-Path $RepoRoot "target\aether_seed.gate.bootstrap.aeth"
    $seedProduct = Join-Path $RepoRoot "target\aether_seed.gate.product.aeth"
    $seedForged = Join-Path $RepoRoot "target\aether_seed.gate.forged.aeth"
    $seedCheckedIn = Join-Path $RepoRoot "seed\aether_seed.aeth"

    cargo run -q -p aether-cli -- compile $seedAe --output $seedBoot --bootstrap
    if ($LASTEXITCODE -ne 0) { Fail "seed bootstrap compile failed" }

    # Product path self-rebuild (no --bootstrap): Aether independence proof.
    cargo run -q -p aether-cli -- compile $seedAe --output $seedProduct
    if ($LASTEXITCODE -ne 0) { Fail "seed product compile failed" }

    cargo run -q -p aether-cli -- forge $seedBoot $seedAe --output $seedForged
    if ($LASTEXITCODE -ne 0) { Fail "seed forge failed" }

    $hb = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedBoot).Hash
    $hp = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedProduct).Hash
    $hf = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedForged).Hash
    $hc = (Get-FileHash -Algorithm SHA256 -LiteralPath $seedCheckedIn).Hash

    if ($hb -ne $hf) {
        Fail "bootstrap≠forged: bootstrap=$hb forged=$hf"
    }
    if ($hb -ne $hp) {
        Fail "bootstrap≠product seed rebuild: bootstrap=$hb product=$hp"
    }
    if ($hb -ne $hc) {
        Fail "bootstrap≠checked-in seed/aether_seed.aeth: bootstrap=$hb checked-in=$hc"
    }
    Write-Host "  seed SHA-256: $hb"
    Write-Host "  bootstrap ≡ product ≡ forged ≡ checked-in OK"
}

# --- Release: package and consumer verification ---
if ($Mode -eq "release") {
    Write-Step "Release build + local technical-preview package"
    cargo build --release -p aether-cli
    if ($LASTEXITCODE -ne 0) { Fail "release CLI build failed" }

    $packageScript = Join-Path $RepoRoot "tools\package-preview.ps1"
    & pwsh -NoProfile -File $packageScript -SkipBuild
    if ($LASTEXITCODE -ne 0) { Fail "technical-preview packaging failed" }

    $version = Get-CliPackageVersion
    $packageRoot = Join-Path $RepoRoot "dist\aether-$version-tp"
    $previewVerifier = Join-Path $packageRoot "verify-preview.ps1"
    if (-not (Test-Path -LiteralPath $previewVerifier -PathType Leaf)) {
        Fail "staged preview verifier is missing: $previewVerifier"
    }

    Write-Step "Consumer verification of local technical-preview package"
    & pwsh -NoProfile -File $previewVerifier -PackageRoot $packageRoot
    if ($LASTEXITCODE -ne 0) { Fail "consumer preview verification failed" }

    Write-Step "Preview verifier rejects an unlisted package file"
    $tamperParent = Join-Path $RepoRoot "target"
    New-Item -ItemType Directory -Force -Path $tamperParent | Out-Null
    $tamperRoot = Assert-ChildPath $tamperParent (Join-Path $tamperParent "gate-preview-unlisted") "preview verifier tamper fixture"
    if (Test-Path -LiteralPath $tamperRoot) {
        Remove-Item -LiteralPath $tamperRoot -Recurse -Force
    }

    $tamperFailure = $null
    try {
        Copy-Item -LiteralPath $packageRoot -Destination $tamperRoot -Recurse -Force
        $unlistedFile = Assert-ChildPath $tamperRoot (Join-Path $tamperRoot "UNLISTED-TAMPER-PROBE.txt") "preview verifier tamper probe"
        $encoding = New-Object System.Text.UTF8Encoding $false
        [System.IO.File]::WriteAllText($unlistedFile, "intentional verification probe`n", $encoding)

        & pwsh -NoProfile -File $previewVerifier -PackageRoot $tamperRoot
        $tamperExit = $LASTEXITCODE
        if ($tamperExit -eq 0) {
            $tamperFailure = "preview verifier accepted an unlisted package file"
        }
        else {
            Write-Host "  unlisted package file rejected (exit $tamperExit) OK"
        }
    }
    finally {
        if (Test-Path -LiteralPath $tamperRoot) {
            Remove-Item -LiteralPath $tamperRoot -Recurse -Force
        }
    }
    if ($null -ne $tamperFailure) { Fail $tamperFailure }
}

Write-Host ""
Write-Host "GATE PASS mode=$Mode" -ForegroundColor Green
exit 0
