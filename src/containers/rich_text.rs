use ggez::{
    graphics::{Color, Drawable, Text, TextFragment},
    Context,
};

#[derive(Debug)]
pub enum Format {
    Link(String), // the url
}

#[derive(Debug)]
pub struct FormatEntry {
    pub format: Format,
    pub start: usize,
    pub end: usize,
}

pub struct RichText {
    pub formatting: Vec<FormatEntry>,
    pub text: Text,
}

impl RichText {
    pub fn new(content: &str, mut text: Text) -> Self {
        let mut formatting = Vec::new();

        let mut chars_it = content.chars().enumerate();

        // count newlines, link length and parse character to offset the character index
        let mut skip_char_count = 0;

        while let Some((n, c)) = chars_it.by_ref().next() {
            if c == '\n' {
                skip_char_count += 1;
            }
            if c == '[' {
                let link_text = chars_it
                    .by_ref()
                    .map(|(_, c)| c)
                    .take_while(|c| *c != ']')
                    .collect::<String>();
                if let Some((_, c)) = chars_it.by_ref().next() {
                    // false positive, it was not a link
                    if c != '(' {
                        text.add(c);
                        continue;
                    }
                }
                let link_url = chars_it
                    .by_ref()
                    .map(|(_, c)| c)
                    .take_while(|c| *c != ')')
                    .collect::<String>();
                let link_url_len = link_url.len();
                formatting.push(FormatEntry {
                    start: n - skip_char_count - 1,                 // skip first (
                    end: n + link_text.len() - skip_char_count - 1, // skip first (
                    format: Format::Link(link_url),
                });
                for c in link_text.chars() {
                    let mut frag: TextFragment = c.into();
                    frag.color = Some(Color::from_rgb_u32(0xCC_66_00));
                    text.add(frag);
                }
                skip_char_count += link_url_len + 4 // []() is 4 characters
            } else {
                text.add(c);
            }
        }

        Self { formatting, text }
    }

    /* Mouse up is impl'd in sprite.rs */
}

impl Drawable for RichText {
    fn draw(&self, ctx: &mut Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.text.draw(ctx, param).unwrap();
        Ok(())
    }
}
