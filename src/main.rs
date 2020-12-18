fn main() -> ggez::GameResult {
    nsengine::run(Some(include_bytes!("../resources.zip").to_vec()))
}
