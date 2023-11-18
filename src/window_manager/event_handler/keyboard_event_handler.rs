use std::{borrow::BorrowMut, cell::RefCell, ops::Index};

use xcb::x::Keycode;

pub struct KeyboardEventHandler {
    input_buffer: RefCell<Vec<KeyCode>>,
}

impl KeyboardEventHandler {
    pub fn new() -> KeyboardEventHandler {
        KeyboardEventHandler {
            input_buffer: RefCell::new(Vec::new()),
        }
    }
    fn input_buffer(&self) -> &mut Vec<KeyCode> {
        self.borrow_mut()
    }
    pub fn on_press(&self, event: xcb::x::KeyPressEvent) {
        self.input_buffer
            .borrow_mut()
            .push(KeyCode::from(event.detail()))
    }

    pub fn on_relese(&self, event: xcb::x::KeyReleaseEvent) {
        if self.input_buffer().contains(&Keycode::from(event.detail())) {
            let index = self
                .input_buffer()
                .iter()
                .position(|key| key == KeyCode::from(event.detail))
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

pub enum KeyPosition {
    Left,
    Right,
}

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
    UnRecognized,
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

pub enum Monitor {
    RiseBrightness,
    LowerBrightness,
    BrightnessCycle,
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

pub enum AudioVolume {
    Lower,
    Rise,
    Mute,
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
