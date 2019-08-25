use ggez::nalgebra as na;
use ggez::{graphics, Context};

pub struct Screen {
    width: f32,
    height: f32,
    #[allow(dead_code)]
    scaled_width: f32,
    #[allow(dead_code)]
    scaled_height: f32,
}

impl Screen {
    pub fn new(ctx: &Context) -> Self {
        let logical_screen = graphics::screen_coordinates(&ctx);
        let (drawable_w, drawable_h) = graphics::drawable_size(&ctx);

        let (logical_w, logical_h) = (logical_screen.w, logical_screen.h);
        let (scaled_w, scaled_h) = (drawable_w / logical_w, drawable_h / logical_h);

        Screen {
            width: logical_w,
            height: logical_h,
            scaled_width: scaled_w,
            scaled_height: scaled_h,
        }
    }

    #[allow(dead_code)]
    pub fn to_screen_coordinate(&self, point: na::Point2<f32>) -> na::Point2<f32> {
        na::Point2::new(point.x * self.scaled_width, point.y * self.scaled_height)
    }

    pub fn center_fit(&self, rect: &graphics::Rect) -> graphics::Rect {
        let side_margins = (self.width - rect.w) / 2.0;
        let height_margins = (self.height - rect.h) / 2.0;

        graphics::Rect::new(side_margins, height_margins, rect.w, rect.h)
    }
}
