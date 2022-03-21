#[derive(Debug)]
pub enum HelloError {
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
pub struct Hello {
    pub process_id: i32,
}

impl Hello {
    const PREFIX: [u8; 5] = [0x09, 0x01, 0x00, 0x00, 0x00];

    pub fn new(process_id: i32) -> Result<Self, HelloError> {
        // no negatives!
        if process_id < 0 {
            Err(HelloError::InvalidProcessId)
        } else {
            Ok(Self { process_id })
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HelloError> {
        // validate size
        let len = bytes.len();

        if len < 13 {
            return Err(HelloError::TooSmall);
        }

        if len > 13 {
            return Err(HelloError::TooLarge);
        }

        // validate prefix
        let prefix = &bytes[..5];

        if prefix != Self::PREFIX {
            return Err(HelloError::InvalidPrefix);
        }

        // get ids
        let id = &bytes[5..9];
        let id2 = &bytes[9..];

        let id = i32::from_le_bytes(unsafe { *(id.as_ptr() as *const [u8; 4]) });
        let id2 = i32::from_le_bytes(unsafe { *(id2.as_ptr() as *const [u8; 4]) });

        if id != id2 {
            return Err(HelloError::ProcessIdMismatch);
        }

        if id < 0 {
            return Err(HelloError::InvalidProcessId);
        }

        Ok(Self { process_id: id })
    }

    pub fn to_bytes(&self) -> [u8; 13] {
        let mut bytes = [0; 13];
        let process_id = self.process_id.to_le_bytes();

        bytes[..5].copy_from_slice(&Self::PREFIX);
        bytes[5..9].copy_from_slice(&process_id);
        bytes[9..].copy_from_slice(&process_id);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::Hello;

    #[test]
    fn from_bytes() {
        let new = Hello::new(20197).expect("valid process id");
        let from_bytes = Hello::from_bytes(&[
            0x09, 0x01, 0x00, 0x00, 0x00, 0xE5, 0x4E, 0x00, 0x00, 0xE5, 0x4E, 0x00, 0x00,
        ])
        .expect("valid hello packet");

        assert_eq!(new.process_id, from_bytes.process_id);
    }
}
