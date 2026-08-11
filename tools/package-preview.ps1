<#
.SYNOPSIS
  Stage the current Aether local technical-preview package under dist/.

.DESCRIPTION
  Builds the release CLI unless -SkipBuild is supplied, then creates an exact,
  version-derived package containing the executable contract, current release
  documents, checked-in seed, schemas, examples, stdlib, and consumer verifier.
  SHA-256SUMS covers every staged file except itself.

.PARAMETER SkipBuild
  Reuse an existing target/release/aether.exe after validating it exists.

.NOTES
  Rule IDs: REL-PACKAGE-001, REL-DETERM-001, OPS-DEL-001, SEC-INPUT-001.
#>
[CmdletBinding()]
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Fail([string]$Message) {
    Write-Host "FAIL: $Message" -ForegroundColor Red
    exit 1
}

function Get-CliPackageVersion([string]$RepoRoot) {
    $manifestPath = Join-Path $RepoRoot "apps\xlang-cli\Cargo.toml"
    if (-not (Test-Path -LiteralPath $manifestPath)) {
        Fail "missing CLI manifest: $manifestPath"
    }

    $manifest = Get-Content -Raw -LiteralPath $manifestPath
    $match = [regex]::Match($manifest, '(?m)^\s*version\s*=\s*"(?<version>[0-9A-Za-z.+-]+)"\s*$')
    if (-not $match.Success) {
        Fail "could not read an explicit package version from $manifestPath"
    }

    $version = $match.Groups["version"].Value
    if ($version -notmatch '^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$') {
        Fail "CLI package version is not a safe SemVer package segment: $version"
    }

    return $version
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

function Assert-RelativePath([string]$Path, [string]$Label) {
    if ([System.IO.Path]::IsPathRooted($Path) -or
        $Path -match '(^|[\\/])\.\.?(?:[\\/]|$)') {
        Fail "$Label must be a confined relative path: $Path"
    }
}

function Write-Utf8NoBom([string]$Path, [string]$Contents) {
    $encoding = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($Path, $Contents, $encoding)
}

$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Set-Location $RepoRoot

$Version = Get-CliPackageVersion $RepoRoot
$PackageName = "aether-$Version-tp"
$DistRoot = Join-Path $RepoRoot "dist"
New-Item -ItemType Directory -Force -Path $DistRoot | Out-Null
$DistRoot = (Resolve-Path -LiteralPath $DistRoot).Path
$PackageRoot = Assert-ChildPath $DistRoot (Join-Path $DistRoot $PackageName) "technical-preview package root"

function Copy-FileToPackage([string]$SourceRelativePath, [string]$DestinationRelativePath = $SourceRelativePath) {
    Assert-RelativePath $SourceRelativePath "source path"
    Assert-RelativePath $DestinationRelativePath "package destination"

    $source = Join-Path $RepoRoot $SourceRelativePath
    if (-not (Test-Path -LiteralPath $source -PathType Leaf)) {
        Fail "required release file is missing: $SourceRelativePath"
    }

    $destination = Assert-ChildPath $PackageRoot (Join-Path $PackageRoot $DestinationRelativePath) "package destination"
    $destinationParent = Split-Path -Parent $destination
    New-Item -ItemType Directory -Force -Path $destinationParent | Out-Null
    Copy-Item -LiteralPath $source -Destination $destination -Force
}

function Copy-DirectoryToPackage([string]$SourceRelativePath) {
    Assert-RelativePath $SourceRelativePath "source directory"

    $source = Join-Path $RepoRoot $SourceRelativePath
    if (-not (Test-Path -LiteralPath $source -PathType Container)) {
        Fail "required release directory is missing: $SourceRelativePath"
    }

    $destination = Assert-ChildPath $PackageRoot (Join-Path $PackageRoot $SourceRelativePath) "package directory"
    New-Item -ItemType Directory -Force -Path $destination | Out-Null
    foreach ($entry in @(Get-ChildItem -LiteralPath $source -Force)) {
        Copy-Item -LiteralPath $entry.FullName -Destination $destination -Recurse -Force
    }
}

if (-not $SkipBuild) {
    Write-Host "=== cargo build --release -p aether-cli ===" -ForegroundColor Cyan
    cargo build --release -p aether-cli
    if ($LASTEXITCODE -ne 0) { Fail "release build failed" }
}

$ExecutableSource = Join-Path $RepoRoot "target\release\aether.exe"
if (-not (Test-Path -LiteralPath $ExecutableSource -PathType Leaf)) {
    Fail "missing $ExecutableSource — run without -SkipBuild or build first"
}

if (Test-Path -LiteralPath $PackageRoot) {
    Write-Host "=== replacing $PackageRoot ===" -ForegroundColor Cyan
    Remove-Item -LiteralPath $PackageRoot -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $PackageRoot | Out-Null

Write-Host "=== staging $PackageRoot ===" -ForegroundColor Cyan
Copy-Item -LiteralPath $ExecutableSource -Destination (Join-Path $PackageRoot "aether.exe") -Force

# The package root README is purpose-built for a portable local preview. The
# repository README contains checkout-specific engineering directions instead.
Copy-FileToPackage "RELEASE-README-0.37-LOCAL-PACKAGES.md" "README.md"
Copy-FileToPackage "MANIFEST.md"
Copy-DirectoryToPackage "docs"
Copy-DirectoryToPackage "seed"
Copy-DirectoryToPackage "schemas"
Copy-DirectoryToPackage "examples"
Copy-DirectoryToPackage "stdlib"
Copy-FileToPackage "tools\verify-preview.ps1" "verify-preview.ps1"

$metadata = [ordered]@{
    schema = "aether.preview/v1"
    product = "Aether"
    packageVersion = $Version
    languageSurface = "0.11"
    aeth = [ordered]@{
        withoutTaskFrames = 11
        taskFrames = 12
    }
    authoring = [ordered]@{
        ast = "aether.ast/v8"
        edit = "aether.edit/v8"
        diagnostic = "aether.diagnostic/v8"
    }
    channel = "local-folder"
    license = "UNLICENSED"
}
$metadataJson = ($metadata | ConvertTo-Json -Depth 4) -replace "`r`n", "`n"
Write-Utf8NoBom (Join-Path $PackageRoot "RELEASE-METADATA.json") "$metadataJson`n"

Write-Host "=== SHA-256SUMS ===" -ForegroundColor Cyan
$sumsPath = Join-Path $PackageRoot "SHA-256SUMS"
$sumLines = New-Object System.Collections.Generic.List[string]
Get-ChildItem -LiteralPath $PackageRoot -Recurse -File |
    Where-Object { $_.Name -ne "SHA-256SUMS" } |
    Sort-Object FullName |
    ForEach-Object {
        $relative = $_.FullName.Substring($PackageRoot.Length)
        $relative = $relative.TrimStart([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar)
        $relative = $relative.Replace([System.IO.Path]::DirectorySeparatorChar, [char]"/")
        $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $_.FullName).Hash.ToLowerInvariant()
        $sumLines.Add("$hash  $relative")
    }
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllLines($sumsPath, $sumLines, $utf8NoBom)

Write-Host "Package ready: $PackageRoot"
Write-Host "Files: $($sumLines.Count) plus SHA-256SUMS"
Get-Content -LiteralPath $sumsPath | Select-Object -First 5
Write-Host "..."
exit 0
