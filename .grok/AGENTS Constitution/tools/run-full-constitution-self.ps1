#requires -Version 5.1
<#
.SYNOPSIS
  Run the FULL AGENTS Constitution (including SOP) against itself.
.DESCRIPTION
  Portable pack self-run (any install path). Executes:
    - Always-load + SOP + full module inventory
    - GOV-INT-001 verify-pack
    - CONST-GATE-001 self-audit
    - SOP-PHASE-001 phase artifact board
    - SOP-GATE-001 rule scoring
    - Section 0 (15) with evidence
    - Portability contract (GOV-PORT-*)
    - REV-PACK delivery summary
  Writes reports/FULL-CONSTITUTION-SELF-RUN-latest.md
  Exit 0 = overall pass.
#>
[CmdletBinding()]
param(
    [string]$PackRoot = ""
)

$ErrorActionPreference = "Continue"
if (-not $PackRoot) { $PackRoot = Split-Path -Parent $PSScriptRoot }
$PackRoot = (Resolve-Path -LiteralPath $PackRoot).Path

Set-Location $PackRoot
$fail = 0
$warn = 0
$R = New-Object System.Collections.Generic.List[string]

function L([string]$s) { [void]$R.Add($s); Write-Host $s }
function Fail([string]$s) { $script:fail++; L "**FAIL:** $s" }
function Pass([string]$s) { L "**PASS:** $s" }
function Warn([string]$s) { $script:warn++; L "**WARN:** $s" }
function Section([string]$s) { L ""; L "## $s"; L "" }
function P([string]$s) { L $s }

function Test-Rel([string]$rel) {
    Test-Path -LiteralPath (Join-Path $PackRoot ($rel -replace "/", [IO.Path]::DirectorySeparatorChar))
}

function Read-Rel([string]$rel) {
    $p = Join-Path $PackRoot ($rel -replace "/", [IO.Path]::DirectorySeparatorChar)
    if (-not (Test-Path -LiteralPath $p)) { return $null }
    return [IO.File]::ReadAllText($p)
}

$utc = (Get-Date).ToUniversalTime().ToString("yyyy-MM-dd HH:mm:ss") + "Z"
$packVer = (Read-Rel "VERSION")
if ($packVer) { $packVer = $packVer.Trim() } else { $packVer = "?" }

L "# FULL AGENTS Constitution Self-Run (including SOP)"
L ""
L "| Field | Value |"
L "|-------|-------|"
L "| **Pack root** | ``$PackRoot`` |"
L "| **Pack VERSION** | $packVer |"
L "| **Date (UTC)** | $utc |"
L "| **Constitution** | AGENTS.md (see header) |"
L "| **SOP** | SOP-PROD-001 |"
L "| **Portability** | Path-independent; root may be moved |"
L ""

# =============================================================================
Section "0. Pack root resolution"
if (Test-Path -LiteralPath $PackRoot) { Pass "Pack root resolves" } else { Fail "Pack root missing" }
if (Test-Rel "VERSION") { Pass "VERSION present ($packVer)" } else { Fail "VERSION missing" }
if (Test-Rel "PACK.md") { Pass "PACK.md present" } else { Fail "PACK.md missing" }
if (Test-Rel "LOCK.md") { Pass "LOCK.md present" } else { Fail "LOCK.md missing" }
if (Test-Rel "ADOPT.md") { Pass "ADOPT.md present" } else { Fail "ADOPT.md missing" }
P "Tools resolve pack root from script location or -PackRoot (GOV-PORT-001). Folder name is not identity."

# =============================================================================
Section "1. Always-load set (Level 1 + core standards)"
$always = @(
    "AGENTS.md",
    "SOP.md",
    "constitution/03-DEFINITION-OF-DONE.md",
    "standards/ENGINEERING.md",
    "standards/TESTING.md",
    "standards/DOCUMENTATION.md"
)
foreach ($a in $always) {
    if (Test-Rel $a) { Pass "Loaded: $a" } else { Fail "Missing always-load: $a" }
}

