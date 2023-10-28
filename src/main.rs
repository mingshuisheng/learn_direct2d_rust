use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F};
use windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE;

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    graphic.draw_and_save((640, 480), w!("output.png"), |ctx| unsafe {
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
        //Ellipse

        let e = D2D1_ELLIPSE{
            point: D2D_POINT_2F{
                x: 100.0,
                y: 100.0
            },
            radiusX: 20.0,
            radiusY: 30.0,
        };

        ctx.FillEllipse(&e, &brush);
    });
}