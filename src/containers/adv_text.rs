use ggez::{Context, event::MouseButton, graphics::{Drawable, Rect, Text}, mint};

pub enum Format {
    Link(String), // the url
}

pub struct FormatEntry {
    pub format: Format,
    pub start: usize,
    pub end: usize,
}

pub struct AdvancedText {
    formatting: Vec<FormatEntry>,
    text: Text,
}

impl AdvancedText {
    pub fn new(content: &str) -> Self {
        let mut text = Text::default();
        let mut formatting = Vec::new();

        let mut chars_it = content.chars().enumerate();

        while let Some((n, c)) = chars_it.by_ref().next() {
            if c == '[' {
                let link_text = chars_it.by_ref()
                    .map(|(_, c)| c)
                    .take_while(|c | *c != ']')
                    .collect::<String>();
                if let Some((_, c)) = chars_it.by_ref().next() {
                    // false positive, it was not a link
                    if c != '(' {
                        text.add(c);
                        continue; 
                    }
                }
                let link_url = chars_it.by_ref()
                    .map(|(_, c)| c)
                    .take_while(|c | *c != ')')
                    .collect::<String>();
                formatting.push(FormatEntry {
                    start: n,
                    end: n + link_text.len(),
                    format: Format::Link(link_url),
                });
                for c in link_text.chars() {
                    text.add(c);
                }
            } else {
                text.add(c);
            }
        }

        Self {
            formatting,
            text,
        }
    }

    pub fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        for format in &self.formatting {
            let positions = &self.text.positions(ctx)[format.start..format.end];
            let bounds = Rect {
                x: positions[0].x,
                y: positions[0].y - 11.0,
                w: positions[positions.len() - 1].x - positions[0].x,
                h: positions[positions.len() - 1].y - (positions[0].y - 11.0),
            };
            if bounds.contains(mint::Point2 { x, y }) {
                match &format.format {
                    Format::Link(url) => {
                        webbrowser::open(url).unwrap();
                    }
                }
            }
        }
    }
}

impl Drawable for AdvancedText {
    fn draw(&self, ctx: &mut Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        self.text.draw(ctx, param).unwrap();
        Ok(())
    }
}