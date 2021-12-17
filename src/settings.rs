use crate::app::Settings;

/// This `SettingElement`s are supposed to be built-in the program
pub struct SettingElement {
    pub description: &'static str,
    pub setting_type: SettingType,
    pub read_value: &'static dyn Fn(&Settings) -> SettingValue,
    pub set_value: &'static dyn Fn(&mut Settings, SettingValue) -> Result<(), ()>,
}

#[allow(dead_code)]
pub enum SettingType {
    Bool,
    String,
    Range(u32, u32),
}

impl SettingType {
    pub fn width(&self) -> usize {
        match self {
            Self::Bool => 3,
            Self::String => 10,
            Self::Range(_, _) => 8,
        }
    }
}

#[allow(dead_code)]
pub enum SettingValue {
    Bool(bool),
    String(String),
    Range(u32),
}

pub const SETTING_ELEMENTS: [SettingElement; 1] = [SettingElement {
    description: "Message Read receipts.",
    setting_type: SettingType::Bool,
    read_value: &|s: &Settings| SettingValue::Bool(s.send_receipts),
    set_value: &|s, v| match v {
        SettingValue::Bool(b) => {
            s.send_receipts = b;
            Ok(())
        }
        _ => Err(()),
    },
}];
