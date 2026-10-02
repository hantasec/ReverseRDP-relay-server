# ReverseRDP relay server

인바운드가 차단된 네트워크에 RDP 로 접속하기 위한 Rust 기반의 TCP 중계 프로젝트입니다.

Rust 의 Tokio를 사용하여 호스트, 게스트와 각각 연결된 두 쌍의 비동기 TCP 세션을 생성한 후, 

두 소켓의 송수신 연결을 서로 포워딩 해줌으로써 TCP relay 채널을 생성합니다.

그 다음, RDP 로 접속하고자 하는 윈도우에서 agent 를 실행해 TCP 채널에서 전송되는 패킷을 RDP listen 포트로 전송합니다.

마지막으로 호스트에서 relay 서버에 RDP 연결 패킷을 보내면, 해당 패킷이 게스트 PC 의 RDP listen 포트까지 전달되어 RDP 연결이 수립됩니다. 


## 구성

- `rdp-reverse-relay`: 운영자 측에서 RDP 클라이언트와 터널 연결을 중계합니다.
- `rdp-reverse-agent`: Windows 대상에서 Relay로 아웃바운드 연결을 생성하고 로컬 RDP 서비스(`127.0.0.1:3389`)와 연결합니다.

현재 실습 구성은 다음과 같습니다.

- T-Kali Relay: `203.230.56.162`
- 관리자 Windows Agent: `203.230.41.34/27`
- Relay 터널 리스너: `203.230.56.162:14443`
- Relay 로컬 RDP 리스너: `127.0.0.1:13389`
- Agent 로컬 RDP 대상: `127.0.0.1:3389`
- Cacti와 관리 DB 주소는 이 프로그램이 직접 사용하지 않습니다.

## 빌드

### Kali Relay

```bash
chmod +x scripts/build-kali-relay.sh scripts/run-kali-relay.sh
./scripts/build-kali-relay.sh
```

생성 파일: `dist/rdp-reverse-relay-linux-x86_64`

### Windows Agent

```powershell
& .\scripts\build-windows-agent.ps1
```

생성 파일: `dist/rdp-reverse-agent-windows-x86_64.exe`

Rust가 없는 배포 대상에는 GitHub Actions의 `Build binaries` 실행 결과에서 다음 완성 파일을 내려받아 전달할 수 있습니다.

- `rdp-reverse-relay-linux-x86_64`
- `rdp-reverse-agent-windows-x86_64.exe`

## 실행 순서

1. T-Kali에서 `./scripts/run-kali-relay.sh`를 실행합니다.
2. 관리자 Windows에서 `203.230.56.162:14443` 도달성과 `127.0.0.1:3389` 로컬 RDP를 확인합니다.
3. 관리자 Windows에서 `dist\rdp-reverse-agent-windows-x86_64.exe`를 실행합니다.
4. T-Kali에서 `127.0.0.1:13389`로 FreeRDP를 연결합니다. Windows의 실제 컴퓨터 이름과 승인된 시험 계정을 사용합니다.

기본 주소는 환경변수로 덮어쓸 수 있습니다.

- Relay: `RDP_LISTEN`, `TUNNEL_LISTEN`
- Agent: `RELAY_ADDRESS`, `LOCAL_RDP_ADDRESS`

## 주의

이 코드는 격리된 환경에서 진행되는 프로젝트를 위해 제작한 것입니다. 제3자 시스템 또는 승인되지 않은 대상에 사용하지 마십시오. 
개발자는 그로 인해 발생하는 어떠한 문제에 대한 책임도 지지 않으며 모든 책임은 사용자에게 있습니다.
