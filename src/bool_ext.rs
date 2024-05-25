#[allow(non_snake_case)]
pub trait BoolExtension {
    fn YesNo(self) -> &'static str;
    fn yesno(self) -> &'static str;
}

impl BoolExtension for bool {
    fn YesNo(self) -> &'static str {
        if self {
            "Yes"
        } else {
            "No"
        }
    }
    // TODO: Rename to YESNO.
    fn yesno(self) -> &'static str {
        if self {
            "YES"
        } else {
            "NO"
        }
    }
}
