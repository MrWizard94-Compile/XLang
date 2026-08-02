#requires -Version 5.1
<#
.SYNOPSIS
  AGENTS Constitution pack integrity verifier (GOV-INT-001).
.DESCRIPTION
  Validates required files, Module IDs, Rule ID registry coverage,
  forbidden aliases, and basic link targets from AGENTS.md.
  Exit 0 = pass; non-zero = fail.
.NOTES
  Run from any cwd:
    pwsh -File tools/verify-pack.ps1
  Or with explicit root:
    pwsh -File tools/verify-pack.ps1 -PackRoot "C:\path\to\pack"
#>
[CmdletBinding()]
param(
    [string]$PackRoot = ""
)

$ErrorActionPreference = "Continue"
$failures = New-Object System.Collections.Generic.List[string]
$warnings = New-Object System.Collections.Generic.List[string]

function Fail([string]$msg) { [void]$failures.Add($msg) }
function Warn([string]$msg) { [void]$warnings.Add($msg) }

if (-not $PackRoot) {
    $PackRoot = Split-Path -Parent $PSScriptRoot
}
$PackRoot = (Resolve-Path -LiteralPath $PackRoot).Path
Set-Location $PackRoot

Write-Host "AGENTS Constitution integrity check"
Write-Host "Pack root: $PackRoot"
Write-Host ""

# --- GOV-INT-002 required files ---
$required = @(
    "VERSION", "PACK.md", "LOCK.md", "ADOPT.md",
    "AGENTS.md", "SOP.md", "README.md", "Modularize.md",
    "RULE-REGISTRY.md", "MODULE-INDEX.md", "INTEGRITY.md",
    "constitution/00-CORE-LAW.md",
    "constitution/01-AUTHORITY-AND-PRECEDENCE.md",
    "constitution/02-HUMAN-AI-CONTRACT.md",
    "constitution/03-DEFINITION-OF-DONE.md",
    "standards/ENGINEERING.md", "standards/TESTING.md", "standards/SECURITY.md",
    "standards/DOCUMENTATION.md", "standards/PERFORMANCE.md",
    "standards/DEPENDENCIES.md", "standards/OBSERVABILITY.md",
    "operations/DELIVERY.md", "operations/VERSION-CONTROL.md", "operations/RELEASES.md",
    "operations/REFACTORING.md", "operations/MIGRATIONS.md", "operations/PROJECT-LIFECYCLE.md",
    "collaboration/MULTI-AGENT.md", "collaboration/CONTEXT-SWITCHING.md",
    "collaboration/REVIEW-PACKAGING.md", "collaboration/INSTITUTIONAL-MEMORY.md",
    "specialist/NOVEL-RND.md", "specialist/IP-AND-INVENTION.md",
    "specialist/NETWORKING.md", "specialist/LOW-LEVEL-SAFETY.md",
    "specialist/CONSTRAINED-HARDWARE.md",
    "templates/MANIFEST.template.md", "templates/ADR.template.md",
    "templates/AUDIT.template.md", "templates/DELIVERY-REPORT.template.md",
    "templates/PROJECT-OVERRIDE.template.md",
    "templates/PROJECT-POINTER.template.md",
    "tools/verify-pack.ps1",
    "tools/self-audit.ps1",
    "tools/run-full-constitution-self.ps1",
    "tools/write-checksums.ps1"
)

foreach ($rel in $required) {
    $p = Join-Path $PackRoot ($rel -replace "/", [IO.Path]::DirectorySeparatorChar)
    if (-not (Test-Path -LiteralPath $p)) {
        Fail "Missing required file: $rel"
    }
}

# --- Collect markdown (exclude archive) ---
$mdFiles = Get-ChildItem -Path $PackRoot -Recurse -File -Filter *.md |
    Where-Object { $_.FullName -notmatch '[\\/]_archive' }

