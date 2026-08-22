[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

if ($PSVersionTable.PSVersion.Major -ge 7) {
    $PSNativeCommandUseErrorActionPreference = $false
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Actual,
        [Parameter(Mandatory = $true)]
        [string]$Expected,
        [Parameter(Mandatory = $true)]
        [string]$Label
    )

    if (-not $Actual.Contains($Expected, [System.StringComparison]::Ordinal)) {
        throw ($Label + ' did not contain expected text: ' + $Expected)
    }
}

function Assert-Equal {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Actual,
        [Parameter(Mandatory = $true)]
        [string]$Expected,
        [Parameter(Mandatory = $true)]
        [string]$Label
    )

    if (-not [string]::Equals($Actual, $Expected, [System.StringComparison]::Ordinal)) {
        throw ($Label + ' differed. Actual: ' + $Actual + '; expected: ' + $Expected)
    }
}

function Invoke-Aether {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    $output = (& cargo run -q -p aether-cli -- @Arguments 2>&1 | Out-String)
    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        throw ('aether ' + ($Arguments -join ' ') + ' failed with exit ' + $exitCode + [Environment]::NewLine + $output)
    }

    return $output
}

function Invoke-AetherFailure {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    $output = (& cargo run -q -p aether-cli -- @Arguments 2>&1 | Out-String)
    $exitCode = $LASTEXITCODE
    if ($exitCode -eq 0) {
        throw ('aether ' + ($Arguments -join ' ') + ' unexpectedly succeeded' + [Environment]::NewLine + $output)
    }

    return $output
}

function Save-Evidence {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        [Parameter(Mandatory = $true)]
        [string]$Content
    )

    $destination = Join-Path $script:EvidenceRoot $Name
    [System.IO.File]::WriteAllText(
        $destination,
        $Content,
        (New-Object System.Text.UTF8Encoding($false))
    )
}

$scriptDirectory = Split-Path -Parent $PSCommandPath
$showcaseRoot = Split-Path -Parent $scriptDirectory
$repositoryRoot = (Resolve-Path (Join-Path $showcaseRoot '..\..')).Path
$workspace = Join-Path $showcaseRoot 'aether.workspace.json'
$protocolProject = Join-Path $showcaseRoot 'protocol\aether.project.json'
$faultProject = Join-Path $showcaseRoot 'fault\aether.project.json'
$replayTest = Join-Path $showcaseRoot 'replay\resource_test.ae'
$faultTest = Join-Path $showcaseRoot 'fault\task_success_test.ae'
$faultSource = Join-Path $showcaseRoot 'fault\main.ae'
$operatorSource = Join-Path $showcaseRoot 'operator\main.ae'
$fixtureRoot = Join-Path $showcaseRoot 'fixtures'
$corruptFixtureRoot = Join-Path $fixtureRoot 'corrupt'

