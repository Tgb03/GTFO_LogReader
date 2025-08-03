use std::ffi::CString;

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum SubscribeCode {
    #[default]
    Tokenizer = 1,
    RunInfo = 2,
    Mapper = 3,
    SeedIndexer = 4,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum SubscriptionType {
    #[default]
    JSON = 1,
    BITDATA = 2,
    CSV = 3,
}

impl SubscriptionType {
    pub fn convert<O>(&self, data: &O) -> Option<CString>
    where
        O: Serialize,
    {
        match self {
            SubscriptionType::JSON => {
                return serde_json::to_string(data)
                    .ok()
                    .map(|v| CString::new(v).ok())
                    .flatten();
            }
            SubscriptionType::BITDATA => {
                let arr = bincode::serialize(data).ok()?;
                return CString::from_vec_with_nul(arr).ok();
            }
            SubscriptionType::CSV => todo!(),
        }
    }
}
