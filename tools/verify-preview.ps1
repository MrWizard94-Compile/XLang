<#
.SYNOPSIS
  Verify a staged Aether local technical-preview package as a consumer.

.DESCRIPTION
  Validates exact package membership and SHA-256SUMS, the release metadata and
  CLI version, then exercises the shipped v11/v12, host, project, workspace,
  authoring, and path-confinement surfaces without a source checkout.

.PARAMETER PackageRoot
  Package directory containing aether.exe and RELEASE-METADATA.json. When
  omitted, the script accepts its current directory, its own staged directory,
  or the current version-derived package under a source checkout's dist/.

.NOTES
  Rule IDs: REL-PACKAGE-001, REL-DETERM-001, SEC-INPUT-001,
  TEST-BEHAVIOR-001, CONST-GATE-001.
#>
[CmdletBinding()]
param(
    [string]$PackageRoot = ""
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
        Fail "missing CLI manifest while locating a source-checkout package: $manifestPath"
    }

    $manifest = Get-Content -Raw -LiteralPath $manifestPath
    $match = [regex]::Match($manifest, '(?m)^\s*version\s*=\s*"(?<version>[0-9A-Za-z.+-]+)"\s*$')
    if (-not $match.Success) {
        Fail "could not read an explicit package version from $manifestPath"
    }

    return $match.Groups["version"].Value
}

function Get-ConfinedPackagePath([string]$Root, [string]$RelativePath, [string]$Label) {
    if ([string]::IsNullOrWhiteSpace($RelativePath) -or
        [System.IO.Path]::IsPathRooted($RelativePath) -or
        $RelativePath.Contains("\") -or
        $RelativePath -match '(^|/)\.\.?(?:/|$)' -or
        $RelativePath.Contains(":")) {
        Fail "$Label is not a normalized confined package path: $RelativePath"
    }

    $rootFull = [System.IO.Path]::GetFullPath($Root)
    $candidate = [System.IO.Path]::GetFullPath((Join-Path $rootFull $RelativePath))
    $separator = [System.IO.Path]::DirectorySeparatorChar
    $prefix = if ($rootFull.EndsWith([string]$separator)) { $rootFull } else { "$rootFull$separator" }
    if (-not $candidate.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)) {
        Fail "$Label escapes the package root: $RelativePath"
    }

    return $candidate
}

function Resolve-PackageRoot {
    if (-not [string]::IsNullOrWhiteSpace($PackageRoot)) {
        $candidate = (Resolve-Path -LiteralPath $PackageRoot).Path
        if (-not (Test-Path -LiteralPath (Join-Path $candidate "aether.exe") -PathType Leaf)) {
            Fail "-PackageRoot does not contain aether.exe: $candidate"
        }
        return $candidate
    }

    $currentDirectory = (Get-Location).Path
    if (Test-Path -LiteralPath (Join-Path $currentDirectory "aether.exe") -PathType Leaf) {
        return (Resolve-Path -LiteralPath $currentDirectory).Path
    }

    $scriptDirectory = if ([string]::IsNullOrWhiteSpace($PSScriptRoot)) { $currentDirectory } else { $PSScriptRoot }
    if (Test-Path -LiteralPath (Join-Path $scriptDirectory "aether.exe") -PathType Leaf) {
        return (Resolve-Path -LiteralPath $scriptDirectory).Path
    }

    $sourceRootCandidate = Join-Path $scriptDirectory ".."
    $sourceManifest = Join-Path $sourceRootCandidate "apps\xlang-cli\Cargo.toml"
    if (Test-Path -LiteralPath $sourceManifest -PathType Leaf) {
        $sourceRoot = (Resolve-Path -LiteralPath $sourceRootCandidate).Path
        $version = Get-CliPackageVersion $sourceRoot
        $packageCandidate = Join-Path $sourceRoot "dist\aether-$version-tp"
        if (Test-Path -LiteralPath (Join-Path $packageCandidate "aether.exe") -PathType Leaf) {
            return (Resolve-Path -LiteralPath $packageCandidate).Path
        }
        Fail "current package not found: $packageCandidate. Run tools/package-preview.ps1 first."
    }

    Fail "could not locate a package root. Pass -PackageRoot."
}