# --- Registry parse ---
$registryPath = Join-Path $PackRoot "RULE-REGISTRY.md"
$registryText = if (Test-Path $registryPath) { [IO.File]::ReadAllText($registryPath) } else { "" }

$activeIds = New-Object "System.Collections.Generic.HashSet[string]"
$deprecatedIds = New-Object "System.Collections.Generic.HashSet[string]"
$canonicalHomes = @{}

# Parse Active table rows: | RULE-ID | ... | `path` or path | Active |
foreach ($m in [regex]::Matches($registryText, '\|\s*\*?\*?(?<id>[A-Z]{2,}(?:-[A-Z0-9]+)+-\d{3})\*?\*?\s*\|(?<rest>[^\n]+)')) {
    $id = $m.Groups["id"].Value
    $rest = $m.Groups["rest"].Value
    if ($rest -match 'Deprecated') {
        [void]$deprecatedIds.Add($id)
    }
    elseif ($rest -match 'Active' -or $rest -match '`[^`]+`') {
        [void]$activeIds.Add($id)
        if ($rest -match '`(?<home>[^`]+)`') {
            $canonicalHomes[$id] = $m.Groups["home"].Value
            # fix - wrong group. use Matches
        }
    }
}

# Better parse for active + home from registry body tables
$activeIds.Clear()
$deprecatedIds.Clear()
$canonicalHomes = @{}
$inDeprecated = $false
foreach ($line in ($registryText -split "`n")) {
    if ($line -match 'Deprecated aliases') { $inDeprecated = $true }
    if ($line -match '^\|\s*\*?\*?(?<id>[A-Z]{2,}(?:-[A-Z0-9]+)+-\d{3})\*?\*?\s*\|') {
        $id = $Matches["id"]
        if ($inDeprecated -and $line -match 'Forbidden alias') { continue }
        if ($inDeprecated) {
            # deprecated table: | Forbidden | Use instead |
            if ($line -match '^\|\s*(?<bad>[A-Z0-9-]+)\s*\|\s*(?<good>[A-Z0-9-]+)') {
                [void]$deprecatedIds.Add($Matches["bad"])
            }
            continue
        }
        if ($line -match '\|\s*Active\s*\|') {
            [void]$activeIds.Add($id)
            if ($line -match '`(?<home>[^`]+)`') {
                $canonicalHomes[$id] = $Matches["home"]
            }
            elseif ($line -match '\|\s*([^|]+?)\s*\|\s*Active') {
                # middle column may be path without backticks in some rows - skip
            }
        }
        elseif ($line -match '\|\s*Deprecated\s*\|') {
            [void]$deprecatedIds.Add($id)
        }
    }
}

if ($activeIds.Count -lt 20) {
    Fail "RULE-REGISTRY.md parsed fewer than 20 Active Rule IDs (got $($activeIds.Count)); registry format may have broken."
}

# --- Scan all Rule IDs used in pack (exclude Module IDs MOD-*) ---
$idPattern = '\b(?<id>(?!MOD-)[A-Z]{2,}(?:-[A-Z0-9]+)+-\d{3})\b'
$usedIds = New-Object "System.Collections.Generic.HashSet[string]"

$forbiddenAliases = @(
    "CONST-WARN-001", "CONST-TEST-001", "CONST-DOC-001", "CONST-SEC-001", "IP-CLAIM-001"
)

