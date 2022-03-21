#[derive(Debug)]
pub enum IdentifyError {
    // process id appears twice in the packet for some reason
    ProcessIdMismatch,
    // if the process id is negative
    InvalidProcessId,
    // if a different prefix is provided
    InvalidPrefix,
    // data is too small
    TooSmall,
    // data is too large
    TooLarge,
}

#[derive(Debug)]
pub struct Identify {
    pub app_id: u32,
}

impl Identify {
    const PREFIX: [u8; 10] = [0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0xCB, 0x80, 0x78, 0x33];
    const SUFFIX: [u8; 5] = [0x01, 0x2D, 0xA3, 0x0B, 0x35];

    pub fn new(app_id: u32) -> Self {
        Self { app_id }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, IdentifyError> {
        // validate size
        let len = bytes.len();

        if len < 19 {
            return Err(IdentifyError::TooSmall);
        }

        if len > 19 {
            return Err(IdentifyError::TooLarge);
        }

        //let prefix = &bytes[..10];
        let app_id = &bytes[10..14];
        //let suffix = &bytes[14..];

        let app_id = u32::from_le_bytes(unsafe { *(app_id.as_ptr() as *const [u8; 4]) });

        Ok(Self { app_id })
    }

    pub fn to_bytes(&self) -> [u8; 19] {
        let mut bytes = [0; 19];
        let app_id_bytes = self.app_id.to_le_bytes();

        bytes[..10].copy_from_slice(&Self::PREFIX);
        bytes[10..14].copy_from_slice(&app_id_bytes);
        bytes[14..].copy_from_slice(&Self::SUFFIX);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::Identify;

    #[test]
    fn from_bytes() {
        let new = Identify::new(730);
        let from_bytes = Identify::from_bytes(&[
            0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0xCB, 0x80, 0x78, 0x33, 0xDA, 0x02, 0x00, 0x00,
            0x01, 0x2D, 0xA3, 0x0B, 0x35,
        ])
        .expect("valid identify packet");

        assert_eq!(new.app_id, from_bytes.app_id);
    }
}