function Assert-ExactChecksums([string]$Root) {
    Write-Host "=== SHA-256SUMS exact integrity ===" -ForegroundColor Cyan
    $sumsPath = Join-Path $Root "SHA-256SUMS"
    if (-not (Test-Path -LiteralPath $sumsPath -PathType Leaf)) {
        Fail "missing SHA-256SUMS"
    }

    $lines = @(Get-Content -LiteralPath $sumsPath | Where-Object { $_.Trim() -ne "" })
    if ($lines.Count -eq 0) {
        Fail "SHA-256SUMS contains no entries"
    }

    $entries = @{}
    foreach ($line in $lines) {
        if ($line -notmatch '^(?<hash>[0-9a-f]{64})\s{2}(?<path>[^\s].*)$') {
            Fail "malformed checksum line: $line"
        }

        $relativePath = $Matches.path
        if ($entries.ContainsKey($relativePath)) {
            Fail "duplicate checksum path: $relativePath"
        }

        $fullPath = Get-ConfinedPackagePath $Root $relativePath "checksum path"
        if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
            Fail "missing file for checksum: $relativePath"
        }

        $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $fullPath).Hash.ToLowerInvariant()
        if ($actualHash -cne $Matches.hash) {
            Fail "checksum mismatch for $relativePath"
        }
        $entries[$relativePath] = $true
    }

    $actualPaths = @(
        Get-ChildItem -LiteralPath $Root -Recurse -File |
            Where-Object { $_.Name -ne "SHA-256SUMS" } |
            ForEach-Object {
                $relative = $_.FullName.Substring($Root.Length)
                $relative = $relative.TrimStart([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar)
                $relative.Replace([System.IO.Path]::DirectorySeparatorChar, [char]"/")
            }
    )

    foreach ($actualPath in $actualPaths) {
        if (-not $entries.ContainsKey($actualPath)) {
            Fail "unlisted package file: $actualPath"
        }
    }
    foreach ($relativePath in $entries.Keys) {
        if ($actualPaths -notcontains $relativePath) {
            Fail "checksum entry has no package file: $relativePath"
        }
    }

    Write-Host "  $($entries.Count) files verified with no unlisted files"
}

function Get-ReleaseMetadata([string]$Root) {
    Write-Host "=== release metadata ===" -ForegroundColor Cyan
    $metadataPath = Join-Path $Root "RELEASE-METADATA.json"
    if (-not (Test-Path -LiteralPath $metadataPath -PathType Leaf)) {
        Fail "missing RELEASE-METADATA.json"
    }

    try {
        $metadata = Get-Content -Raw -LiteralPath $metadataPath | ConvertFrom-Json
    }
    catch {
        Fail "invalid RELEASE-METADATA.json: $($_.Exception.Message)"
    }

    if ($metadata.schema -ne "aether.preview/v1" -or
        $metadata.product -ne "Aether" -or
        $metadata.channel -ne "local-folder" -or
        $metadata.license -ne "UNLICENSED") {
        Fail "release metadata does not describe the supported Aether local preview channel"
    }
    if ([string]$metadata.packageVersion -notmatch '^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$') {
        Fail "release metadata has an invalid packageVersion"
    }
    if ($metadata.languageSurface -ne "0.11" -or
        $metadata.aeth.withoutTaskFrames -ne 11 -or
        $metadata.aeth.taskFrames -ne 12 -or
        $metadata.authoring.ast -ne "aether.ast/v8" -or
        $metadata.authoring.edit -ne "aether.edit/v8" -or
        $metadata.authoring.diagnostic -ne "aether.diagnostic/v8") {
        Fail "release metadata does not match the supported 0.36 language/artifact/authoring contract"
    }

    Write-Host "  Aether $($metadata.packageVersion), AETH v11/v12, authoring v8"
    return $metadata
}

