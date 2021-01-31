use nsengine::CREDITS_TEXT;

fn main() -> ggez::GameResult {
    unsafe { CREDITS_TEXT = "Default credits" };
    nsengine::run(Some(include_bytes!("../resources.zip").to_vec()))
}
