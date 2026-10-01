# ReverseRDP relay server

승인된 GNS3 격리 실습망에서 Reverse RDP 연결 구조를 검증하기 위한 Rust 기반 TCP 중계 프로젝트입니다.

## 구성

- `rdp-reverse-relay`: 운영자 측에서 RDP 클라이언트와 터널 연결을 중계합니다.
- `rdp-reverse-agent`: Windows 대상에서 Relay로 아웃바운드 연결을 생성하고 로컬 RDP 서비스(`127.0.0.1:3389`)와 연결합니다.

현재 실습 구성은 다음과 같습니다.

- Relay 터널 리스너: `172.16.121.4:14443`
- Relay 로컬 RDP 리스너: `127.0.0.1:13389`
- Agent 로컬 RDP 대상: `127.0.0.1:3389`

## 빌드

각 프로젝트 폴더에서 다음 명령을 실행합니다.

```powershell
cargo build --release
```

생성 파일:

- `rdp-reverse-relay/target/release/rdp-reverse-relay.exe`
- `rdp-reverse-agent/target/release/rdp-reverse-agent.exe`

## 실행 순서

1. Relay 호스트에서 `rdp-reverse-relay.exe`를 실행합니다.
2. Windows 게스트에서 `rdp-reverse-agent.exe`를 실행합니다.
3. Relay 호스트에서 `mstsc.exe /v:127.0.0.1:13389 /prompt`를 실행합니다.

## 주의

이 코드는 승인된 격리 실습 환경 전용입니다. 실제 대학망, 제3자 시스템 또는 승인되지 않은 대상에 사용하지 마십시오.