function Assert-RequiredPackageFiles([string]$Root, [string]$Version) {
    Write-Host "=== required package contents ===" -ForegroundColor Cyan
    $requiredFiles = @(
        "aether.exe",
        "README.md",
        "MANIFEST.md",
        "RELEASE-METADATA.json",
        "verify-preview.ps1",
        "seed/aether_seed.aeth",
        "schemas/aether-ast-v8.schema.json",
        "schemas/aether-edit-v8.schema.json",
        "schemas/aether-diagnostic-v8.schema.json",
        "examples/welcome.ae",
        "examples/host-pilot.ae",
        "examples/active-cancel.ae",
        "examples/task-frame-capacity.ae",
        "examples/task-loop.ae",
        "examples/project/aether.project.json",
        "examples/workspace/aether.workspace.json",
        "docs/AETHER_0.36.md",
        "docs/AETHER_AUTHORING_PROTOCOL_v8.md",
        "docs/CHANGELOG-0.36.md",
        "docs/RELEASE_NOTES-0.36-TECHNICAL-PREVIEW.md",
        "docs/THREAT_MODEL-0.36-TECHNICAL-PREVIEW.md"
    )
    foreach ($relativePath in $requiredFiles) {
        $fullPath = Get-ConfinedPackagePath $Root $relativePath "required package path"
        if (-not (Test-Path -LiteralPath $fullPath -PathType Leaf)) {
            Fail "required package file is missing: $relativePath"
        }
    }

    $stdlibPath = Join-Path $Root "stdlib"
    if (-not (Test-Path -LiteralPath $stdlibPath -PathType Container)) {
        Fail "required stdlib directory is missing"
    }
    Write-Host "  current contract, docs, seed, schemas, corpus, and stdlib present"
}

function Invoke-Example([string]$Executable, [string]$Root, [string]$WorkRoot, [string]$ExampleRelativePath, [int]$ExpectedProgramExit, [string]$Version) {
    $sourcePath = Get-ConfinedPackagePath $Root $ExampleRelativePath "example path"
    $stem = [System.IO.Path]::GetFileNameWithoutExtension($sourcePath)
    $artifactPath = Join-Path $WorkRoot "$stem.aeth"

    & $Executable compile $sourcePath --output $artifactPath | Out-Host
    if ($LASTEXITCODE -ne 0) {
        Fail "compile failed for $ExampleRelativePath"
    }

    $runOutput = & $Executable run $artifactPath 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        Fail "run process failed for $ExampleRelativePath"
    }
    $expectedLine = "Aether $Version exited with $ExpectedProgramExit"
    if (-not $runOutput.Contains($expectedLine)) {
        Fail "$ExampleRelativePath expected '$expectedLine'"
    }

    return [string]$runOutput
}

$Package = Resolve-PackageRoot
Set-Location $Package
Write-Host "Package root: $Package"

Assert-ExactChecksums $Package
$release = Get-ReleaseMetadata $Package
Assert-RequiredPackageFiles $Package ([string]$release.packageVersion)

$Executable = Join-Path $Package "aether.exe"
Write-Host "=== version ===" -ForegroundColor Cyan
$versionOutput = & $Executable version 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) {
    Fail "version command failed"
}
$expectedVersionOutput = "Aether $($release.packageVersion)"
if ($versionOutput.Trim() -cne $expectedVersionOutput) {
    Fail "expected '$expectedVersionOutput', received '$($versionOutput.Trim())'"
}
Write-Host "  $expectedVersionOutput"

