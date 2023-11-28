use std::{cell::RefCell, fmt::format};

pub struct KeyboardEventHandler {
    input_buffer: RefCell<Vec<KeyCode>>,
}

impl KeyboardEventHandler {
    pub fn new() -> KeyboardEventHandler {
        let keyboard_event_handler = KeyboardEventHandler {
            input_buffer: RefCell::new(Vec::new()),
        };
        keyboard_event_handler
    }
    fn input_buffer(&self) -> &mut Vec<KeyCode> {
        self.input_buffer.get_mut()
    }
    pub fn on_press(&self, event: xcb::x::KeyPressEvent) {
        self.input_buffer
            .borrow_mut()
            .push(KeyCode::from(event.detail()))
    }

    pub fn on_relese(&self, event: xcb::x::KeyReleaseEvent) {
        if self.input_buffer().contains(&KeyCode::from(event.detail())) {
            let index = self
                .input_buffer()
                .iter()
                .position(|key| *key == KeyCode::from(event.detail()))
                .unwrap();
            self.input_buffer().remove(index);
        }
    }
}

impl From<u8> for KeyCode {
    fn from(value: u8) -> Self {
        if let Ok(letter) = Letter::try_from(value) {
            return Self::Letter(letter);
        }
        if let Ok(numpad_key) = Numpad::try_from(value) {
            return Self::Numpad(numpad_key);
        }
        if let Ok(modkey) = ModKey::try_from(value) {
            return Self::Modkey(modkey);
        }
        if let Ok(functional_key) = Functional::try_from(value) {
            return Self::Functional(functional_key);
        }

        match value {
            9 => Self::Escape,
            10..=18 => Self::Num(1 + value - 10),
            19 => Self::Num(0),
            22 => Self::BackSpace,
            23 => Self::Tab,
            36 => Self::Return,
            65 => Self::Space,
            66 => Self::CapsLock,
            67..=76 => Self::F(1 + value - 67),
            77 => Self::NumLock,
            95..=96 => Self::F(11 + value - 95),

            107 | 218 => Self::Print,
            110 => Self::Home,
            111 => Self::Up,
            112 => Self::PageUp,
            113 => Self::Left,
            114 => Self::Right,
            115 => Self::End,
            116 => Self::Down,
            117 => Self::PageUp,
            118 => Self::Insert,
            119 => Self::Delete,
            127 => Self::Pause,

            135 => Self::Menu,
            136 => Self::Cancel,
            137 | 190 => Self::Redo,
            138 => Self::SunProps,
            139 => Self::Undo,
            140 => Self::SunFront,
            144 => Self::Find,
            146 => Self::Help,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum KeyCode {
    BackSpace,
    Cancel,
    CapsLock,
    Delete,
    Down,
    End,
    Escape,
    F(u8),
    Find,
    Functional(Functional),
    Help,
    Home,
    Insert,
    Left,
    Letter(Letter),
    Menu,
    Modkey(ModKey),
    Num(u8),
    NumLock,
    Numpad(Numpad),
    PageDown,
    PageUp,
    Pause,
    Print,
    Redo,
    Return,
    Right,
    Space,
    SunFront,
    SunProps,
    Tab,
    Undo,
    Up,
}

#[derive(PartialEq, Eq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl ToString for Letter {
    fn to_string(&self) -> String {
        match self {
            Letter::A => "a",
            Letter::B => "b",
            Letter::C => "c",
            Letter::D => "d",
            Letter::E => "e",
            Letter::F => "f",
            Letter::G => "g",
            Letter::H => "h",
            Letter::I => "i",
            Letter::J => "j",
            Letter::K => "k",
            Letter::L => "l",
            Letter::M => "m",
            Letter::N => "n",
            Letter::O => "o",
            Letter::P => "p",
            Letter::Q => "q",
            Letter::R => "r",
            Letter::S => "s",
            Letter::T => "t",
            Letter::U => "u",
            Letter::V => "v",
            Letter::W => "w",
            Letter::X => "x",
            Letter::Y => "y",
            Letter::Z => "z",
        }
        .to_string()
    }
}

impl TryFrom<u8> for Letter {
    fn try_from(value: u8) -> Result<Letter, Self::Error> {
        Ok(match value {
            24 => Self::Q,
            25 => Self::W,
            26 => Self::E,
            27 => Self::R,
            28 => Self::T,
            29 => Self::Y,
            30 => Self::U,
            31 => Self::I,
            32 => Self::O,
            33 => Self::P,
            52 => Self::Z,
            53 => Self::X,
            54 => Self::C,
            55 => Self::V,
            56 => Self::B,
            57 => Self::N,
            58 => Self::M,
            38 => Self::A,
            39 => Self::S,
            40 => Self::D,
            41 => Self::F,
            42 => Self::G,
            43 => Self::H,
            44 => Self::J,
            45 => Self::K,
            46 => Self::L,
            _ => return Err(()),
        })
    }

    type Error = ();
}

#[derive(PartialEq, Eq)]
pub enum ModKey {
    Shift(KeyPosition),
    Control(KeyPosition),
    Alt(KeyPosition),
    Super(KeyPosition),
    Meta(KeyPosition),
    Hyper,
}

impl TryFrom<u8> for ModKey {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            50 => Self::Shift(KeyPosition::Left),
            62 => Self::Shift(KeyPosition::Right),
            37 => Self::Control(KeyPosition::Left),
            105 => Self::Control(KeyPosition::Right),
            207 => Self::Hyper,
            64 | 205 => Self::Meta(KeyPosition::Left),
            108 => Self::Meta(KeyPosition::Right),
            133 | 206 => Self::Super(KeyPosition::Left),
            134 => Self::Super(KeyPosition::Right),
            64 | 204 => Self::Alt(KeyPosition::Left),
            108 => Self::Alt(KeyPosition::Right),
            _ => return Err(()),
        })
    }
}

