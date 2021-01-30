use ggez::{
    event::MouseButton,
    graphics::{Color, Drawable, Rect, Text, TextFragment},
    mint, Context,
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

        // count newlines to offset the character index
        let mut newline_count = 0;

        while let Some((n, c)) = chars_it.by_ref().next() {
            if c == '\n' {
                newline_count += 1;
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
                formatting.push(FormatEntry {
                    start: n - newline_count,
                    end: n + link_text.len() - newline_count,
                    format: Format::Link(link_url),
                });
                for c in link_text.chars() {
                    let mut frag: TextFragment = c.into();
                    frag.color = Some(Color::from_rgb_u32(0xCC_66_00));
                    text.add(frag);
                }
            } else {
                text.add(c);
            }
        }

        Self { formatting, text }
    }
}

impl Drawable for RichText {
    fn draw(&self, ctx: &mut Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.text.draw(ctx, param).unwrap();
        Ok(())
    }
}
