use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW, D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let geometry = unsafe {
        let geometry = graphic.d2d_factory.CreatePathGeometry().unwrap();
        let sink = geometry.Open().unwrap();
        sink.BeginFigure(D2D_POINT_2F{
            x: 320.0,
            y: 80.0,
        }, D2D1_FIGURE_BEGIN_FILLED);

        sink.AddLine(D2D_POINT_2F{
            x: 534.0,
            y: 400.0,
        });

        sink.AddLine(D2D_POINT_2F{
            x: 106.0,
            y: 400.0,
        });

        // sink.AddLine(D2D_POINT_2F{
        //     x: 320.0,
        //     y: 80.0,
        // });

        sink.EndFigure(D2D1_FIGURE_END_OPEN);

        sink.BeginFigure(D2D_POINT_2F{
            x: 10.0,
            y: 10.0,
        }, D2D1_FIGURE_BEGIN_HOLLOW);

        sink.AddLine(D2D_POINT_2F{
            x: 40.0,
            y: 10.0,
        });

        sink.AddLine(D2D_POINT_2F{
            x: 40.0,
            y: 40.0,
        });

        sink.AddLine(D2D_POINT_2F{
            x: 10.0,
            y: 40.0,
        });

        sink.EndFigure(D2D1_FIGURE_END_CLOSED);

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

        let red_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            r: 1.0,
            a: 1.0,
            ..Default::default()
        }, None).unwrap();

        ctx.DrawGeometry(&geometry, &green_brush, 10.0, None);
        ctx.FillGeometry(&geometry, &red_brush, None);
    });
}