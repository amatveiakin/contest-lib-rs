#[allow(non_snake_case)]
pub trait BoolExtension {
    fn YesNo(self) -> &'static str;
    fn YESNO(self) -> &'static str;
}

impl BoolExtension for bool {
    fn YesNo(self) -> &'static str {
        if self {
            "Yes"
        } else {
            "No"
        }
    }
    fn YESNO(self) -> &'static str {
        if self {
            "YES"
        } else {
            "NO"
        }
    }
}
