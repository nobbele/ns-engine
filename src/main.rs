fn main() -> ggez::GameResult {
    nsengine::run(include_bytes!("../resources.zip").to_vec())
}