use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F, D2D_SIZE_F};
use windows::Win32::Graphics::Direct2D::{D2D1_ARC_SEGMENT, D2D1_ELLIPSE};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let geometry = unsafe {
        let geometry = graphic.d2d_factory.CreatePathGeometry().unwrap();
        let sink = geometry.Open().unwrap();
        sink.BeginFigure(D2D_POINT_2F{
            x: 200.0,
            y: 200.0,
        }, D2D1_FIGURE_BEGIN_FILLED);

        sink.AddArc(&D2D1_ARC_SEGMENT{
            point: D2D_POINT_2F{
                x: 300.0,
                y: 300.0 + 50.0,
            },
            size: D2D_SIZE_F{
                width: 100.0,
                height: 100.0
            },
            // rotationAngle: 0.0,
            // sweepDirection: Default::default(),
            // arcSize: Default::default(),
            ..Default::default()
        });


        sink.EndFigure(D2D1_FIGURE_END_OPEN);

        sink.Close().unwrap();
        geometry
    };

    graphic.draw_and_save((640, 480), w!("output.png"), |ctx| unsafe {
        ctx.Clear(Some(&D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }));

        let green_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            g: 1.0,
            a: 1.0,
            ..Default::default()
        }, None).unwrap();

        // let red_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
        //     r: 1.0,
        //     a: 1.0,
        //     ..Default::default()
        // }, None).unwrap();

        let blue_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            b: 1.0,
            a: 1.0,
            ..Default::default()
        }, None).unwrap();

        ctx.DrawEllipse(&D2D1_ELLIPSE{
            point: D2D_POINT_2F{
                x: 300.0 - 14.0,
                y: 200.0 + 51.0,
            },
            radiusX: 100.0,
            radiusY: 100.0,
        }, &blue_brush, 10.0, None);
        ctx.DrawGeometry(&geometry, &green_brush, 10.0, None);
        // ctx.FillGeometry(&geometry, &red_brush, None);
    });
}