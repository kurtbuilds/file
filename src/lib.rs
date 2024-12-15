use std::fmt::Debug;

#[derive(Clone)]
pub struct File {
    pub name: String,
    pub content: Vec<u8>,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("name", &self.name)
            .field("content", &format!("{} bytes", self.content.len()))
            .finish()
    }
}

impl File {
    pub fn mime_type(&self) -> String {
        mime_guess::from_path(&self.name)
            .first_or_octet_stream()
            .to_string()
    }
}
