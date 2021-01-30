use nsengine::CREDITS_TEXT;

fn main() -> ggez::GameResult {
    unsafe {
        CREDITS_TEXT = "Note From Developers:
Thank you for playing our Winter VN Jam submission!
Due to school and other life stuff, the story isn't as polished as we wanted it to be. It is rather rushed, we apologize.
Credits
Project Lead:
- Programmer: [Nobbele](https://nobbele.itch.io/)
-  Character Artist: [TheJayDuck](https://thejayduck.github.io/)
- Background Photos: [TheJayDuck](https://thejayduck.github.io/) & [Nobbele](https://nobbele.itch.io/)
- Story: [Nobbele](https://nobbele.itch.io/)
- SFX: [TheJayDuck](https://thejayduck.github.io/)
- Proofreader: [TheJayDuck](https://thejayduck.github.io/) & [Nobbele](https://nobbele.itch.io/)
Additional Backgrounds:
- Google
Additional Music
- [MusMus](https://musmus.main.jp/)"
    };
    nsengine::run(Some(include_bytes!("../resources.zip").to_vec()))
}
