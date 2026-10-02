$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
$manifest = Join-Path $repoRoot 'rdp-reverse-agent\Cargo.toml'
$source = Join-Path $repoRoot 'rdp-reverse-agent\target\release\rdp-reverse-agent.exe'
$outputDir = Join-Path $repoRoot 'dist'
$output = Join-Path $outputDir 'rdp-reverse-agent-windows-x86_64.exe'

$cargoCommand = Get-Command cargo.exe -ErrorAction SilentlyContinue
$cargoPath = if ($cargoCommand) {
    $cargoCommand.Source
} else {
    Join-Path $env:USERPROFILE '.cargo\bin\cargo.exe'
}
if (-not (Test-Path -LiteralPath $cargoPath -PathType Leaf)) {
    throw 'cargo.exe를 찾을 수 없습니다.'
}

& $cargoPath build --manifest-path $manifest --release
if ($LASTEXITCODE -ne 0) { throw "Agent 빌드 실패: $LASTEXITCODE" }

New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
Copy-Item -LiteralPath $source -Destination $output -Force

Write-Host "Agent 생성 완료: $output"
Get-FileHash -Algorithm SHA256 -LiteralPath $output
