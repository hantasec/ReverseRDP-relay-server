# ReverseRDP relay server

인바운드가 차단된 네트워크에 RDP 로 접속하기 위한 Rust 기반의 TCP 중계 프로젝트입니다.

Rust 의 Tokio를 사용하여 호스트, 게스트와 각각 연결된 두 쌍의 비동기 TCP 세션을 생성한 후, 

두 소켓의 송수신 연결을 서로 포워딩 해줌으로써 TCP relay 채널을 생성합니다.

그 다음, RDP 로 접속하고자 하는 윈도우에서 agent 를 실행해 TCP 채널에서 전송되는 패킷을 RDP listen 포트로 전송합니다.

마지막으로 호스트에서 relay 서버에 RDP 연결 패킷을 보내면, 해당 패킷이 게스트 PC 의 RDP listen 포트까지 전달되어 RDP 연결이 수립됩니다. 


## 구성

- `rdp-reverse-relay`: 운영자 측에서 RDP 클라이언트와 터널 연결을 중계합니다.
- `rdp-reverse-agent`: Windows 대상에서 Relay로 아웃바운드 연결을 생성하고 로컬 RDP 서비스(`127.0.0.1:3389`)와 연결합니다.

## 빌드

각 프로젝트 폴더에서 cargo build 명령어로 실행 파일을 생성합니다.

```powershell
cargo build --release
```

생성 파일:

- `rdp-reverse-relay/target/release/rdp-reverse-relay.exe`
- `rdp-reverse-agent/target/release/rdp-reverse-agent.exe`

## 실행 순서

1. Relay 호스트에서 `rdp-reverse-relay.exe`를 실행합니다.
2. Windows 게스트에서 `rdp-reverse-agent.exe`를 실행합니다.
3. Relay 호스트에서 Relay 서버로 RDP 연결을 실시합니다.

## 주의

이 코드는 격리된 환경에서 진행되는 프로젝트를 위해 제작한 것입니다. 제3자 시스템 또는 승인되지 않은 대상에 사용하지 마십시오. 
개발자는 그로 인해 발생하는 어떠한 문제에 대한 책임도 지지 않으며 모든 책임은 사용자에게 있습니다.