impl ToString for ModKey {
    fn to_string(&self) -> String {
        match self {
            ModKey::Shift(side) => format!("{}_Sh", side.to_string()),
            ModKey::Control(side) => format!("{}_C", side.to_string()),
            ModKey::Alt(side) => format!("{}_A", side.to_string()),
            ModKey::Super(side) => format!("{}_Sup", side.to_string()),
            ModKey::Meta(side) => format!("{}_M", side.to_string()),
            ModKey::Hyper => "H".to_string(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum KeyPosition {
    Left,
    Right,
}

impl ToString for KeyPosition {
    fn to_string(&self) -> String {
        match self {
            KeyPosition::Left => "L",
            KeyPosition::Right => "R",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq)]
pub enum Symbols {
    Apostrophe,
    BackSlash,

    BracketLeft,
    BracketRight,
    Colon,
    Grave,
    Greater,
    Less,
    LessGreaterBar,
    Linefeed,
    Minus,
    ParenLeft,
    ParenRight,
    PlusAndEqual,
    PlusMinus,
    Slash,
}

#[derive(PartialEq, Eq)]
pub enum Numpad {
    Num(u8),
    Devide,
    Substract,
    Add,
    Multiply,
    NumLock,
    Enter,
    Equal,
    Decimal,
    Delete,
}

impl TryFrom<u8> for Symbols {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            47 => Self::Colon,
            48 => Self::Apostrophe,
            49 => Self::Grave,
            51 => Self::BackSlash,
            59 => Self::Less,
            60 => Self::Greater,
            61 => Self::Slash,
            109 => Self::Linefeed,
            34 => Self::BracketLeft,
            35 => Self::BracketRight,
            187 => Self::ParenLeft,
            188 => Self::ParenRight,
            20 => Self::Minus,
            94 => Self::LessGreaterBar,
            21 => Self::PlusAndEqual,
            126 => Self::PlusMinus,
            _ => return Err(()),
        })
    }
    type Error = ();
}

impl TryFrom<u8> for Numpad {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            79..=81 => Numpad::Num(7 + value - 79),
            83..=85 => Numpad::Num(4 + value - 83),
            87..=89 => Numpad::Num(1 + value - 87),
            90 => Numpad::Num(0),
            63 => Numpad::Multiply,
            82 => Numpad::Substract,
            86 => Numpad::Add,
            91 => Numpad::Delete,
            104 => Numpad::Enter,
            106 => Numpad::Devide,
            125 => Numpad::Equal,
            129 => Numpad::Decimal,
            _ => return Err(()),
        })
    }
    type Error = ();
}