$evidenceName = 'aether-atlas-evidence-' + [guid]::NewGuid().ToString('N')
$script:EvidenceRoot = Join-Path $repositoryRoot ('target\' + $evidenceName)
if (Test-Path -LiteralPath $script:EvidenceRoot) {
    throw ('refusing to reuse evidence directory: ' + $script:EvidenceRoot)
}

New-Item -ItemType Directory -Path $script:EvidenceRoot | Out-Null

Push-Location $repositoryRoot
try {
    $packVerifier = Join-Path $repositoryRoot '..\..\AGENTS Constitution\tools\verify-pack.ps1'
    $packOutput = (& pwsh -NoProfile -File $packVerifier 2>&1 | Out-String)
    if ($LASTEXITCODE -ne 0) {
        throw ('AGENTS Constitution pack verification failed' + [Environment]::NewLine + $packOutput)
    }
    Save-Evidence 'pack-verify.txt' $packOutput

    $workspaceOutput = Invoke-Aether @('workspace', 'verify', $workspace)
    Save-Evidence 'workspace-verify.txt' $workspaceOutput

    $protocolTests = Invoke-Aether @('project', 'test', $protocolProject)
    Save-Evidence 'protocol-tests.txt' $protocolTests

    $faultTests = Invoke-Aether @('project', 'test', $faultProject)
    Save-Evidence 'fault-tests.txt' $faultTests

    $testReport = Join-Path $script:EvidenceRoot 'standalone-tests.json'
    $standaloneTests = Invoke-Aether @('test', $replayTest, $faultTest, '--report', $testReport)
    Save-Evidence 'standalone-tests.txt' $standaloneTests

    $auditArtifact = Join-Path $script:EvidenceRoot 'audit.aeth'
    $auditBuild = Invoke-Aether @('workspace', 'build', $workspace, '--package', 'audit', '--output', $auditArtifact)
    $auditRun = Invoke-Aether @('run', $auditArtifact)
    Assert-Contains $auditRun 'exited with 0' 'cross-package policy audit exit'
    Save-Evidence 'audit-build.txt' $auditBuild
    Save-Evidence 'audit-run.txt' $auditRun

    $replayOne = Join-Path $script:EvidenceRoot 'replay-one.aeth'
    $replayTwo = Join-Path $script:EvidenceRoot 'replay-two.aeth'
    $replayBuildOne = Invoke-Aether @('workspace', 'build', $workspace, '--package', 'replay', '--output', $replayOne)
    $replayBuildTwo = Invoke-Aether @('workspace', 'build', $workspace, '--package', 'replay', '--output', $replayTwo)
    Save-Evidence 'replay-build-one.txt' $replayBuildOne
    Save-Evidence 'replay-build-two.txt' $replayBuildTwo

    $replayHashOne = (Get-FileHash -Algorithm SHA256 -LiteralPath $replayOne).Hash.ToLowerInvariant()
    $replayHashTwo = (Get-FileHash -Algorithm SHA256 -LiteralPath $replayTwo).Hash.ToLowerInvariant()
    Assert-Equal $replayHashOne $replayHashTwo 'deterministic replay artifact hash'

    $replayRun = Invoke-Aether @('run', $replayOne)
    Assert-Contains $replayRun 'atlas-replay' 'pure replay output'
    Assert-Contains $replayRun 'exited with 4720242' 'pure replay exit'
    Save-Evidence 'replay-run.txt' $replayRun

    $operatorArtifact = Join-Path $script:EvidenceRoot 'operator.aeth'
    $operatorBuild = Invoke-Aether @('workspace', 'build', $workspace, '--package', 'operator', '--output', $operatorArtifact)
    Save-Evidence 'operator-build.txt' $operatorBuild

    $operatorStructure = Invoke-Aether @('structure', $operatorSource)
    Assert-Contains $operatorStructure 'aether.product-structure/v1' 'operator product structure'
    Save-Evidence 'operator.product-structure.json' $operatorStructure

    $ungraftedOperator = Invoke-AetherFailure @('run', $operatorArtifact)
    Assert-Contains $ungraftedOperator 'AE-HOST-003' 'ungranted operator'
    Save-Evidence 'operator-ungranted.txt' $ungraftedOperator

    $validOutputRoot = Join-Path $script:EvidenceRoot 'operator-valid'
    $corruptOutputRoot = Join-Path $script:EvidenceRoot 'operator-corrupt'
    New-Item -ItemType Directory -Path $validOutputRoot, $corruptOutputRoot | Out-Null

    $validOperator = Invoke-Aether @(
        'run',
        $operatorArtifact,
        '--grant-read',
        $fixtureRoot,
        '--grant-write',
        $validOutputRoot
    )
    Assert-Contains $validOperator 'exited with 20' 'valid operator exit'
    $validReceipt = [System.IO.File]::ReadAllText((Join-Path $validOutputRoot 'atlas.receipt'))
    Assert-Equal $validReceipt 'ATLAS-ACCEPTED 10650' 'valid operator receipt'
    Save-Evidence 'operator-valid-run.txt' $validOperator

    $corruptOperator = Invoke-Aether @(
        'run',
        $operatorArtifact,
        '--grant-read',
        $corruptFixtureRoot,
        '--grant-write',
        $corruptOutputRoot
    )
    Assert-Contains $corruptOperator 'exited with 14' 'corrupt operator exit'
    $corruptReceipt = [System.IO.File]::ReadAllText((Join-Path $corruptOutputRoot 'atlas.receipt'))
    Assert-Equal $corruptReceipt 'ATLAS-REJECTED' 'corrupt operator receipt'
    Save-Evidence 'operator-corrupt-run.txt' $corruptOperator

    $faultArtifact = Join-Path $script:EvidenceRoot 'fault.aeth'
    $faultBuild = Invoke-Aether @('workspace', 'build', $workspace, '--package', 'fault', '--output', $faultArtifact)
    Save-Evidence 'fault-build.txt' $faultBuild

    $faultBytes = [System.IO.File]::ReadAllBytes($faultArtifact)
    if ($faultBytes.Length -lt 5 -or
        $faultBytes[0] -ne 65 -or
        $faultBytes[1] -ne 69 -or
        $faultBytes[2] -ne 84 -or
        $faultBytes[3] -ne 72 -or
        $faultBytes[4] -ne 12) {
        throw 'fault artifact is not an AETH v12 task-frame artifact'
    }

    $faultRun = Invoke-Aether @('run', $faultArtifact)
    Assert-Contains $faultRun 'exited with 91' 'fault persona exit'
    Save-Evidence 'fault-run.txt' $faultRun

    $faultStructure = Invoke-Aether @('structure', $faultSource)
    Assert-Contains $faultStructure 'aether.product-structure/v1' 'fault product structure'
    Save-Evidence 'fault.product-structure.json' $faultStructure

    $tamperedArtifact = Join-Path $script:EvidenceRoot 'replay-tampered.aeth'
    $tamperedBytes = [System.IO.File]::ReadAllBytes($replayOne)
    $tamperedBytes[$tamperedBytes.Length - 1] = $tamperedBytes[$tamperedBytes.Length - 1] -bxor 1
    [System.IO.File]::WriteAllBytes($tamperedArtifact, $tamperedBytes)
    $tamperedRun = Invoke-AetherFailure @('run', $tamperedArtifact)
    Assert-Contains $tamperedRun 'Aether artifact error' 'tampered artifact refusal'
    Save-Evidence 'replay-tampered.txt' $tamperedRun

    $bundle = Join-Path $script:EvidenceRoot 'atlas-protocol.bundle'
    $cache = Join-Path $script:EvidenceRoot 'package-cache'
    $installed = Join-Path $script:EvidenceRoot 'atlas-protocol-installed'
    $packagePack = Invoke-Aether @('pkg', 'pack', $protocolProject, '--output', $bundle)
    $packageVerify = Invoke-Aether @('pkg', 'verify', $bundle)
    $packagePublish = Invoke-Aether @('pkg', 'publish', $bundle, '--cache', $cache)
    $packageCacheVerify = Invoke-Aether @('pkg', 'verify-cache', $cache)
    $packageInstall = Invoke-Aether @(
        'pkg',
        'install',
        '--cache',
        $cache,
        '--name',
        'atlas_protocol',
        '--version',
        '0.1.0',
        '--output',
        $installed
    )
    $installedVerify = Invoke-Aether @('project', 'verify', (Join-Path $installed 'aether.project.json'))
    Save-Evidence 'package-pack.txt' $packagePack
    Save-Evidence 'package-verify.txt' $packageVerify
    Save-Evidence 'package-publish.txt' $packagePublish
    Save-Evidence 'package-cache-verify.txt' $packageCacheVerify
    Save-Evidence 'package-install.txt' $packageInstall
    Save-Evidence 'package-installed-verify.txt' $installedVerify

    $summary = @(
        '# Aether Atlas verification evidence',
        '',
        ('Replay artifact SHA-256: {0}' -f $replayHashOne),
        'Replay deterministic hash match: true',
        'Cross-package policy audit exit: 0',
        'Replay runtime exit: 4720242',
        ('Valid operator receipt: {0}' -f $validReceipt),
        ('Corrupt operator receipt: {0}' -f $corruptReceipt),
        'Fault artifact version: AETH v12',
        'Fault runtime exit: 91',
        'Tampered replay artifact: rejected before execution',
        'M25 protocol bundle/cache/install: verified',
        '',
        ('Evidence directory: {0}' -f $script:EvidenceRoot)
    ) -join [Environment]::NewLine
    Save-Evidence 'SUMMARY.md' ($summary + [Environment]::NewLine)

    Write-Output $summary
}
finally {
    Pop-Location
}