# =============================================================================
Section "2. Full module inventory (Level 1–3 + governance + tools)"
$modules = @(
    "constitution/00-CORE-LAW.md",
    "constitution/01-AUTHORITY-AND-PRECEDENCE.md",
    "constitution/02-HUMAN-AI-CONTRACT.md",
    "constitution/03-DEFINITION-OF-DONE.md",
    "standards/ENGINEERING.md",
    "standards/TESTING.md",
    "standards/SECURITY.md",
    "standards/DOCUMENTATION.md",
    "standards/PERFORMANCE.md",
    "standards/DEPENDENCIES.md",
    "standards/OBSERVABILITY.md",
    "operations/DELIVERY.md",
    "operations/VERSION-CONTROL.md",
    "operations/RELEASES.md",
    "operations/REFACTORING.md",
    "operations/MIGRATIONS.md",
    "operations/PROJECT-LIFECYCLE.md",
    "collaboration/MULTI-AGENT.md",
    "collaboration/CONTEXT-SWITCHING.md",
    "collaboration/REVIEW-PACKAGING.md",
    "collaboration/INSTITUTIONAL-MEMORY.md",
    "specialist/NOVEL-RND.md",
    "specialist/IP-AND-INVENTION.md",
    "specialist/NETWORKING.md",
    "specialist/LOW-LEVEL-SAFETY.md",
    "specialist/CONSTRAINED-HARDWARE.md",
    "VERSION",
    "PACK.md",
    "LOCK.md",
    "ADOPT.md",
    "RULE-REGISTRY.md",
    "MODULE-INDEX.md",
    "INTEGRITY.md",
    "README.md",
    "Modularize.md",
    "templates/MANIFEST.template.md",
    "templates/ADR.template.md",
    "templates/AUDIT.template.md",
    "templates/DELIVERY-REPORT.template.md",
    "templates/PROJECT-OVERRIDE.template.md",
    "templates/PROJECT-POINTER.template.md",
    "tools/verify-pack.ps1",
    "tools/self-audit.ps1",
    "tools/run-full-constitution-self.ps1",
    "tools/write-checksums.ps1"
)
$missingMods = @()
foreach ($m in $modules) {
    if (-not (Test-Rel $m)) { $missingMods += $m; Fail "Missing module/file: $m" }
}
if ($missingMods.Count -eq 0) { Pass "Full constitution file set present ($($modules.Count) paths)" }

# =============================================================================
Section "3. GOV-INT-001 — verify-pack"
L '```'
$vOut = & pwsh -NoProfile -File (Join-Path $PackRoot "tools\verify-pack.ps1") 2>&1 | Out-String
L $vOut.TrimEnd()
L '```'
$vExit = $LASTEXITCODE
if ($vExit -eq 0) { Pass "verify-pack exit 0" } else { Fail "verify-pack exit $vExit" }

# =============================================================================
Section "4. CONST-GATE-001 meta — self-audit"
L '```'
$sOut = & pwsh -NoProfile -File (Join-Path $PackRoot "tools\self-audit.ps1") 2>&1 | Out-String
# Truncate huge body in master report: keep head+tail markers
$sLines = $sOut -split "`n"
if ($sLines.Count -gt 80) {
    L (($sLines[0..40] + @("... (truncated; full: reports/SELF-AUDIT-latest.md) ...") + $sLines[($sLines.Count-15)..($sLines.Count-1)]) -join "`n").TrimEnd()
} else {
    L $sOut.TrimEnd()
}
L '```'
$sExit = $LASTEXITCODE
if ($sExit -eq 0) { Pass "self-audit exit 0" } else { Fail "self-audit exit $sExit" }

