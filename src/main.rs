use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW, D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F};
use windows::Win32::Graphics::Direct2D::D2D1_QUADRATIC_BEZIER_SEGMENT;

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let geometry = unsafe {
        let geometry = graphic.d2d_factory.CreatePathGeometry().unwrap();
        let sink = geometry.Open().unwrap();
        sink.BeginFigure(D2D_POINT_2F {
            x: 100.0,
            y: 400.0,
        }, D2D1_FIGURE_BEGIN_FILLED);

        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: 200.0,
                y: 300.0,
            },
            point2: D2D_POINT_2F {
                x: 100.0,
                y: 100.0,
            },
            point3: D2D_POINT_2F {
                x: 200.0,
                y: 100.0,
            },
        });

        sink.AddQuadraticBezier(&D2D1_QUADRATIC_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: 600.0,
                y: 200.0,
            },
            point2: D2D_POINT_2F {
                x: 500.0,
                y: 400.0,
            },
        });

        // sink.AddLines();
        // sink.AddBeziers();
        // sink.AddQuadraticBeziers();
        // //
        // sink.AddLine(p1);
        // sink.AddLine(p2);
        // sink.AddLine(p3);
        // // // 等价于
        // sink.AddLines(&[p1, p2, p3]);

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

        ctx.DrawGeometry(&geometry, &green_brush, 10.0, None);

        // let red_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
        //     r: 1.0,
        //     a: 1.0,
        //     ..Default::default()
        // }, None).unwrap();
        // ctx.FillGeometry(&geometry, &red_brush, None);
    });
}