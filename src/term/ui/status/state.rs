#[derive(Default)]
pub struct StatusState {
    pub text: String,
}
impl From<&str> for StatusState {
    fn from(s: &str) -> Self {
        Self { text: s.into() }
    }
}
impl StatusState {
    pub fn from<T: Into<String>>(s: T) -> Self {
        Self { text: s.into() }
    }
    pub fn set_text<T: Into<String>>(&mut self, s: T) {
        self.text = s.into();
    }
}
