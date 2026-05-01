param(
    [Parameter(Mandatory = $true)]
    [string]$Version,

    [Parameter(Mandatory = $true)]
    [string]$Target,

    [Parameter(Mandatory = $true)]
    [string]$OutputDir,

    [string]$BinaryPath
)

$ErrorActionPreference = 'Stop'

if ($Target -ne 'x86_64-pc-windows-msvc') {
    throw "Unsupported windows packaging target: $Target"
}

if (-not $BinaryPath) {
    $BinaryPath = Join-Path "target/$Target/release" 'canon.exe'
}

if (-not (Test-Path $BinaryPath)) {
    throw "Expected binary not found at $BinaryPath"
}

New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null

$workDir = Join-Path ([System.IO.Path]::GetTempPath()) ("canon-release-" + [System.Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Path $workDir -Force | Out-Null

try {
    Copy-Item $BinaryPath (Join-Path $workDir 'canon.exe') -Force
    $artifactName = "canon-$Version-windows-x86_64.zip"
    $artifactPath = Join-Path $OutputDir $artifactName
    if (Test-Path $artifactPath) {
        Remove-Item $artifactPath -Force
    }
    Compress-Archive -Path (Join-Path $workDir 'canon.exe') -DestinationPath $artifactPath -Force
    Write-Output $artifactPath
}
finally {
    Remove-Item $workDir -Recurse -Force
}