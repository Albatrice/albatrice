use std::net::*;
use abes_nice_things::{input, Input};
mod formats;

static mut QUIET: bool = false;

#[macro_export]
macro_rules! quiet {
    () => {
        unsafe {
            if !crate::QUIET {
                println!()
            }
        }
    };
    ($($args:tt)*) => {
        unsafe {
            if !crate::QUIET  {
                println!($($args)*)
            }
        }
    }
}

fn main() {
    let settings = Settings::new();
    if let None = settings {
        return
    }
    let settings = settings.unwrap();
    match settings.host {
        Some(port) => host(port, settings),
        None => connect(settings),
    }
}
fn host(port: u16, settings: Settings) {
    quiet!("Listening...");
    for connection in TcpListener::bind(
        (Ipv4Addr::UNSPECIFIED, port)
    ).expect("Failed to bind to port").incoming() {
        quiet!("Incoming connection");
        match connection {
            Ok(stream) => {
                if let Err(error) = formats::hand_shake(stream, settings.clone()) {
                    eprintln!("{error}")
                }
            }
            Err(error) => eprintln!("Failed to connect: {error}")
        }
    }
}
fn connect(settings: Settings) {
    loop {
        match settings.clone().mode {
            Mode::Send => println!("What addr:port do you want to send a file to?"),
            Mode::Recv => println!("What addr:port do you want to recieve a file from?")
        }
        let addr = match settings.clone().target {
            Some(target) => target,
            None => input()
        };
        match TcpStream::connect(addr) {
            Ok(stream) => {
                if let Err(error) = formats::hand_shake(stream, settings.clone()) {
                    eprintln!("{error}")
                }
            }
            Err(error) => eprintln!("Failed to connect: {error}")
        }
    }
}
#[derive(Clone)]
struct Settings {
    mode: Mode,
    host: Option<u16>,
    overide: Option<formats::FormatID>,
    // Sender only
    path: Option<String>,
    // Client only
    target: Option<String>,
    // Reciever only
    auto_accept: bool,
}
impl Settings {
    const HELP: &str = include_str!("help.txt");
    fn new() -> Option<Settings> {
        let mut out = Settings::default();
        let mut mode = None;
        let mut args = std::env::args();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "help" => {
                    println!("{}", Settings::HELP);
                    return None
                }
                "--recv" => mode = Some(Mode::Recv),
                "--send" => mode = Some(Mode::Send),
                "--host" => {
                    out.host = Some(
                        args.next().expect("Need a port after --host")
                        .parse::<u16>().expect("Need a port after --host")
                    );
                },
                "--override" => {
                    out.overide = Some(
                        args.next().expect("Need a format id after --override")
                        .parse::<formats::FormatID>().expect("Need for a format id after --override")
                    );
                    assert!(
                        out.overide.unwrap() > formats::HIGHEST,
                        "Need a valid format id after --override"
                    )
                }
                "--no-override" => out.overide = None,
                "--path" => {
                    out.path = Some(
                        args.next().expect("Need a file path after --path")
                    )
                }
                "--no-path" => out.path = None,
                "--target" => {
                    out.target = Some(
                        args.next().expect("Need a addr:port after --target")
                    )
                }
                "--no-target" => out.target = None,
                "--quiet" => unsafe {
                    QUIET = true;
                }
                "--normal" => unsafe {
                    QUIET = false;
                }
                "--auto-accept" => out.auto_accept = true,
                "--no-auto-accept" => out.auto_accept = false,
                _ => {
                    println!("{}", Settings::HELP);
                    return None
                }
            }
        }
        match mode {
            Some(mode) => out.mode = mode,
            None => {
                match <Input>::new().cond(&|string| {
                    match string.as_str() {
                        "recv"|"r"|"send"|"s" => Ok(true),
                        _ => Ok(false)
                    }
                }).msg("").get().unwrap().as_str() {
                    "recv"|"r" => out.mode = Mode::Recv,
                    "send"|"s" => out.mode = Mode::Send,
                    _ => unreachable!()
                }
            }
        }
        return Some(out);
    }
    fn get_format(&self) -> formats::FormatID {
        match self.overide {
            Some(format) => format,
            None => formats::HIGHEST
        }
    }
}
impl Default for Settings {
    fn default() -> Self {
        Settings {
            mode: Mode::Recv,
            host: None,
            overide: None,
            path: None,
            target: None,
            auto_accept: false,
        }
    }
}
#[derive(Clone, Copy)]
enum Mode {
    Send,
    Recv,
}