impl ToString for Numpad {
    fn to_string(&self) -> String {
        format!(
            "N_{}",
            match self {
                Numpad::Num(num) => num.to_string().as_str(),
                Numpad::Devide => "/",
                Numpad::Substract => "-",
                Numpad::Add => "+",
                Numpad::Multiply => "*",
                Numpad::NumLock => "Lock",
                Numpad::Enter => "Enter",
                Numpad::Equal => "=",
                Numpad::Decimal => ".",
                Numpad::Delete => "delete",
            },
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum Functional {
    Audio(Audio),
    Back,
    Battery,
    Bluetooth,
    BrightnessAuto,
    Calculator,
    Close,
    Copy,
    Cut,
    DOS,
    Display,
    DisplayOff,
    Documents,
    Eject,
    Explorer,
    Farward,
    Favourite,
    Finance,
    Game,
    Go,
    HomePage,
    KBD(KBD),
    Kill,
    Launch(u8),
    LaunchA,
    LaunchB,
    Mail,
    MailFarward,
    Menu,
    Messanger,
    Monitor(Monitor),
    MyComputer,
    New,
    NextVMode,
    Open,
    Paste,
    Phone,
    PowerOff,
    PrevVMode,
    Reload,
    Reply,
    RotateWindows,
    Save,
    ScreenSaver,
    ScrollDown,
    ScrollLock,
    ScrollUp,
    Search,
    Send,
    Shop,
    Sleep,
    Suspend,
    TaskPlane,
    Tools,
    TouchPad(TouchPad),
    UWB,
    WLAN,
    WWAN,
    WWW,
    WakeUp,
    WebCamera,
    Xref,
}

impl TryFrom<u8> for Functional {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Ok(audio_key) = Audio::try_from(value) {
            return Ok(Self::Audio(audio_key));
        }

        if let Ok(kbd_key) = KBD::try_from(value) {
            return Ok(Self::KBD(kbd_key));
        }

        if let Ok(monitor_key) = Monitor::try_from(value) {
            return Ok(Self::Monitor(monitor_key));
        }

        if let Ok(touchpad_key) = TouchPad::try_from(value) {
            return Ok(Self::TouchPad(touchpad_key));
        }

        Ok(match value {
            255 => Self::Kill,
            254 => Self::WWAN,
            253 => Self::DisplayOff,
            250 => Self::PrevVMode,
            249 => Self::NextVMode,
            247 => Self::UWB,
            246 => Self::WLAN,
            245 => Self::Bluetooth,
            244 => Self::Battery,
            243 => Self::Documents,
            242 => Self::Save,
            241 => Self::MailFarward,
            240 => Self::Reply,
            153 | 239 => Self::Send,
            235 => Self::Display,
            229 => Self::Shop,
            228 => Self::Game,
            227 => Self::Finance,
            226 => Self::Go,
            225 => Self::Search,
            224 => Self::Messanger,
            163 | 223 => Self::Mail,
            220 => Self::WebCamera,
            214 => Self::Close,
            213 => Self::Suspend,
            212 => Self::LaunchB,
            210..=211 => Self::Launch(3 + value - 210),
            192..=196 => Self::Launch(5 + value - 192),
            179 | 191 => Self::Tools,
            189 => Self::New,
            186 => Self::ScrollDown,
            185 => Self::ScrollUp,
            182 => Self::Close,
            181 => Self::Reload,
            180 => Self::HomePage,
            177 => Self::Phone,
            169 | 170 => Self::Eject,
            167 => Self::Farward,
            166 => Self::Back,
            165 => Self::MyComputer,
            164 => Self::Favourite,
            162 => Self::TaskPlane,
            161 => Self::RotateWindows,
            160 => Self::ScreenSaver,
            159 => Self::DOS,
            158 => Self::WWW,
            156..=157 => Self::Launch(1 + value - 156),
            155 => Self::Xref,
            153 => Self::Send,
            152 => Self::Explorer,
            151 => Self::WakeUp,
            150 => Self::Sleep,
            148 => Self::Calculator,
            147 => Self::Menu,
            145 => Self::Cut,
            143 => Self::Paste,
            142 => Self::Open,
            141 => Self::Copy,
            128 => Self::LaunchA,
            124 => Self::PowerOff,
            78 => Self::ScrollLock,

            _ => return Err(()),
        })
    }
}

impl ToString for Functional {
    fn to_string(&self) -> String {
        format!(
            "Fn_{}",
            match self {
                Functional::Audio(audio) => audio.to_string().as_str(),
                Functional::Back => "Back",
                Functional::Battery => "Battery",
                Functional::Bluetooth => "Bluetooth",
                Functional::BrightnessAuto => "BrightnessAuto",
                Functional::Calculator => "Calculator",
                Functional::Close => "Close",
                Functional::Copy => "Copy",
                Functional::Cut => "Cut",
                Functional::DOS => "DOS",
                Functional::Display => "Display",
                Functional::DisplayOff => "Display_Off",
                Functional::Documents => "Documents",
                Functional::Eject => "Eject",
                Functional::Explorer => "Explorer",
                Functional::Farward => "Farward",
                Functional::Favourite => "Favourite",
                Functional::Finance => "Finance",
                Functional::Game => "Game",
                Functional::Go => "Go",
                Functional::HomePage => "Home_Page",
                Functional::KBD(kbd) => kbd.to_string().as_str(),
                Functional::Kill => "Kill",
                Functional::Launch(_) => "Launch",
                Functional::LaunchA => "Launch_A",
                Functional::LaunchB => "Launch_B",
                Functional::Mail => "Mail",
                Functional::MailFarward => "Mail_Farward",
                Functional::Menu => "Menu",
                Functional::Messanger => "Messanger",
                Functional::Monitor(monitor) => monitor.to_string().as_str(),
                Functional::MyComputer => "My_Computer",
                Functional::New => "New",
                Functional::NextVMode => "Next_V_Mode",
                Functional::Open => "Open",
                Functional::Paste => "Paste",
                Functional::Phone => "Phone",
                Functional::PowerOff => "Power_Off",
                Functional::PrevVMode => "Prev_V_Mode",
                Functional::Reload => "Reload",
                Functional::Reply => "Reply",
                Functional::RotateWindows => "Rotate_Windows",
                Functional::Save => "Save",
                Functional::ScreenSaver => "Screen_Saver",
                Functional::ScrollDown => "Scroll_Down",
                Functional::ScrollLock => "Scroll_Lock",
                Functional::ScrollUp => "Scroll_Up",
                Functional::Search => "Search",
                Functional::Send => "Send",
                Functional::Shop => "Shop",
                Functional::Sleep => "Sleep",
                Functional::Suspend => "Suspend",
                Functional::TaskPlane => "Task_Plane",
                Functional::Tools => "Tools",
                Functional::TouchPad(touchpad) => touchpad.to_string().as_str(),
                Functional::UWB => "UWB",
                Functional::WLAN => "WLAN",
                Functional::WWAN => "WWAN",
                Functional::WWW => "WW",
                Functional::WakeUp => "Wake_Up",
                Functional::WebCamera => "Web_Camera",
                Functional::Xref => "Xref",
            }
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum KBD {
    ToggleLight,
    BrightnessUp,
    BrightnessDown,
}

impl TryFrom<u8> for KBD {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            236 => Self::ToggleLight,
            237 => Self::BrightnessDown,
            238 => Self::BrightnessUp,
            _ => return Err(()),
        })
    }
}

#[derive(PartialEq, Eq)]
pub enum Monitor {
    RiseBrightness,
    LowerBrightness,
    BrightnessCycle,
}

impl ToString for KBD {
    fn to_string(&self) -> String {
        format!(
            "KDB_{}",
            match self {
                KBD::ToggleLight => "ToggleLight",
                KBD::BrightnessUp => "Brightness+",
                KBD::BrightnessDown => "Brightness-",
            }
        )
    }
}

impl TryFrom<u8> for Monitor {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            232 => Self::LowerBrightness,
            233 => Self::RiseBrightness,
            251 => Self::BrightnessCycle,
            _ => return Err(()),
        })
    }
}

