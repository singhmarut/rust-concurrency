
pub mod network{

    extern crate futures;
    extern crate tokio_core;
    extern crate tokio_io;
    extern crate tokio_service;
    extern crate tokio_proto;
    extern crate bytes;

    use futures::{future, Future, Stream};
    use tokio_io::{AsyncRead,AsyncWrite};
    use tokio_io::codec::{Encoder, Decoder};
    use tokio_io::codec::Framed;
    use tokio_core::net::TcpListener;
    //use tokio_service::Service;
    //use network::network::tokio_service;
    use tokio_core::reactor::Core;
    use self::tokio_proto::TcpServer;

    use std::io;
    use std::str;
    //use bytes::BytesMut;


    pub struct LineCodec;
    pub struct LineProto;
    pub struct ServerProto;

    impl<T: AsyncRead + AsyncWrite + 'static> self::tokio_proto::pipeline::ServerProto<T> for LineProto {
        // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
        type Request = String;

        // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
        type Response = String;

        // A bit of boilerplate to hook in the codec:
        type Transport = Framed<T, LineCodec>;
        type BindTransport = Result<Self::Transport, io::Error>;
        fn bind_transport(&self, io: T) -> Self::BindTransport {
            Ok(io.framed(LineCodec))
        }
    }

    impl Decoder for LineCodec {
        type Item = String;
        type Error = io::Error;

        fn decode(&mut self, buf: &mut bytes::BytesMut) -> io::Result<Option<String>> {
            if let Some(i) = buf.iter().position(|&b| b == b'\n') {
                // remove the serialized frame from the buffer.
                let line = buf.split_to(i);

                // Also remove the '\n'
                buf.split_to(1);

                // Turn this data into a UTF string and return it in a Frame.
                match str::from_utf8(&line) {
                    Ok(s) => Ok(Some(s.to_string())),
                    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                                 "invalid UTF-8")),
                }
            } else {
                Ok(None)
            }
        }
    }

    impl Encoder for LineCodec {
        type Item = String;
        type Error = io::Error;

        fn encode(&mut self, msg: String, buf: &mut bytes::BytesMut) -> io::Result<()> {
            buf.extend(msg.as_bytes());
            buf.extend(b"\n");
            Ok(())
        }
    }

    impl self::tokio_service::Service for Echo {
        // These types must match the corresponding protocol types:
        type Request = String;
        type Response = String;

        // For non-streaming protocols, service errors are always io::Error
        type Error = io::Error;

        // The future for computing the response; box it for simplicity.
        type Future = Box<Future<Item = Self::Response, Error =  Self::Error>>;

        // Produce a future for computing a response from a request.
        fn call(&self, req: Self::Request) -> Self::Future {
            // In this case, the response is immediate.
            Box::new(future::ok(req))
        }
    }

    pub struct Echo;

    pub fn launch_tcp_server(){
        let addr = "0.0.0.0:12345".parse().unwrap();

        // The builder requires a protocol and an address
        let server = TcpServer::new(LineProto, addr);

        // We provide a way to *instantiate* the service for each new
        // connection; here, we just immediately return a new instance.
        server.serve(|| Ok(Echo));
    }
    pub fn send_data_to_network(core: &mut Core) {
        // Create the event loop that will drive this server
        //let mut core = Core::new().unwrap();
        let handle = core.handle();

        // Bind the server's socket
        let addr = "127.0.0.1:8080".parse().unwrap();
        let tcp = TcpListener::bind( &addr, & handle).unwrap();
        println!("Starting Server");

        // Iterate incoming connections
        let server = tcp.incoming().for_each( | (tcp, _) | {

            println!("Starting reading ");
            // Split up the read and write halves
            let (reader, writer) = tcp.split();

            // Future of the copy
            let bytes_copied = io::copy(reader, writer);

            // ... after which we'll print what happened
            let handle_conn = bytes_copied.map( | (n, _, _)| {
                println ! ("wrote {} bytes", n)
            }).map_err( | err | {
                println ! ("IO error {:?}", err)
            });

            // Spawn the future as a concurrent task
            handle.spawn(handle_conn);

            Ok(())
        });
        // Spin up the server on the event loop
        core.run(server).unwrap();
    }
}
