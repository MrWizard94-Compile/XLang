#requires -Version 5.1
<#
.SYNOPSIS
  Write SHA256 checksums for pack files (portable release aid).
.NOTES
  Paths in the checksum file are pack-relative (forward slashes).
#>
[CmdletBinding()]
param(
    [string]$PackRoot = ""
)

if (-not $PackRoot) { $PackRoot = Split-Path -Parent $PSScriptRoot }
$PackRoot = (Resolve-Path -LiteralPath $PackRoot).Path

$reportDir = Join-Path $PackRoot "reports"
New-Item -ItemType Directory -Force -Path $reportDir | Out-Null
$out = Join-Path $reportDir "CHECKSUMS.sha256"

$files = Get-ChildItem -Path $PackRoot -Recurse -File |
    Where-Object {
        $_.FullName -notmatch '[\\/]_archive' -and
        $_.FullName -notmatch '[\\/]reports[\\/]' -and
        $_.Name -ne "CHECKSUMS.sha256"
    } |
    Sort-Object FullName

$lines = New-Object System.Collections.Generic.List[string]
foreach ($f in $files) {
    $rel = $f.FullName.Substring($PackRoot.Length).TrimStart("\", "/").Replace("\", "/")
    $hash = (Get-FileHash -LiteralPath $f.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    [void]$lines.Add("$hash  $rel")
}

$header = @(
    "# AGENTS Constitution pack checksums (SHA256)",
    "# Pack root relative paths. Generated (UTC): $((Get-Date).ToUniversalTime().ToString('yyyy-MM-dd HH:mm:ss'))Z",
    "# VERSION: $((Get-Content (Join-Path $PackRoot 'VERSION') -ErrorAction SilentlyContinue | Select-Object -First 1))"
)
[IO.File]::WriteAllLines($out, ($header + $lines))
Write-Host "Wrote $out ($($files.Count) files)"
exit 0
