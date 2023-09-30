pub trait BoolExtension {
    fn yesno(self) -> &'static str;
}

impl BoolExtension for bool {
    fn yesno(self) -> &'static str {
        if self {
            "YES"
        } else {
            "NO"
        }
    }
}
