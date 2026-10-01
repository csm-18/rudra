use crate::diagnostics::create_diagnostic;

pub fn parse(args: &mut Vec<String>) {
    let error = create_diagnostic(5083, &["hello.txt"]);
    if let Some(err) = error {
        err.print();
    }
}
