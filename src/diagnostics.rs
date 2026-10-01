use std::sync::LazyLock;

use crate::utils;

static CSM_FUNNY_COMMENTS: LazyLock<bool> =
    LazyLock::new(|| utils::env_var_exits("CSM_FUNNY_COMMENTS"));

#[derive(Clone)]
pub struct Diagnostic {
    pub code: isize,
    pub message: String,
    pub category: String,
    pub comment: String,
}
impl Diagnostic {
    pub fn print(&self) {
        println!("{} TS{}: {}", self.category, self.code, self.message);
        if *CSM_FUNNY_COMMENTS {
            println!("  {}", self.comment);
        }
    }
}

pub static DIAGNOSTICS: LazyLock<Vec<Diagnostic>> = LazyLock::new(|| {
    vec![Diagnostic {
        code: 5083,
        message: "Cannot read file '{}'.".to_owned(),
        category: "error".to_owned(),
        comment: "Oye, file kithe hai? 😂".to_owned(),
    }]
});

pub fn create_diagnostic(code: isize, args: &[&str]) -> Option<Diagnostic> {
    let d = DIAGNOSTICS.iter().find(|d| d.code == code)?;

    let mut diagnostic = d.clone();
    let mut msg = diagnostic.message;
    for arg in args {
        msg = msg.replacen("{}", arg, 1);
    }

    diagnostic.message = msg;
    Some(diagnostic)
}
