pub use chrono::Local;
pub use gettid::gettid;

#[macro_export]
macro_rules! aaa {
    ($($arg:tt)*) => {{
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut s = String::from(format!("[{} {} {}:{}] ", $crate::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), $crate::gettid(),  file!(), line!()));
        s += &String::from(format!($($arg)*));

        if let Ok(mut f) = OpenOptions::new().write(true).append(true).create(true).open("/tmp/debug.txt") {
            writeln!(f, "{}", s).unwrap();
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        aaa!("{} {}", "aaaa", 123);
        assert_eq!(2 + 2, 4);
    }
}
