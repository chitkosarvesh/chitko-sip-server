mod sip_parser;

use crate::sip_parser::SipMessage;
use tokio::io::AsyncReadExt;

/// entrypoint for the sip server
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let arguments = ChitkoSipServerArgs::parse();
    configure_logging(arguments.verbose, arguments.log_file_path);
    log::info!(
        "Starting {} Version: {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    start_tcp_server(arguments.host, arguments.port).await?;
    Ok(())
}

/// configures logging
fn configure_logging(verbose: bool, log_path: Option<String>) -> log4rs::Handle {
    let stdout = log4rs::append::console::ConsoleAppender::builder().build();
    let file = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{d} - {m}{n}",
        )))
        .build(if log_path.is_some() {
            log_path.unwrap()
        } else {
            "log/server.log".to_string()
        })
        .unwrap();
    let config = log4rs::Config::builder()
        .appender(log4rs::config::Appender::builder().build("stdout", Box::new(stdout)))
        .appender(log4rs::config::Appender::builder().build("file", Box::new(file)))
        .build(
            log4rs::config::Root::builder()
                .appender("stdout")
                .appender("file")
                .build(if verbose {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Warn
                }),
        )
        .unwrap();
    log4rs::init_config(config).unwrap()
}

async fn start_tcp_server(host: String, port: u16) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind((host, port)).await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        log::info!("Incoming connection from {}", addr);
        tokio::spawn(process_stream(stream));
    }
}

async fn process_stream(stream: tokio::net::TcpStream) {
    let mut buf_reader = tokio::io::BufReader::new(stream);
    let mut buf = vec![0; 65535];
    tokio::spawn(async move {
        match buf_reader.read(&mut buf).await {
            Ok(n) if n == 0 => {
                log::info!("Connection closed by client");
            }
            Ok(n) => {
                log::debug!("Read {} bytes from client", n);
                log::debug!("{}", str::from_utf8(&buf[..n]).unwrap());
                SipMessage::new(str::from_utf8(&buf[..n]).unwrap());
            }
            Err(e) => log::error!("Error reading from client: {}", e),
        }
    });
}

/// struct to hold all command line arguments
struct ChitkoSipServerArgs {
    verbose: bool,
    host: String,
    port: u16,
    log_file_path: Option<String>,
}

/// implements basic functions for the arguments struct like parsing and converting to string
impl ChitkoSipServerArgs {
    fn parse() -> ChitkoSipServerArgs {
        let mut args = ChitkoSipServerArgs {
            verbose: false,
            host: "0.0.0.0".to_string(),
            port: 5060,
            log_file_path: None,
        };
        {
            let mut argument_parser = argparse::ArgumentParser::new();
            argument_parser.set_description("Chitko SIP Server");
            argument_parser.refer(&mut args.verbose).add_option(
                &["-v", "--verbose"],
                argparse::StoreTrue,
                "sets verbosity of the log output (default: false)",
            );
            argument_parser.refer(&mut args.host).add_option(
                &["-H", "--host"],
                argparse::Store,
                "IP address to bind the server to (default: 0.0.0.0)",
            );
            argument_parser.refer(&mut args.port).add_option(
                &["-p", "--port"],
                argparse::Store,
                "port to bind the sip server to (default: 5060)",
            );
            argument_parser.refer(&mut args.log_file_path).add_option(
                &["-l", "--log-file-path"],
                argparse::StoreOption,
                "path of the file to write logs to (default: ./log/server.log)",
            );
            argument_parser.parse_args_or_exit();
        }
        args
    }
}
