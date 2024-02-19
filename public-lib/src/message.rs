use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpAddresses {
    pub name: String,
    pub v4: Option<Ipv4Addr>,
    pub v6: Option<Ipv6Addr>,
}

impl IpAddresses {
    pub fn empty(name: String) -> IpAddresses {
        IpAddresses {
            name,
            v4: None,
            v6: None,
        }
    }

    pub fn append_address(&mut self, ip: IpAddr) -> &mut Self {
        match ip {
            IpAddr::V4(v4) => self.v4 = Some(v4),
            IpAddr::V6(v6) => self.v6 = Some(v6),
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessagePack {
    Establish {
        id: Uuid,
        password: String,
    },
    Acknowledge,
    AddrRequest,
    AddrResponse {
        adapter_addresses: Vec<IpAddresses>
    },
    Error {
        message: String
    },
}

impl MessagePack {
    pub fn to_message(&self) -> Message {
        Message::Text(self.to_string())
    }

    pub fn to_framework_message(&self) -> axum::extract::ws::Message {
        axum::extract::ws::Message::Text(self.to_string())
    }
}

impl Display for MessagePack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl FromStr for MessagePack {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

