pub mod public_lib {
    use std::collections::HashMap;
    use std::env;
    use std::fmt::{Display, Formatter};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};
    use tokio_tungstenite::tungstenite::Message;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct IpAddresses {
        pub v4: Option<Ipv4Addr>,
        pub v6: Option<Ipv6Addr>,
    }

    impl IpAddresses {
        pub fn empty() -> IpAddresses {
            IpAddresses {
                v4: None,
                v6: None,
            }
        }

        pub fn new_with_one_address(ip: IpAddr) -> IpAddresses {
            match ip {
                IpAddr::V4(v4) => IpAddresses {
                    v4: Some(v4),
                    v6: None,
                },
                IpAddr::V6(v6) => IpAddresses {
                    v4: None,
                    v6: Some(v6),
                },
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
        },
        Acknowledge,
        AddrRequest,
        AddrResponse {
            adapter_addresses: HashMap<String, IpAddresses>
        },
        Error {
            message: String
        },
    }

    impl MessagePack {
        pub fn to_message(&self) -> Message {
            Message::Text(self.to_string())
        }

        pub fn to_warp_message(&self) -> warp::ws::Message {
            warp::ws::Message::text(self.to_string())
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

    const ENV_RUST_LOG: &str = "RUST_LOG";

    pub fn set_default_logger_level(log_level: &str) {
        if env::var(ENV_RUST_LOG).is_err() {
            env::set_var(ENV_RUST_LOG, log_level);
        }
    }
}