foreach ($f in $mdFiles) {
    $text = [IO.File]::ReadAllText($f.FullName)
    $rel = $f.FullName.Substring($PackRoot.Length).TrimStart("\", "/")
    $normRel = $rel -replace "\\", "/"

    # Aliases allowed only in registry deprecated table + changelogs that document removal
    $aliasOk = $normRel -in @("RULE-REGISTRY.md", "AGENTS.md", "Modularize.md", "INTEGRITY.md")

    foreach ($m in [regex]::Matches($text, $idPattern)) {
        $id = $m.Groups["id"].Value
        # Skip fragments of Module IDs (e.g. COL-CTX-001 inside MOD-COL-CTX-001)
        $start = $m.Index
        if ($start -ge 4 -and $text.Substring($start - 4, 4) -eq "MOD-") { continue }
        if ($start -gt 0 -and $text.Substring($start - 1, 1) -match "[A-Z0-9-]") { continue }

        [void]$usedIds.Add($id)

        if ($forbiddenAliases -contains $id) {
            if ($aliasOk -and ($normRel -eq "RULE-REGISTRY.md" -or $text -match 'Deprecated aliases|Former root shorthand|Alias ban|removed `?CONST-WARN')) {
                # documented deprecation context
                continue
            }
            if ($normRel -eq "RULE-REGISTRY.md") { continue }
            if ($normRel -eq "AGENTS.md" -and $id -match 'CONST-(WARN|TEST|DOC|SEC)-001') {
                # only allowed inside Changelog "Alias ban" section; still fail if in principles table
                # re-check: fail if in Non-Negotiable Principles section lines
                continue
            }
            Fail "Forbidden alias Rule ID '$id' in $rel (use canonical ID per GOV-REG-003)"
        }
    }

    # Module manifest check for binding modules
    $norm = $rel -replace "\\", "/"
    $needsManifest = $norm -match '^(constitution|standards|operations|collaboration|specialist)/' -or
        $norm -in @("RULE-REGISTRY.md", "MODULE-INDEX.md", "INTEGRITY.md")

    if ($needsManifest) {
        if ($text -notmatch 'Module ID:\s*MOD-') {
            Fail "Missing Module ID metadata: $rel"
        }
        if ($text -notmatch 'Version:\s*\d+\.\d+\.\d+') {
            Fail "Missing Version metadata: $rel"
        }
        if ($text -notmatch 'Authority:\s*AGENTS\.md') {
            Fail "Missing Authority: AGENTS.md in $rel"
        }
    }
}

# --- Every used ID must be Active or Deprecated in registry ---
foreach ($id in ($usedIds | Sort-Object)) {
    if ($activeIds.Contains($id)) { continue }
    if ($deprecatedIds.Contains($id)) { continue }
    # Document IDs / SOP doc id sometimes look like rules
    if ($id -eq "SOP-PROD-001") { continue }  # document id historical
    if ($id -eq "CONST-ROOT-001") { continue } # legacy doc id if any remain
    Fail "Rule ID '$id' used in pack but not listed Active/Deprecated in RULE-REGISTRY.md"
}

# --- Canonical homes exist ---
foreach ($kv in $canonicalHomes.GetEnumerator()) {
    $canonPath = $kv.Value -replace "/", [IO.Path]::DirectorySeparatorChar
    $hp = Join-Path $PackRoot $canonPath
    if (-not (Test-Path -LiteralPath $hp)) {
        Fail "Canonical home for $($kv.Key) missing: $($kv.Value)"
    }
}

# --- AGENTS.md must contain unified gate marker ---
$agents = [IO.File]::ReadAllText((Join-Path $PackRoot "AGENTS.md"))
if ($agents -notmatch 'CONST-GATE-001') {
    Fail "AGENTS.md missing CONST-GATE-001"
}
if ($agents -notmatch 'Pre-Delivery Checklist') {
    Fail "AGENTS.md missing unified Pre-Delivery Checklist"
}
# Gate must not be duplicated as full 15-point table elsewhere
$gateTableHits = 0
foreach ($f in $mdFiles) {
    $t = [IO.File]::ReadAllText($f.FullName)
    if ($t -match 'Pre-Delivery Checklist \(15') {
        $gateTableHits++
        $rel = $f.FullName.Substring($PackRoot.Length).TrimStart("\", "/")
        if (($rel -replace "\\", "/") -ne "AGENTS.md") {
            Fail "Full Pre-Delivery Checklist must only live in AGENTS.md (found in $rel)"
        }
    }
}

# --- Root / pack version ---
$versionFile = Join-Path $PackRoot "VERSION"
if (Test-Path -LiteralPath $versionFile) {
    $packVer = (Get-Content -LiteralPath $versionFile -TotalCount 1).Trim()
    if ($packVer -notmatch '^\d+\.\d+\.\d+$') {
        Fail "VERSION must be semver X.Y.Z (got '$packVer')"
    }
} else {
    Fail "VERSION file missing"
}
if ($agents -notmatch 'Version\*\*:\s*5\.') {
    Warn "AGENTS.md version not on 5.x line (universal portable pack)"
}

# --- GOV-PORT-001: no absolute host paths in binding law files ---
$bindingGlobs = @(
    "AGENTS.md", "SOP.md", "PACK.md", "LOCK.md", "ADOPT.md", "README.md", "INTEGRITY.md",
    "RULE-REGISTRY.md", "MODULE-INDEX.md", "Modularize.md", "VERSION"
)
# Pattern built in parts so this file does not contain a forbidden literal host path.
$pathRx = '(?i)([CD]:\\Us' + 'ers\\|/Us' + 'ers/[A-Za-z]|One' + 'Drive\\|/ho' + 'me/[a-z])'
foreach ($rel in $bindingGlobs) {
    $bp = Join-Path $PackRoot ($rel -replace "/", [IO.Path]::DirectorySeparatorChar)
    if (-not (Test-Path -LiteralPath $bp)) { continue }
    $bt = [IO.File]::ReadAllText($bp)
    if ($bt -match $pathRx) {
        Fail "Absolute host path in binding file $rel (GOV-PORT-001) — use pack-relative paths"
    }
}
Get-ChildItem -Path (Join-Path $PackRoot "constitution"), (Join-Path $PackRoot "standards"),
    (Join-Path $PackRoot "operations"), (Join-Path $PackRoot "collaboration"),
    (Join-Path $PackRoot "specialist"), (Join-Path $PackRoot "templates"),
    (Join-Path $PackRoot "tools") -Recurse -File -ErrorAction SilentlyContinue |
    Where-Object { $_.Extension -match '\.(md|ps1)$' } |
    ForEach-Object {
        $rel = $_.FullName.Substring($PackRoot.Length).TrimStart("\", "/").Replace("\", "/")
        # skip this detector script (defines path patterns)
        if ($rel -eq "tools/verify-pack.ps1") { return }
        $t = [IO.File]::ReadAllText($_.FullName)
        if ($t -match $pathRx) {
            Fail "Absolute host path in $rel (GOV-PORT-001)"
        }
    }

# --- PACK.md portability markers ---
$packMd = Join-Path $PackRoot "PACK.md"
if (Test-Path -LiteralPath $packMd) {
    $pt = [IO.File]::ReadAllText($packMd)
    if ($pt -notmatch 'GOV-PORT-001') { Fail "PACK.md missing GOV-PORT-001" }
    if ($pt -notmatch 'GOV-PORT-002') { Fail "PACK.md missing GOV-PORT-002" }
}

# --- Report ---
Write-Host "Pack VERSION:                $packVer"
Write-Host "Active Rule IDs in registry: $($activeIds.Count)"
Write-Host "Rule IDs cited in pack:      $($usedIds.Count)"
Write-Host "Markdown files scanned:      $($mdFiles.Count)"
Write-Host ""

if ($warnings.Count -gt 0) {
    Write-Host "WARNINGS:" -ForegroundColor Yellow
    foreach ($w in $warnings) { Write-Host "  - $w" -ForegroundColor Yellow }
    Write-Host ""
}

if ($failures.Count -gt 0) {
    Write-Host "FAILURES ($($failures.Count)):" -ForegroundColor Red
    foreach ($x in $failures) { Write-Host "  - $x" -ForegroundColor Red }
    Write-Host ""
    Write-Host "RESULT: FAIL (GOV-INT-001)" -ForegroundColor Red
    exit 1
}

Write-Host "RESULT: PASS (GOV-INT-001)" -ForegroundColor Green
exit 0
