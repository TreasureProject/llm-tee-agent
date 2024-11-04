use nix::sys::socket::{self, AddressFamily, Backlog, SockFlag, SockType, VsockAddr};
use std::io;
use std::os::fd::{AsRawFd, IntoRawFd};
use std::os::unix::io::{FromRawFd, RawFd};
use std::os::unix::net::UnixStream;

pub struct VsockStream {
    stream: UnixStream,
}

impl VsockStream {
    fn new(stream: UnixStream) -> Self {
        VsockStream { stream }
    }
}

impl io::Read for VsockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl io::Write for VsockStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

pub struct VsockListener {
    fd: RawFd,
}

impl VsockListener {
    pub fn bind(port: u32) -> io::Result<Self> {
        let fd = socket::socket(
            AddressFamily::Vsock,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let addr = VsockAddr::new(-1i32 as u32, port);

        socket::bind(fd.as_raw_fd(), &addr).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let backlog = Backlog::new(128).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        socket::listen(&fd, backlog).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(VsockListener {
            fd: fd.into_raw_fd(),
        })
    }

    pub fn accept(&self) -> io::Result<VsockStream> {
        let client_fd =
            socket::accept(self.fd).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(VsockStream::new(unsafe {
            UnixStream::from_raw_fd(client_fd)
        }))
    }
}

pub struct VsockClient;

impl VsockClient {
    pub fn connect(cid: u32, port: u32) -> io::Result<VsockStream> {
        let fd = socket::socket(
            AddressFamily::Vsock,
            SockType::Stream,
            SockFlag::empty(),
            None,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let addr = VsockAddr::new(cid, port);

        socket::connect(fd.as_raw_fd(), &addr)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(VsockStream::new(unsafe {
            UnixStream::from_raw_fd(fd.into_raw_fd())
        }))
    }
}
