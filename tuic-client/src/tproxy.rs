use crate::{config::Tproxy, connection::Connection as TuicConnection, Error};
use bytes::Bytes;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use std::{
    collections::HashMap,
    io::{Error as IoError, ErrorKind},
    net::{IpAddr, SocketAddr, TcpListener as StdTcpListener, UdpSocket as StdUdpSocket},
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};
use tokio::{
    io::{self, AsyncWriteExt},
    net::{TcpListener, UdpSocket},
};
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tuic::Address as TuicAddress;

static TPROXY_SERVER: OnceCell<TproxyServer> = OnceCell::new();

pub struct TproxyServer {
    tcp_listen: TcpListener,
    udp_socket: UdpSocket,
}

impl TproxyServer {
    pub fn set_config(cfg: Tproxy) -> Result<(), Error> {
        let domain = match cfg.server.ip() {
            IpAddr::V4(_) => Domain::IPV4,
            IpAddr::V6(_) => Domain::IPV6,
        };

        let tcp_socket = {
            let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;

            socket.set_reuse_address(true)?;
            socket.bind(&SockAddr::from(cfg.server))?;
            socket.listen(128)?;
            TcpListener::from_std(StdTcpListener::from(socket))?
        };

        let udp_socket = {
            let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
            socket.set_reuse_address(true)?;
            socket.bind(&SockAddr::from(cfg.server))?;
            UdpSocket::from_std(StdUdpSocket::from(socket))?
        };
        
        let server = Self {
            tcp_listen: tcp_socket,
            udp_socket: udp_socket,
        };

        TPROXY_SERVER
            .set(server)
            .map_err(|_| "socks5 server already initialized")
            .unwrap();
        
        Ok(())
    }

    pub async fn start(){
        let server = TPROXY_SERVER.get().unwrap();
        
    }
}
