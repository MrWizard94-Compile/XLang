<#
.SYNOPSIS
  Records repeatable release-profile timings for Aether's self-host rebuild.

.DESCRIPTION
  Builds the release integration-test binary once, then directly executes the
  multi-generation self-host proof several times. Build time is intentionally
  excluded from the samples. The JSON report records the exact test, toolchain,
  operating-system identifiers, and min/median/max distribution; it is local
  engineering evidence, not a cross-machine performance claim.

.NOTES
  Rule IDs: PERF-EVIDENCE-001, TEST-BEHAVIOR-001, ENG-WARN-001
#>
[CmdletBinding()]
param(
    [ValidateRange(3, 15)]
    [int]$Runs = 3,

    [ValidateRange(0, 3)]
    [int]$WarmupRuns = 0,

    [ValidateRange(1, 900)]
    [int]$TimeoutSeconds = 300,

    [string]$Output = "target/seed-self-host-performance.json"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot
$TestName = "seed_profile_compiler_rebuilds_itself_and_a_distinct_valid_variant"

if ([System.IO.Path]::IsPathRooted($Output)) {
    $ReportPath = $Output
}
else {
    $ReportPath = Join-Path $RepoRoot $Output
}
$ReportDirectory = Split-Path -Parent $ReportPath
New-Item -ItemType Directory -Force -Path $ReportDirectory | Out-Null

$LogDirectory = Join-Path $RepoRoot "target/measure-seed-self-host"
New-Item -ItemType Directory -Force -Path $LogDirectory | Out-Null

Write-Host "Building release self-host test binary..."
& cargo test --release -p aether-core --test seed_self_host --no-run
if ($LASTEXITCODE -ne 0) {
    throw "release self-host test build failed with exit code $LASTEXITCODE"
}

$TestBinary = Get-ChildItem -LiteralPath (Join-Path $RepoRoot "target/release/deps") -Filter "seed_self_host-*.exe" -File |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1 -ExpandProperty FullName
if ([string]::IsNullOrWhiteSpace($TestBinary)) {
    throw "release self-host test binary was not produced"
}

function Invoke-SelfHostSample([string]$Label) {
    $StdoutPath = Join-Path $LogDirectory "$Label.stdout.txt"
    $StderrPath = Join-Path $LogDirectory "$Label.stderr.txt"
    $Timer = [System.Diagnostics.Stopwatch]::StartNew()
    $Process = Start-Process -FilePath $TestBinary -ArgumentList @($TestName, "--exact") -PassThru -WindowStyle Hidden -RedirectStandardOutput $StdoutPath -RedirectStandardError $StderrPath
    if (-not $Process.WaitForExit($TimeoutSeconds * 1000)) {
        Stop-Process -Id $Process.Id -Force
        throw "self-host $Label exceeded the $TimeoutSeconds-second timeout; stopped process $($Process.Id)"
    }
    $Process.WaitForExit()
    $Timer.Stop()
    if ($Process.ExitCode -ne 0) {
        $Stdout = Get-Content -LiteralPath $StdoutPath -Raw
        $Stderr = Get-Content -LiteralPath $StderrPath -Raw
        throw "self-host $Label failed with exit code $($Process.ExitCode)`n$Stdout`n$Stderr"
    }
    return [int64]$Timer.ElapsedMilliseconds
}

for ($Index = 1; $Index -le $WarmupRuns; $Index++) {
    $Elapsed = Invoke-SelfHostSample "warmup-$Index"
    Write-Host "warmup=$Index elapsed-ms=$Elapsed"
}

$Samples = [System.Collections.Generic.List[int64]]::new()
for ($Index = 1; $Index -le $Runs; $Index++) {
    $Elapsed = Invoke-SelfHostSample "sample-$Index"
    $Samples.Add($Elapsed)
    Write-Host "sample=$Index elapsed-ms=$Elapsed"
}

$Ordered = @($Samples | Sort-Object)
$Median = if ($Runs % 2 -eq 1) {
    $Ordered[[int][Math]::Floor($Runs / 2)]
}
else {
    [int64](($Ordered[($Runs / 2) - 1] + $Ordered[$Runs / 2]) / 2)
}

$Report = [ordered]@{
    schema = "aether.seed-self-host.performance/v1"
    generated_at_utc = (Get-Date).ToUniversalTime().ToString("O")
    profile = "release"
    test = $TestName
    runs = $Runs
    warmup_runs = $WarmupRuns
    timeout_seconds = $TimeoutSeconds
    samples_ms = @($Samples)
    min_ms = $Ordered[0]
    median_ms = $Median
    max_ms = $Ordered[$Ordered.Count - 1]
    test_binary = $TestBinary
    test_binary_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $TestBinary).Hash
    cargo = (& cargo --version)
    rustc = (& rustc --version)
    os = [Environment]::OSVersion.VersionString
    processor_identifier = [Environment]::GetEnvironmentVariable("PROCESSOR_IDENTIFIER")
    logical_processors = [Environment]::ProcessorCount
    notes = @(
        "Build time is excluded after cargo test --no-run.",
        "Each sample directly invokes the same exact integration test binary.",
        "This local distribution is not a cross-machine or cross-language performance claim."
    )
}

$Report | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath $ReportPath -Encoding utf8NoBOM
Write-Host "report=$ReportPath"
Write-Host "min-ms=$($Report.min_ms) median-ms=$($Report.median_ms) max-ms=$($Report.max_ms)"
