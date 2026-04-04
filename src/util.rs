#[macro_export]
macro_rules! __log {
    ($val: expr $(,)?) => (
        web_sys::console::log_1(&format!("[{}:{}] {}", std::file!(), std::line!(), $val).into());
    )
}

#[macro_export]
macro_rules! __dbg {
    ($val: expr $(,)?) => {
        match $val {
            tmp => {
                web_sys::console::debug_1(&format!("[{}:{}] {} = {:#?}", std::file!(), std::line!(), std::stringify!($val), &tmp).into());
                tmp
            }
        }
    }
}
