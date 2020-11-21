use ggez::{
    graphics::{self, drawable_size},
    mint as na, Context,
};

pub fn get_item_y(ctx: &Context, n: f32, max: f32) -> f32 {
    let offset = (-50.0 * max / 2.0) + (50.0 * n);
    Position::Center.add_in(ctx, (0.0, offset)).y
}

pub fn get_item_index(ctx: &Context, y: f32, max: f32) -> u32 {
    let start_y = get_item_y(ctx, 0.0, max);
    ((y - start_y) / 50.0) as u32
}

pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Position {
    pub fn add_in(&self, ctx: &Context, offset: (f32, f32)) -> na::Point2<f32> {
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

    pub fn add_in_from(&self, rect: &graphics::Rect, offset: (f32, f32)) -> na::Point2<f32> {
        match self {
            Position::TopLeft => [rect.x + offset.0, rect.y + offset.1],
            Position::TopRight => [(rect.x + rect.w) - offset.0, rect.y + offset.1],
            Position::BottomLeft => [rect.x + offset.0, (rect.y + rect.h) - offset.1],
            Position::BottomRight => [(rect.x + rect.w) - offset.0, (rect.y + rect.h) - offset.1],
            Position::Center => [
                (rect.x + rect.w) / 2.0 + offset.0,
                (rect.y + rect.h) / 2.0 + offset.1,
            ],
        }
        .into()
    }
}

pub fn points_to_rect(a: na::Point2<f32>, b: na::Point2<f32>) -> graphics::Rect {
    graphics::Rect {
        x: a.x,
        y: a.y,
        w: b.x - a.x,
        h: b.y - a.y,
    }
}
