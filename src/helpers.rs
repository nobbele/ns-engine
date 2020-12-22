use ggez::{
    graphics::{self, drawable_size},
    Context,
};

pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Position {
    pub fn add_in(&self, ctx: &Context, offset: glam::Vec2) -> glam::Vec2 {
        self.add_in_from(
            &graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: drawable_size(ctx).0,
                h: drawable_size(ctx).1,
            },
            offset,
        )
    }

    pub fn add_in_from(&self, rect: &graphics::Rect, offset: glam::Vec2) -> glam::Vec2 {
        match self {
            Position::TopLeft => glam::Vec2::new(rect.left(), rect.top()) + offset,
            Position::TopRight => {
                glam::Vec2::new(rect.right(), rect.top()) + glam::Vec2::new(-offset.x, offset.y)
            }
            Position::BottomLeft => {
                glam::Vec2::new(rect.left(), rect.bottom()) + glam::Vec2::new(offset.x, -offset.y)
            }
            Position::BottomRight => glam::Vec2::new(rect.right(), rect.bottom()) - offset,
            Position::Center => glam::Vec2::new(rect.right() / 2.0, rect.bottom() / 2.0) + offset,
        }
    }
}

#[test]
fn test_position() {
    let canvas = graphics::Rect {
        x: 0.0,
        y: 0.0,
        w: 1280.0,
        h: 720.0,
    };

    assert_eq!(Position::Center.add_in_from(&canvas, glam::Vec2::new(0.0, 0.0)), glam::Vec2::new(1280.0 / 2.0, 720.0 / 2.0));
    assert_eq!(Position::TopRight.add_in_from(&canvas, glam::Vec2::new(0.0, 0.0)), glam::Vec2::new(1280.0, 0.0));
    assert_eq!(Position::TopRight.add_in_from(&canvas, glam::Vec2::new(10.0, 10.0)), glam::Vec2::new(1270.0, 10.0));
    assert_eq!(Position::BottomLeft.add_in_from(&canvas, glam::Vec2::new(10.0, 10.0)), glam::Vec2::new(10.0, 710.0));
}

pub fn points_to_rect(a: glam::Vec2, b: glam::Vec2) -> graphics::Rect {
    graphics::Rect {
        x: a.x,
        y: a.y,
        w: b.x - a.x,
        h: b.y - a.y,
    }
}
