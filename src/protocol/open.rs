use std::fs::OpenOptions;

use super::{impl_packet_for, impl_request_id, FileAttributes, Packet, RequestId};

/// Opening flags according to the specification
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct OpenFlags(u32);

bitflags! {
    impl OpenFlags: u32 {
        const READ = 0x00000001;
        const WRITE = 0x00000002;
        const APPEND = 0x00000004;
        const CREATE = 0x00000008;
        const TRUNCATE = 0x00000010;
        const EXCLUDE = 0x00000020;
    }
}

impl From<OpenFlags> for OpenOptions {
    fn from(value: OpenFlags) -> Self {
        let mut open_opts = &mut OpenOptions::new();
        if value.contains(OpenFlags::READ) {
            open_opts = open_opts.read(true);
        }
        if value.contains(OpenFlags::WRITE) {
            open_opts = open_opts.write(true);
        }
        if value.contains(OpenFlags::APPEND) {
            open_opts = open_opts.append(true);
        }
        if value.contains(OpenFlags::CREATE) {
            open_opts = open_opts.create(true);
        }
        if value.contains(OpenFlags::TRUNCATE) {
            open_opts = open_opts.truncate(true);
        }
        if value.contains(OpenFlags::EXCLUDE) {
            // TODO
            log::warn!("Found flag OpenFlags::EXCLUDE")
        }

        open_opts.clone()
    }
}

/// Implementation for SSH_FXP_OPEN
#[derive(Debug, Serialize, Deserialize)]
pub struct Open {
    pub id: u32,
    pub filename: String,
    pub pflags: OpenFlags,
    pub attrs: FileAttributes,
}

impl_request_id!(Open);
impl_packet_for!(Open);
