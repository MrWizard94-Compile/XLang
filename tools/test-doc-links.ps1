<#
.SYNOPSIS
  Exercise the documentation link verifier against stable valid and invalid fixtures.

.NOTES
  Rule IDs: TEST-BEHAVIOR-001, DOC-SYNC-001, CONST-GATE-001.
#>
[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

function Fail([string]$Message) {
    Write-Host "FAIL: $Message" -ForegroundColor Red
    exit 1
}

$toolsRoot = (Resolve-Path $PSScriptRoot).Path
$verifier = Join-Path $toolsRoot "verify-doc-links.ps1"
$fixtureRoot = Join-Path $toolsRoot "test-fixtures\doc-links"
$validRoot = Join-Path $fixtureRoot "valid"
$invalidRoots = @(
    (Join-Path $fixtureRoot "invalid"),
    (Join-Path $fixtureRoot "invalid-absolute"),
    (Join-Path $fixtureRoot "invalid-escape")
)

foreach ($path in @($verifier, $validRoot) + $invalidRoots) {
    if (-not (Test-Path -LiteralPath $path)) {
        Fail "required documentation-link test path is missing: $path"
    }
}

& pwsh -NoProfile -File $verifier -RepositoryRoot $validRoot
if ($LASTEXITCODE -ne 0) {
    Fail "valid documentation-link fixture was rejected"
}

foreach ($invalidRoot in $invalidRoots) {
    & pwsh -NoProfile -File $verifier -RepositoryRoot $invalidRoot
    if ($LASTEXITCODE -eq 0) {
        Fail "invalid documentation-link fixture was accepted: $invalidRoot"
    }
}

Write-Host "Documentation link verifier fixture tests PASS."
exit 0
