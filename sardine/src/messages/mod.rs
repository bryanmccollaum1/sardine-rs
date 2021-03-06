mod srd_accept;
mod srd_confirm;
mod srd_delegate;
mod srd_header;
mod srd_initiate;
mod srd_message;
mod srd_offer;

pub const SRD_SIGNATURE: u32 = 0x00445253;

pub mod srd_msg_id {
    pub const SRD_INITIATE_MSG_ID: u8 = 1;
    pub const SRD_OFFER_MSG_ID: u8 = 2;
    pub const SRD_ACCEPT_MSG_ID: u8 = 3;
    pub const SRD_CONFIRM_MSG_ID: u8 = 4;
    pub const SRD_DELEGATE_MSG_ID: u8 = 5;
}

pub mod srd_flags {
    pub const SRD_FLAG_MAC: u16 = 0x0001;
    pub const SRD_FLAG_CBT: u16 = 0x0002;
}

pub use messages::srd_accept::SrdAccept;
pub use messages::srd_confirm::SrdConfirm;
pub use messages::srd_delegate::SrdDelegate;
pub use messages::srd_header::SrdHeader;
pub use messages::srd_initiate::SrdInitiate;
pub use messages::srd_message::Message;
pub use messages::srd_message::SrdMessage;
pub use messages::srd_offer::SrdOffer;

pub use messages::srd_accept::new_srd_accept_msg;
pub use messages::srd_confirm::new_srd_confirm_msg;
pub use messages::srd_delegate::new_srd_delegate_msg;
pub use messages::srd_initiate::new_srd_initiate_msg;
pub use messages::srd_offer::new_srd_offer_msg;

fn expand_start<T: Default>(buffer: &mut Vec<T>, new_size: usize) {
    if new_size > buffer.len() {
        for _ in 0..(new_size - buffer.len()) {
            buffer.insert(0, Default::default());
        }
    }
}