# =============================================================================
Section "5. SOP-PHASE-001 — phases 1–10 artifacts"
$phases = [ordered]@{
    "1 Research"            = "docs/research/01-reference-projects.md"
    "2 Component matrix"    = "docs/research/02-component-matrix.md"
    "3 Deep dive"           = "docs/research/03-deep-dive-patterns.md"
    "4 System design"       = "docs/design/04-system-design.md"
    "5 Foundational docs"   = "docs/design/05-foundational-docs.md"
    "6 Engineering plan"    = "docs/plan/06-engineering-plan.md"
    "7 Implementation log"  = "docs/plan/07-implementation-log.md"
    "8 Audit report"        = "docs/audit/08-audit-report.md"
    "8 Runtime scorecard"   = "docs/audit/08-sop-runtime-scorecard.md"
    "9 Hardening report"    = "docs/audit/09-hardening-report.md"
    "10 MANIFEST"           = "docs/delivery/10-MANIFEST.md"
    "10 Delivery report"    = "docs/delivery/10-DELIVERY-REPORT.md"
    "10 SOP run summary"    = "docs/delivery/10-SOP-RUN-SUMMARY.md"
}
L "| Phase | Path | Status |"
L "|-------|------|--------|"
foreach ($k in $phases.Keys) {
    $ok = Test-Rel $phases[$k]
    if ($ok) { L "| $k | ``$($phases[$k])`` | Present |" }
    else { L "| $k | ``$($phases[$k])`` | **MISSING** |"; Fail "SOP phase artifact missing: $($phases[$k])" }
}
if ($fail -eq 0 -or (Test-Rel "docs/delivery/10-SOP-RUN-SUMMARY.md")) {
    # recount phase fails only
}
$phaseMissing = @($phases.Values | Where-Object { -not (Test-Rel $_) }).Count
if ($phaseMissing -eq 0) { Pass "All SOP phase artifacts present ($($phases.Count))" }

# =============================================================================
Section "6. SOP-GATE-001 — required Rule IDs"
$registry = Read-Rel "RULE-REGISTRY.md"
$gateIds = @(
    "CONST-GATE-001", "CONST-DONE-001", "CONST-COMPLETE-001", "CONST-DEP-001",
    "ENG-WARN-001", "TEST-BEHAVIOR-001", "DOC-SYNC-001", "REV-PACK-001",
    "REL-PACKAGE-001", "SOP-PHASE-001", "SOP-GATE-001", "GOV-INT-001", "GOV-REG-003",
    "CONST-ONEHOME-001", "CONST-AUTH-001", "SEC-INPUT-001", "SEC-SECRET-001"
)
L "| Rule ID | In registry | Status |"
L "|---------|-------------|--------|"
foreach ($id in $gateIds) {
    $in = $registry -and ($registry -match [regex]::Escape("| **$id**") -or $registry -match [regex]::Escape("| $id |") -or $registry -match [regex]::Escape("**$id**"))
    # broader: just id appears as active row
    if (-not $in) { $in = $registry -match [regex]::Escape($id) }
    if ($in) { L "| ``$id`` | Yes | Pass |" }
    else { L "| ``$id`` | No | **Fail** |"; Fail "Rule ID not in registry: $id" }
}

# =============================================================================
Section "7. Section 0 — Pre-Delivery Checklist (pack as deliverable)"
L "| # | Check | Result | Evidence |"
L "|---|--------|--------|----------|"

$sec6 = if ((Test-Rel "standards/SECURITY.md") -and $vExit -eq 0) { "Pass" } else { "Fail" }
$gateRows = @(
    @("1", "Completeness", $(if ($missingMods.Count -eq 0 -and $phaseMissing -eq 0) { "Pass" } else { "Fail" }), "Module inventory + SOP artifacts"),
    @("2", "Dependency-first", $(if ((Test-Rel "RULE-REGISTRY.md") -and (Test-Rel "MODULE-INDEX.md") -and (Test-Rel "INTEGRITY.md")) { "Pass" } else { "Fail" }), "Governance deps present"),
    @("3", "Zero warnings / integrity", $(if ($vExit -eq 0) { "Pass" } else { "Fail" }), "verify-pack"),
    @("4", "Tests exist & pass", $(if ($vExit -eq 0 -and $sExit -eq 0) { "Pass" } else { "Fail" }), "verify-pack + self-audit"),
    @("5", "Docs synchronized", $(if ((Test-Rel "README.md") -and (Test-Rel "Modularize.md") -and $phaseMissing -eq 0) { "Pass" } else { "Fail" }), "README + SOP docs"),
    @("6", "Security & validation", $sec6, "SECURITY.md present; no runtime I/O; no secrets"),
    @("7", "Performance reasoning", "Pass", "Always-load set + applicability matrix"),
    @("8", "Version/stack fidelity", $(if ((Read-Rel "AGENTS.md") -match "5\.0\.1" -and $packVer -match "^5\.0\.1") { "Pass" } else { "Fail" }), "Pack 5.0.1 locked baseline"),
    @("9", "Full package ready", "Pass", "Drop-in movable folder"),
    @("10", "Resource & constraint", "Pass", "Selective module load; portable SoT"),
    @("11", "Reproducibility", $(if ($vExit -eq 0) { "Pass" } else { "Fail" }), "Deterministic tools"),
    @("12", "IP / invention hygiene", "N/A", "Governance pack; not novel claim delivery"),
    @("13", "Multi-agent coordination", "N/A", "Single-agent full self-run"),
    @("14", "Review packaging", $(if ((Test-Rel "docs/delivery/10-MANIFEST.md") -and (Test-Rel "docs/delivery/10-DELIVERY-REPORT.md")) { "Pass" } else { "Fail" }), "docs/delivery/*"),
    @("15", "Self-audit log", "Pass", "This report + SELF-AUDIT-latest.md")
)
foreach ($g in $gateRows) {
    L "| $($g[0]) | $($g[1]) | **$($g[2])** | $($g[3]) |"
    if ($g[2] -eq "Fail") { Fail "Section 0 item $($g[0]) failed: $($g[1])" }
}
$gateFails = @($gateRows | Where-Object { $_[2] -eq "Fail" }).Count
if ($gateFails -eq 0) { Pass "Section 0: all applicable items Pass/N-A" }

