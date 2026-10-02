use std::env;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;

const DEFAULT_RELAY_ADDRESS: &str = "203.230.56.162:14443";
const DEFAULT_LOCAL_RDP_ADDRESS: &str = "127.0.0.1:3389";

async fn connect_socket(address: &str, connection_name: &str) -> std::io::Result<TcpStream> {
    println!("{connection_name} 연결 시도 : {address}");

    let connection = TcpStream::connect(address).await;

    match connection {
        Ok(socket) => {
            println!("{connection_name} 연결 성공 : {address}");
            Ok(socket)
        }
        Err(error) => {
            println!("{connection_name} 연결 실패 : {error}");
            Err(error)
        }
    }
}

async fn forward_data(
    mut reader: OwnedReadHalf,
    mut writer: OwnedWriteHalf,
    direction: &str,
) -> std::io::Result<u64> {
    let mut buffer = [0u8; 4096];
    let mut total_bytes = 0u64;

    loop {
        let read_result = reader.read(&mut buffer).await;

        let bytes_read = match read_result {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                println!("[{direction}] 읽기 실패 : {error}");
                return Err(error);
            }
        };

        if bytes_read == 0 {
            println!("[{direction}] 연결 종료");
            break;
        }

        let write_result = writer.write_all(&buffer[..bytes_read]).await;

        match write_result {
            Ok(()) => {
                total_bytes += bytes_read as u64;
                println!("[{direction}] {bytes_read}바이트 전달");
            }
            Err(error) => {
                println!("[{direction}] 쓰기 실패 : {error}");
                return Err(error);
            }
        }
    }

    Ok(total_bytes)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let relay_address =
        env::var("RELAY_ADDRESS").unwrap_or_else(|_| DEFAULT_RELAY_ADDRESS.to_string());
    let local_rdp_address =
        env::var("LOCAL_RDP_ADDRESS").unwrap_or_else(|_| DEFAULT_LOCAL_RDP_ADDRESS.to_string());

    // 중계 서버의 터널 포트에 연결한다.
    let relay_socket = connect_socket(&relay_address, "Relay").await?;

    // Windows에서 실행 중인 로컬 RDP 서비스에 연결한다.
    let local_rdp_socket = connect_socket(&local_rdp_address, "Local RDP").await?;

    // 각 TCP 소켓을 읽기와 쓰기 객체로 분리한다.
    let (relay_reader, relay_writer) = relay_socket.into_split();
    let (rdp_reader, rdp_writer) = local_rdp_socket.into_split();

    // reader와 writer를 교차 연결해서 양방향 전달 구조를 생성한다.
    let relay_to_rdp = forward_data(relay_reader, rdp_writer, "Relay -> Local RDP");
    let rdp_to_relay = forward_data(rdp_reader, relay_writer, "Local RDP -> Relay");

    // 두 방향을 병행 실행하고 둘의 종료를 기다린다.
    let (relay_to_rdp_result, rdp_to_relay_result) = tokio::join!(relay_to_rdp, rdp_to_relay);

    let relay_to_rdp_bytes = relay_to_rdp_result?;
    let rdp_to_relay_bytes = rdp_to_relay_result?;

    println!("Agent 중계 세션 종료");
    println!("Relay -> Local RDP : {relay_to_rdp_bytes}바이트");
    println!("Local RDP -> Relay : {rdp_to_relay_bytes}바이트");

    Ok(())
}
