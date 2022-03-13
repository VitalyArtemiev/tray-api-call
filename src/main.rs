use core::mem::MaybeUninit;
use trayicon::*;
use winapi::um::winuser;
use crate::State::{Connected, Disconnected};



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Events {
    ClickTrayIcon,
    DoubleClickTrayIcon,
    CheckStatus,
    ToggleStatus,
    Exit,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Unknown,
    Connected,
    Disconnected,
}

struct Connection {
    state: State,
    msg: String
}

type ConnResult = Result<State, Error>;

impl Connection {
    fn new(address: String) -> Connection {

    }

    fn check(self) -> ConnResult {
        Ok(self.state)
    }

    fn connect(self) -> ConnResult {
        Ok(self.state)
    }

    fn disconnect(self) -> ConnResult {
        Ok(self.state)
    }

    fn toggle(mut self) {
        match self.state {
            State::Unknown => {
                self.check().map_err(|e|{}).map(|s| {self.toggle()});
            }
            State::Connected => {
                self.disconnect().map(|s| {self.state = s}).map_err(|e| {println!("WTF")});
            }
            State::Disconnected => {
                self.connect().map(|s| {self.state = s});
            }
        }
    }
}


fn main() {
    let (s, r) = std::sync::mpsc::channel::<Events>();
    let icon_off_buf = include_bytes!("../res/wireguard_off.ico");
    let icon_on_buf = include_bytes!("../res/wireguard_on.ico");

    let icon_off = Icon::from_buffer(icon_off_buf, None, None).unwrap();
    let icon_on = Icon::from_buffer(icon_on_buf, None, None).unwrap();

    let conn = Connection::new();

    // Needlessly complicated tray icon with all the whistles and bells
    let mut tray_icon = TrayIconBuilder::new()
        .sender(s)
        .icon_from_buffer(icon_off_buf)
        .tooltip("Cool Tray 👀 Icon")
        .on_click(Events::ClickTrayIcon)
        .on_double_click(Events::DoubleClickTrayIcon)
        .menu(
            MenuBuilder::new()
                .item("Reload status", Events::CheckStatus)
                .checkable("Enable tunnel", false, Events::ToggleStatus)
                .separator()
                .item("Exit", Events::Exit),
        )
        .build()
        .unwrap();

    std::thread::spawn(move || {
        r.iter().for_each(|m| match m {
            Events::DoubleClickTrayIcon => {
                println!("Double click");
                tray_icon.set_icon(&icon_on).unwrap();
            }
            Events::ClickTrayIcon => {
                println!("Single click");
            }
            Events::Exit => {
                println!("Please exit");
            }
            e => {
                println!("{:?}", e);
            }
        })
    });

    // Your applications message loop. Because all applications require an
    // application loop, you are best served using an `winit` crate.
    loop {
        unsafe {
            let mut msg = MaybeUninit::uninit();
            let bret = winuser::GetMessageA(msg.as_mut_ptr(), 0 as _, 0, 0);
            if bret > 0 {
                winuser::TranslateMessage(msg.as_ptr());
                winuser::DispatchMessageA(msg.as_ptr());
            } else {
                break;
            }
        }
    }
}
