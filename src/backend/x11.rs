use breadx::{
    display::DisplayConnection,
    prelude::*,
    protocol::{
        xproto::{AtomEnum, ChangeWindowAttributesAux, EventMask},
        Event,
    },
};

use super::{Backend, BackendError, WindowAttribute};

pub struct X11 {
    connection: DisplayConnection,
    active_window_atom: u32,
    window_name_atom: u32,
    window_class_atom: u32,
}

impl Backend for X11 {
    fn create() -> Result<Self, BackendError> {
        (|| {
            let mut connection = DisplayConnection::connect(None)?;
            connection.change_window_attributes_checked(
                connection.default_screen().root,
                ChangeWindowAttributesAux::default().event_mask(EventMask::PROPERTY_CHANGE),
            )?;
            connection.flush()?;
            let active_window_atom = connection
                .intern_atom_immediate(true, "_NET_ACTIVE_WINDOW")?
                .atom;
            let window_name_atom = connection.intern_atom_immediate(true, "WM_NAME")?.atom;
            let window_class_atom = connection.intern_atom_immediate(true, "WM_CLASS")?.atom;

            Ok::<X11, Box<dyn std::error::Error>>(Self {
                connection,
                active_window_atom,
                window_name_atom,
                window_class_atom,
            })
        })()
        .map_err(|x| BackendError::Initialize { source: x })
    }

    fn active_window_matches<F>(&mut self, attribute: WindowAttribute, mut predicate: F) -> bool
    where
        F: FnMut(&str) -> bool,
    {
        (|| {
            let root = self.connection.default_screen().root;
            let active_window_id = self
                .connection
                .get_property_immediate(
                    false,
                    root,
                    self.active_window_atom,
                    u8::from(AtomEnum::WINDOW),
                    0,
                    1,
                )
                .ok()?
                .value32()?
                .next()?;
            // https://github.com/bread-graphics/breadx/issues/92
            let any_type_atom: u8 = breadx::protocol::xproto::AtomEnum::ANY.into();

            match attribute {
                WindowAttribute::Name => {
                    let window_title = self
                        .connection
                        .get_property_immediate(
                            false,
                            active_window_id,
                            self.window_name_atom,
                            any_type_atom,
                            0,
                            1024,
                        )
                        .ok()?
                        .value;
                    println!("{:?}", &String::from_utf8_lossy(&window_title));
                    Some(predicate(&String::from_utf8_lossy(&window_title)))
                }
                WindowAttribute::Class => {
                    let window_class = self
                        .connection
                        .get_property_immediate(
                            false,
                            active_window_id,
                            self.window_class_atom,
                            any_type_atom,
                            0,
                            1024,
                        )
                        .ok()?
                        .value;

                    let class = String::from_utf8_lossy(&window_class);
                    Some(
                        (&class)
                            .split('\0')
                            .filter(|s| !s.is_empty())
                            .any(predicate),
                    )
                }
            }
        })()
        .unwrap_or(false)
    }

    fn wait_for_active_window(&mut self) {
        'main: loop {
            if let Ok(event) = self.connection.wait_for_event() {
                match event {
                    Event::PropertyNotify(event) if event.atom == self.active_window_atom => {
                        break 'main;
                    }
                    _ => {}
                }
            }
        }
    }
}
