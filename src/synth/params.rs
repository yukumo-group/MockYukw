// AqVersion defines versions for aquestalk
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AqVersion {
    V1 = 1,
    V2 = 2,
    V10 = 10,
}

impl AqVersion {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::V1),
            2 => Some(Self::V2),
            10 => Some(Self::V10),
            _ => None,
        }
    }

    pub fn to_str(&self) -> Option<&'static str>{
        match self{
            AqVersion::V1 => Some("AQ1"),
            AqVersion::V2 => Some("AQ2"),
            AqVersion::V10 => Some("AQ10"),
            _ => None
        }
    }
}