impl ToString for Monitor {
    fn to_string(&self) -> String {
        format!(
            "Mon_{}",
            match self {
                Monitor::RiseBrightness => "Brightness+",
                Monitor::LowerBrightness => "Brightness-",
                Monitor::BrightnessCycle => "Brightness_Cycle",
            }
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum TouchPad {
    On,
    Off,
    Toggle,
}

impl TryFrom<u8> for TouchPad {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            199 => Self::Toggle,
            200 => Self::On,
            201 => Self::Off,
            _ => return Err(()),
        })
    }
}

impl ToString for TouchPad {
    fn to_string(&self) -> String {
        format!(
            "TouchPad_{}",
            match self {
                TouchPad::On => "On",
                TouchPad::Off => "Off",
                TouchPad::Toggle => "Toggle",
            },
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum Audio {
    Volume(AudioVolume),
    Play,
    Next,
    Prev,
    Stop,
    Pause,
    MuteMicrophone,
    Record,
    Preset,
    Rewind,
    Forward,
    Media,
}

impl TryFrom<u8> for Audio {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Ok(volume) = AudioVolume::try_from(value) {
            return Ok(Self::Volume(volume));
        }
        Ok(match value {
            172 | 208 | 215 => Self::Play,
            171 => Self::Next,
            209 => Self::Pause,
            174 => Self::Stop,
            198 => Self::MuteMicrophone,
            175 => Self::Record,
            173 => Self::Prev,
            221 => Self::Preset,
            176 => Self::Rewind,
            216 => Self::Forward,
            234 => Self::Media,
            _ => return Err(()),
        })
    }
}

impl ToString for Audio {
    fn to_string(&self) -> String {
        format!(
            "Audio_{}",
            match self {
                Audio::Volume(volume) => volume.to_string().as_str(),
                Audio::Play => "Play",
                Audio::Next => "Next",
                Audio::Prev => "Prev",
                Audio::Stop => "Stop",
                Audio::Pause => "Pause",
                Audio::MuteMicrophone => "Mute_Micro",
                Audio::Record => "Record",
                Audio::Preset => "Preset",
                Audio::Rewind => "Rewind",
                Audio::Forward => "Forward",
                Audio::Media => "Media",
            }
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum AudioVolume {
    Lower,
    Rise,
    Mute,
}

impl ToString for AudioVolume {
    fn to_string(&self) -> String {
        format!(
            "Vol{}",
            match self {
                AudioVolume::Lower => "-",
                AudioVolume::Rise => "+",
                AudioVolume::Mute => "Mute",
            }
        )
    }
}

impl TryFrom<u8> for AudioVolume {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            121 => Self::Mute,
            122 => Self::Lower,
            123 => Self::Rise,
            _ => return Err(()),
        })
    }
}
