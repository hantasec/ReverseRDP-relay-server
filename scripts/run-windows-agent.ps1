$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = Split-Path -Parent $PSScriptRoot
$agent = Join-Path $repoRoot 'dist\rdp-reverse-agent-windows-x86_64.exe'

if (-not (Test-Path -LiteralPath $agent -PathType Leaf)) {
    throw 'Agent 실행 파일이 없습니다. 먼저 scripts\build-windows-agent.ps1을 실행하십시오.'
}

$localRdp = Test-NetConnection -ComputerName 127.0.0.1 -Port 3389 -WarningAction SilentlyContinue
if (-not $localRdp.TcpTestSucceeded) {
    throw '로컬 RDP 127.0.0.1:3389에 연결할 수 없습니다.'
}

$relay = Test-NetConnection -ComputerName 203.230.56.162 -Port 14443 -WarningAction SilentlyContinue
if (-not $relay.TcpTestSucceeded) {
    throw 'Relay 203.230.56.162:14443에 연결할 수 없습니다.'
}

$env:RELAY_ADDRESS = '203.230.56.162:14443'
$env:LOCAL_RDP_ADDRESS = '127.0.0.1:3389'
& $agent
