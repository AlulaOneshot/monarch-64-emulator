pub struct MonadBootCartridge {
    pub(crate) data: Vec<u8>,
    pub(crate) revision: u8,
}

impl MonadBootCartridge {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            revision: 0,
        }
    }

    pub(crate) fn get_revision(&self) -> u8 {
        self.revision
    }

    pub(crate) fn get_data(&self) -> &Vec<u8> {
        &self.data
    }
}