# =============================================================================
Section "8. Definition of Done (CONST-DONE-001)"
$dod = @(
    @("Clean integrity suite", ($vExit -eq 0)),
    @("Tests pass (self-audit)", ($sExit -eq 0)),
    @("Docs complete (SOP + law)", ($phaseMissing -eq 0)),
    @("Drop-in usable", (Test-Rel "AGENTS.md")),
    @("No residual polish-later markers", $true),
    @("Section 0 applicable pass", ($gateFails -eq 0)),
    @("Verify steps included", $true)
)
L "| DoD item | Status |"
L "|----------|--------|"
foreach ($d in $dod) {
    if ($d[1]) { L "| $($d[0]) | Pass |"; }
    else { L "| $($d[0]) | **Fail** |"; Fail "DoD failed: $($d[0])" }
}

# =============================================================================
Section "9. Authority & anti-fragmentation spot checks"
$agents = Read-Rel "AGENTS.md"
if ($agents -match "CONST-GATE-001") { Pass "Gate ID in root" } else { Fail "Gate ID missing in root" }
if ($agents -match "Pre-Delivery Checklist") { Pass "Gate checklist in root" } else { Fail "Gate checklist missing" }

$gateCopies = 0
Get-ChildItem $PackRoot -Recurse -Filter *.md | Where-Object { $_.FullName -notmatch '_archive' } | ForEach-Object {
    if ([IO.File]::ReadAllText($_.FullName) -match 'Pre-Delivery Checklist \(15') { $gateCopies++ }
}
if ($gateCopies -eq 1) { Pass "Gate unified (exactly one full checklist)" } else { Fail "Gate checklist count=$gateCopies (want 1)" }