$temporaryRoot = [System.IO.Path]::GetFullPath([System.IO.Path]::GetTempPath())
$workRoot = [System.IO.Path]::GetFullPath((Join-Path $temporaryRoot ("aether-preview-verify-" + [Guid]::NewGuid().ToString("N"))))
$temporaryPrefix = if ($temporaryRoot.EndsWith([string][System.IO.Path]::DirectorySeparatorChar)) { $temporaryRoot } else { "$temporaryRoot$([System.IO.Path]::DirectorySeparatorChar)" }
if (-not $workRoot.StartsWith($temporaryPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
    Fail "generated verification workspace escapes the temporary directory"
}

try {
    New-Item -ItemType Directory -Force -Path $workRoot | Out-Null

    Write-Host "=== v11 welcome compile + run ===" -ForegroundColor Cyan
    $welcomeOutput = Invoke-Example $Executable $Package $workRoot "examples/welcome.ae" 73 ([string]$release.packageVersion)
    if (-not $welcomeOutput.Contains("Aether")) {
        Fail "welcome example did not print its expected Text payload"
    }
    Write-Host "  welcome program exit 73 OK"

    Write-Host "=== pure host pilot ===" -ForegroundColor Cyan
    [void](Invoke-Example $Executable $Package $workRoot "examples/host-pilot.ae" 48 ([string]$release.packageVersion))
    Write-Host "  host-pilot program exit 48 OK"

    Write-Host "=== v12 active-frame cancellation ===" -ForegroundColor Cyan
    [void](Invoke-Example $Executable $Package $workRoot "examples/active-cancel.ae" 9 ([string]$release.packageVersion))
    [void](Invoke-Example $Executable $Package $workRoot "examples/task-frame-capacity.ae" 3 ([string]$release.packageVersion))
    [void](Invoke-Example $Executable $Package $workRoot "examples/task-loop.ae" 3 ([string]$release.packageVersion))
    Write-Host "  active cancellation, frame capacity, and checkpoint loop outputs OK"

    Write-Host "=== v8 structural authoring ===" -ForegroundColor Cyan
    $structureOutput = & $Executable structure (Get-ConfinedPackagePath $Package "examples/active-cancel.ae" "authoring example path") 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        Fail "structure command failed for active-cancel.ae"
    }
    try {
        $structure = $structureOutput | ConvertFrom-Json
    }
    catch {
        Fail "structure output was not valid JSON: $($_.Exception.Message)"
    }
    $taskWeaves = @($structure.program.weaves | Where-Object { $_.name -eq "staged" })
    if ($structure.schema -ne "aether.ast/v8" -or
        $taskWeaves.Count -ne 1 -or
        $taskWeaves[0].task -ne $true -or
        $structure.canonicalSource -notmatch '(?m)^\s+checkpoint$') {
        Fail "v8 structural authoring did not expose the shipped task/checkpoint contract"
    }
    Write-Host "  aether.ast/v8 task/checkpoint contract OK"

    Write-Host "=== project and workspace integrity ===" -ForegroundColor Cyan
    $projectOutput = Join-Path $workRoot "project-out"
    New-Item -ItemType Directory -Force -Path $projectOutput | Out-Null
    & $Executable project verify (Get-ConfinedPackagePath $Package "examples/project/aether.project.json" "project path") --output-dir $projectOutput
    if ($LASTEXITCODE -ne 0) {
        Fail "project verify failed"
    }
    & $Executable workspace verify (Get-ConfinedPackagePath $Package "examples/workspace/aether.workspace.json" "workspace path")
    if ($LASTEXITCODE -ne 0) {
        Fail "workspace verify failed"
    }
    Write-Host "  project and workspace verification OK"

    Write-Host "=== negative project path escape ===" -ForegroundColor Cyan
    $negativeDirectory = Join-Path $workRoot "escape-project"
    New-Item -ItemType Directory -Force -Path $negativeDirectory | Out-Null
    $negativeProject = Join-Path $negativeDirectory "aether.project.json"
    $negativeJson = @'
{
  "schema": "aether.project/v1",
  "name": "escape_demo",
  "version": "0.0.1",
  "units": [
    { "path": "../welcome.ae", "role": "main" }
  ]
}
'@ -replace "`r`n", "`n"
    $encoding = New-Object System.Text.UTF8Encoding $false
    [System.IO.File]::WriteAllText($negativeProject, "$negativeJson`n", $encoding)
    $previousErrorPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & $Executable project verify $negativeProject 2>&1 | Out-Null
    $negativeExit = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorPreference
    if ($negativeExit -eq 0) {
        Fail "path-escape project unexpectedly verified"
    }
    Write-Host "  path escape rejected (exit $negativeExit) OK"
}
finally {
    if (Test-Path -LiteralPath $workRoot) {
        Remove-Item -LiteralPath $workRoot -Recurse -Force
    }
}

Write-Host ""
Write-Host "PREVIEW VERIFY PASS" -ForegroundColor Green
exit 0
