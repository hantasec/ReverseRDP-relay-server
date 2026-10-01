use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;

async fn bind_listener(address: &str, listener_name: &str) -> std::io::Result<TcpListener>{
    let bind_result = TcpListener::bind(address).await;

    match bind_result {
        Ok(listener) => {
            println!("{listener_name} listening on {address}");
            Ok(listener)
        }
        Err(error) => {
            println!("{listener_name} failed to open port : {error}");
            Err(error)
        }
    }
}

async fn session_accept(listener: &TcpListener, session_name: &str) -> std::io::Result<(TcpStream, SocketAddr)> {
    println!("waiting sessions to accept ... ");
    let connection = listener.accept().await;
    let (socket, peer_address) = match connection {
        Ok(connection) => {
            connection
        }
        Err(error) => {
            println!("{session_name} failed to connect ... {error}");
            return Err(error);
        }
    };
    println!("{session_name} connected : {peer_address}");
    Ok((socket, peer_address))
}

async fn forward_data( mut reader: OwnedReadHalf, mut writer: OwnedWriteHalf, direction: &str,) -> std::io::Result<u64> {
    let mut buffer = [0u8; 4096];
    let mut total_bytes = 0u64;
    loop {
        let read_result = reader.read(&mut buffer).await;
        let bytes_read = match read_result {
            Ok(bytes_read) => bytes_read,
            Err(error) => {
                println!("[{direction}] 읽기 실패: {error}");
                return Err(error);
            }
        };
        if bytes_read == 0 {
            println!("[{direction}] 연결 종료");
            break;
        }
        let write_result =
            writer.write_all(&buffer[..bytes_read]).await;
        match write_result {
            Ok(()) => {
                total_bytes += bytes_read as u64;
                println!("[{direction}] {bytes_read}바이트 전달");
            }
            Err(error) => {
                println!("[{direction}] 쓰기 실패: {error}");
                return Err(error);
            }
        }
    }
    Ok(total_bytes)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // RDP 클라이언트용 리스너, windows 에이전트 터널용 리스너 생성
    let rdp_listener = bind_listener("127.0.0.1:13389", "RDP").await?;
    let tunnel_listener = bind_listener("172.16.121.4:14443", "Tunnel").await?;
    // 터널, RDP 클라이언트 연결 소켓 획득
    let (tunnel_socket, _) = session_accept(&tunnel_listener, "Tunnel").await?;
    let (rdp_socket, _) = session_accept(&rdp_listener, "RDP").await?;
    // 소켓(TCPStream)으로부터 소유권을 가진 reader, writer 획득
    let (rdp_reader, rdp_writer) = rdp_socket.into_split();
    let (tunnel_reader, tunnel_writer) = tunnel_socket.into_split();
    // reader 와 writer 를 교차 연결해서 전달 구조 생성
    let rdp_to_tunnel = forward_data(rdp_reader, tunnel_writer, "RDP -> Tunnel",);
    let tunnel_to_rdp = forward_data(tunnel_reader, rdp_writer, "Tunnel -> RDP",);
    // join 으로 병행실행, 둘의 종료를 기다린다.
    let (result_one, result_two) = tokio::join!(rdp_to_tunnel, tunnel_to_rdp,);
    // 둘 다 제대로 결과를 반환하였는지 확인.
    let rdp_to_tunnel_bytes = result_one?;
    let tunnel_to_rdp_bytes = result_two?;

    println!("중계 세션이 종료되었습니다.");
    println!("RDP -> Tunnel: {rdp_to_tunnel_bytes}바이트");
    println!("Tunnel -> RDP: {tunnel_to_rdp_bytes}바이트");

    Ok(())
}