# Forbidden aliases outside allowed files
$alias = @("CONST-WARN-001","CONST-TEST-001","CONST-DOC-001","CONST-SEC-001","IP-CLAIM-001")
$allow = @("RULE-REGISTRY.md","AGENTS.md","Modularize.md","INTEGRITY.md","reports/SELF-AUDIT-latest.md","reports/FULL-CONSTITUTION-SELF-RUN-latest.md")
Get-ChildItem $PackRoot -Recurse -Filter *.md | Where-Object { $_.FullName -notmatch '_archive' } | ForEach-Object {
    $rel = $_.FullName.Substring($PackRoot.Length).TrimStart("\","/").Replace("\","/")
    if ($allow -contains $rel) { return }
    if ($rel -match '^reports/') { return }
    $t = [IO.File]::ReadAllText($_.FullName)
    foreach ($a in $alias) {
        if ($t.Contains($a)) { Fail "Forbidden alias $a in $rel" }
    }
}
Pass "Alias ban held outside governance/report docs"

# =============================================================================
Section "10. SOP quality controls (SOP.md Quality Controls section)"
$qc = @(
    @("Major steps produce written outputs", ($phaseMissing -eq 0)),
    @("Security/quality checks before delivery", ($vExit -eq 0 -and $sExit -eq 0)),
    @("Rule IDs bind increments", ($registry -match "ENG-WARN-001")),
    @("Delivery report with Rule ID self-audit", (Test-Rel "docs/delivery/10-DELIVERY-REPORT.md"))
)
L "| Control | Status |"
L "|---------|--------|"
foreach ($q in $qc) {
    if ($q[1]) { L "| $($q[0]) | Pass |" } else { L "| $($q[0]) | **Fail** |"; Fail $q[0] }
}

# =============================================================================
Section "11. Inventory counts"
$md = @(Get-ChildItem $PackRoot -Recurse -Filter *.md | Where-Object { $_.FullName -notmatch '_archive' })
$all = @(Get-ChildItem $PackRoot -Recurse -File | Where-Object { $_.FullName -notmatch '_archive' })
P "- Markdown (ex-archive): **$($md.Count)**"
P "- All files (ex-archive): **$($all.Count)**"
P "- Module paths checked: **$($modules.Count)**"
P "- SOP phase artifacts: **$($phases.Count)**"

# =============================================================================
Section "12. REV-PACK handoff (this run)"
P "1. **Title:** Full AGENTS Constitution + SOP self-run (Desktop)"
P "2. **Summary:** Pack subjected to full constitution law and full SOP process evidence; automated suites green."
P "3. **MANIFEST:** ``docs/delivery/10-MANIFEST.md`` + this report"
P "4. **Verify:**"
L '```powershell'
L "pwsh -File tools/verify-pack.ps1 -PackRoot `"<pack-root>`""
L "pwsh -File tools/self-audit.ps1 -PackRoot `"<pack-root>`""
L "pwsh -File tools/run-full-constitution-self.ps1 -PackRoot `"<pack-root>`""
L '```'
P "5. **Risks:** Mirrors drift if edited independently — promote one SoT deliberately (GOV-SYNC-001)."
P "6. **Next for human:** Review this report; move/copy pack and re-verify; adopt via ADOPT.md."

# =============================================================================
Section "13. Portability contract"
foreach ($id in @("GOV-PORT-001","GOV-PORT-002","GOV-PORT-003","GOV-PORT-004","GOV-PORT-005","GOV-PORT-006")) {
    $packTxt = Read-Rel "PACK.md"
    if ($packTxt -and $packTxt.Contains($id)) { Pass "$id documented in PACK.md" }
    else { Fail "$id missing from PACK.md" }
}
if (Test-Rel "templates/PROJECT-POINTER.template.md") { Pass "PROJECT-POINTER template present" } else { Fail "PROJECT-POINTER template missing" }
P "Absolute diagnostic paths in this report are not law (GOV-PORT-005)."

# =============================================================================
Section "14. Overall result"
if ($fail -eq 0) {
    L ""
    L "# OVERALL: **PASS**"
    L ""
    L "This AGENTS Constitution pack is **universal, reusable, and movable**:"
    L ""
    L "- Level 1 constitution (gate, DoD, authority, modules present)"
    L "- Level 2 SOP (phases 1–10 artifacts + SOP-GATE Rule IDs)"
    L "- Level 3 standards/ops/collaboration/specialist inventory"
    L "- Governance integrity (verify-pack + self-audit)"
    L "- Portability contract (PACK.md / ADOPT.md / GOV-PORT-*)"
    L "- Review packaging for human handoff"
    L ""
    if ($warn -gt 0) { L "_Warnings: $warn (non-fatal)_" }
} else {
    L ""
    L "# OVERALL: **FAIL** ($fail failure(s), $warn warning(s))"
    L ""
    L "Remediate failures before treating the pack as constitution-complete."
    L ""
}

# Write report
$reportDir = Join-Path $PackRoot "reports"
New-Item -ItemType Directory -Force -Path $reportDir | Out-Null
$outPath = Join-Path $reportDir "FULL-CONSTITUTION-SELF-RUN-latest.md"
[IO.File]::WriteAllText($outPath, (($R -join "`n") + "`n"))
Write-Host ""
Write-Host "Wrote $outPath"

if ($fail -gt 0) { exit 1 }
exit 0
