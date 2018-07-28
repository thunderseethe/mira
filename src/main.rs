extern crate gl;
extern crate glutin;
extern crate libc;
extern crate errno;

use std::collections::HashMap;
use std::io::{Read, Write, ErrorKind};

use glutin::dpi::*;
use glutin::GlContext;

mod tty;
mod data;


struct SizeInfo {
    row: u16,
    col: u16,
    xpixel: u16,
    ypixel: u16,
}

impl SizeInfo {
    fn new() -> Self {
        SizeInfo {
            row: 24,
            col: 80,
            xpixel: 800,
            ypixel: 600,
        }
    }
}

impl tty::ToWinsize for SizeInfo {
    fn to_winsize(&self) -> libc::winsize {
        libc::winsize {
            ws_row: self.row as libc::c_ushort,
            ws_col: self.col as libc::c_ushort,
            ws_xpixel: self.xpixel as libc::c_ushort,
            ws_ypixel: self.ypixel as libc::c_ushort,
        }
    }
}


fn main() {
    let tty_size = SizeInfo::new();
    let pty = tty::new(None, &HashMap::new(), None, &tty_size, None);
    let mut file = pty.reader();
    println!("{:?}", file);
    let mut string = String::new();
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        file.write(b"test").expect("file written");
        match file.read_to_string(&mut string) {
            Ok(0) => break,
            Ok(read_count) => {
                print!("{:?}", string);
            }
            Err(err) => {
                println!("{:?}", err);
                match err.kind() {
                    ErrorKind::WouldBlock => { /*continue */ },
                    _ => {
                       break;
                    },
                }
            }
        }
    }
}

fn test_window() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Test Terminal")
        .with_dimensions(LogicalSize::new(800.0, 600.0));

    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }
    
    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => { running = false },
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => (),
                },
                _ => ()
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        gl_window.swap_buffers().unwrap();
    }
}
