pub fn env_var_exits(name: &str) -> bool {
    if std::env::var_os(name).is_some() {
        return true;
    }
    false
}
