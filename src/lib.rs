extern crate libc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate libnss;
#[macro_use]
extern crate log;
extern crate env_logger;

use libnss::host::{Addresses, Host, HostHooks};
use std::net::{IpAddr, Ipv4Addr};

struct ZeroTierHosts;
libnss_host_hooks!(zerotier, ZeroTierHosts);

// /var/lib/zerotier-one/authtoken.secret
// $HOME/.zeroTierOneAuthToken

// https://github.com/rust-lang/rust/issues/54691
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
pub static INITIALIZE: extern "C" fn() = nss_zerotier_initialize;

#[no_mangle]
pub extern "C" fn nss_zerotier_initialize() {
    env_logger::init();
}

impl HostHooks for ZeroTierHosts {
    fn get_all_entries() -> Vec<Host> {
        debug!("Getting all ZeroTier hosts");
        vec![Host {
            name: "what.zt".to_string(),
            addresses: Addresses::V4(vec![Ipv4Addr::new(128, 0, 0, 1), Ipv4Addr::new(129, 0, 0, 1)]),
            aliases: vec!["who.zt".to_string()],
        }]
    }

    fn get_host_by_addr(addr: IpAddr) -> Option<Host> {
        debug!("Looking up host by address {}", addr);
        Some(Host {
            name: "what.zt".to_string(),
            addresses: Addresses::V4(vec![Ipv4Addr::new(128, 0, 0, 1), Ipv4Addr::new(129, 0, 0, 1)]),
            aliases: vec!["who.zt".to_string()],
        })
    }

    fn get_host_by_name(name: &str) -> Option<Host> {
        debug!("Looking up host by name {}", name);
        // if name.ends_with(".zt") {
        Some(Host {
            name: "what.zt".to_string(),
            addresses: Addresses::V4(vec![Ipv4Addr::new(128, 0, 0, 1), Ipv4Addr::new(129, 0, 0, 1)]),
            aliases: vec!["who.zt".to_string()],
        })
        // } else {
        //     None
        // }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
