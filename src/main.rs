use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_RECT_F};
use windows::Win32::Graphics::Direct2D::D2D1_ROUNDED_RECT;

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    graphic.draw_and_save((640, 680), w!("output.png"), |ctx| unsafe {
        ctx.Clear(Some(&D2D1_COLOR_F{
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }));


        let brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F{
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }, None).unwrap();

        //RoundedRectangle
        let round_rect = D2D1_ROUNDED_RECT{
            rect: D2D_RECT_F{
                left: 100.0,
                top: 100.0,
                right: 400.0,
                bottom: 500.0,
            },
            radiusX: (400.0 - 100.0) / 2.0,
            radiusY: (500.0 - 100.0) / 2.0,
        };

        ctx.DrawRoundedRectangle(&round_rect, &brush, 10.0, None);
        // ctx.FillRoundedRectangle(&round_rect, &brush);
    });
}