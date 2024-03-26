/// Return string with red color
/// # Examples:
/// ```
/// use create_new_library::colors::red;
/// let s = red("Hello");
/// ```
pub fn red(s:&str) -> String {
    format!("\x1b[31m{}\x1b[0m",s)
}