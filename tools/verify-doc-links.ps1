<#
.SYNOPSIS
  Verify local Markdown links in an Aether checkout or fixture tree.

.DESCRIPTION
  Scans Markdown files for inline links outside fenced code blocks. External,
  mail, and fragment-only links are intentionally ignored. Every local target
  must resolve inside the selected root and exist as a file or directory.

.PARAMETER RepositoryRoot
  Root of the source tree or a focused fixture tree to scan.

.NOTES
  Rule IDs: DOC-SYNC-001, CONST-GATE-001, SEC-INPUT-001.
#>
[CmdletBinding()]
param(
    [string]$RepositoryRoot = (Join-Path $PSScriptRoot "..")
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Fail([string]$Message) {
    Write-Host "FAIL: $Message" -ForegroundColor Red
    exit 1
}

function Test-PathWithinRoot([string]$Root, [string]$Candidate) {
    $rootFull = [System.IO.Path]::GetFullPath($Root)
    $candidateFull = [System.IO.Path]::GetFullPath($Candidate)
    $separator = [System.IO.Path]::DirectorySeparatorChar
    $prefix = if ($rootFull.EndsWith([string]$separator)) { $rootFull } else { "$rootFull$separator" }

    return $candidateFull.Equals($rootFull, [System.StringComparison]::OrdinalIgnoreCase) -or
        $candidateFull.StartsWith($prefix, [System.StringComparison]::OrdinalIgnoreCase)
}

function Get-LocalLinkPath([string]$Destination) {
    $target = $Destination.Trim()
    if ($target.StartsWith("<") -and $target.EndsWith(">") -and $target.Length -ge 2) {
        $target = $target.Substring(1, $target.Length - 2)
    }

    if ($target.Length -eq 0 -or
        $target.StartsWith("#") -or
        $target.StartsWith("//")) {
        return $null
    }

    # A Windows drive path and a file URI are local absolute references, not
    # external URLs. Preserve them so the caller rejects them explicitly.
    if ($target -match '^[A-Za-z]:[\\/]' -or $target -match '^(?i:file):') {
        return $target
    }

    if ($target -match '^[A-Za-z][A-Za-z0-9+.-]*:') {
        return $null
    }

    $fragmentIndex = $target.IndexOf("#")
    if ($fragmentIndex -ge 0) {
        $target = $target.Substring(0, $fragmentIndex)
    }
    $queryIndex = $target.IndexOf("?")
    if ($queryIndex -ge 0) {
        $target = $target.Substring(0, $queryIndex)
    }
    if ($target.Length -eq 0) {
        return $null
    }

    try {
        return [System.Uri]::UnescapeDataString($target)
    }
    catch {
        return $null
    }
}

$rootCandidate = [System.IO.Path]::GetFullPath($RepositoryRoot)
if (-not (Test-Path -LiteralPath $rootCandidate -PathType Container)) {
    Fail "repository root does not exist: $rootCandidate"
}
$root = (Resolve-Path -LiteralPath $rootCandidate).Path

$docsDirectory = Join-Path $root "docs"
if (Test-Path -LiteralPath $docsDirectory -PathType Container) {
    # A repository scan intentionally covers the public documentation surface:
    # root Markdown entry points plus docs/. Tool fixtures, hidden assistant
    # templates, generated output, and vendored governance are not product docs.
    $markdownFiles = @(
        Get-ChildItem -LiteralPath $root -File -Filter *.md
        Get-ChildItem -LiteralPath $docsDirectory -Recurse -File -Filter *.md
    ) | Sort-Object FullName
}
else {
    # Fixture roots have no docs/ directory, so every Markdown file is in scope.
    $markdownFiles = @(
        Get-ChildItem -LiteralPath $root -Recurse -File -Filter *.md |
            Sort-Object FullName
    )
}

$linkPattern = [regex]::new('!?\[[^\]\r\n]*\]\((?<destination><[^>\r\n]+>|[^)\s\r\n]+)(?:\s+(?:"[^"]*"|''[^'']*''))?\)')
$violations = New-Object System.Collections.Generic.List[string]
$checkedLinks = 0
$backtickFence = [string]::new([char]96, 3)

foreach ($file in $markdownFiles) {
    $inFence = $false
    $lineNumber = 0
    foreach ($line in [System.IO.File]::ReadLines($file.FullName)) {
        $lineNumber += 1
        $trimmed = $line.TrimStart()
        if ($trimmed.StartsWith($backtickFence) -or $trimmed.StartsWith("~~~")) {
            $inFence = -not $inFence
            continue
        }
        if ($inFence) {
            continue
        }

        foreach ($match in $linkPattern.Matches($line)) {
            $destination = $match.Groups["destination"].Value
            $localPath = Get-LocalLinkPath $destination
            if ($null -eq $localPath) {
                continue
            }

            $checkedLinks += 1
            $relativeSource = [System.IO.Path]::GetRelativePath($root, $file.FullName).Replace("\", "/")
            if ($localPath -match '^(?i:file):' -or [System.IO.Path]::IsPathRooted($localPath)) {
                $violations.Add(("{0}:{1} absolute local link is forbidden: {2}" -f $relativeSource, $lineNumber, $destination))
                continue
            }

            $candidate = [System.IO.Path]::GetFullPath((Join-Path $file.DirectoryName $localPath))
            if (-not (Test-PathWithinRoot $root $candidate)) {
                $violations.Add(("{0}:{1} link escapes repository root: {2}" -f $relativeSource, $lineNumber, $destination))
                continue
            }
            if (-not (Test-Path -LiteralPath $candidate)) {
                $violations.Add(("{0}:{1} missing local link target: {2}" -f $relativeSource, $lineNumber, $destination))
            }
        }
    }
}

if ($violations.Count -gt 0) {
    Write-Host "Documentation link check FAILED ($($violations.Count) violation(s))" -ForegroundColor Red
    $violations | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}

Write-Host "Documentation link check PASS: $($markdownFiles.Count) Markdown file(s), $checkedLinks local link(s)."
exit 0
