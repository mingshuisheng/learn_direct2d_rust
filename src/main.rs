use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_BEZIER_SEGMENT, D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_OPEN, D2D_POINT_2F, D2D_RECT_F};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let size = (640, 480);
    let length = 240.0;

    let left_top = D2D_POINT_2F {
        x: (size.0 as f32 - length) / 2.0,
        y: (size.1 as f32 - length) / 2.0,
    };

    let geometry = unsafe {
        let geometry = graphic.d2d_factory.CreatePathGeometry().unwrap();
        let sink = geometry.Open().unwrap();

        let half_length = length / 2.0;

        let left = D2D_POINT_2F {
            x: left_top.x,
            y: left_top.y + half_length,
        };

        let top = D2D_POINT_2F {
            x: left_top.x + half_length,
            y: left_top.y,
        };

        let right = D2D_POINT_2F {
            x: left_top.x + length,
            y: left_top.y + half_length,
        };

        let bottom = D2D_POINT_2F {
            x: left_top.x + half_length,
            y: left_top.y + length,
        };

        sink.BeginFigure(left, D2D1_FIGURE_BEGIN_FILLED);

        let offset1 = 0.0;
        let offset2 = length * 0.35;

        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: left.x - offset1,
                y: left.y - offset2,
            },
            point2: D2D_POINT_2F {
                x: top.x - offset2,
                y: top.y - offset1,
            },
            point3: top,
        });

        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: top.x + offset2,
                y: top.y - offset1,
            },
            point2: D2D_POINT_2F {
                x: right.x + offset1,
                y: right.y - offset2,
            },
            point3: right,
        });

        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: right.x + offset1,
                y: right.y + offset2,
            },
            point2: D2D_POINT_2F {
                x: bottom.x + offset2,
                y: bottom.y + offset1,
            },
            point3: bottom,
        });

        sink.AddBezier(&D2D1_BEZIER_SEGMENT {
            point1: D2D_POINT_2F {
                x: bottom.x - offset2,
                y: bottom.y + offset1,
            },
            point2: D2D_POINT_2F {
                x: left.x - offset1,
                y: left.y + offset2,
            },
            point3: left,
        });

        sink.EndFigure(D2D1_FIGURE_END_OPEN);

        sink.Close().unwrap();
        geometry
    };

    graphic.draw_and_save(size, w!("output.png"), |ctx| unsafe {
        ctx.Clear(Some(&D2D1_COLOR_F {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }));

        let green_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            g: 1.0,
            a: 1.0,
            ..Default::default()
        }, None).unwrap();

        let red_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            r: 255.0 / 255.0,
            g: 105.0 / 255.0,
            b: 0.0 / 255.0,
            a: 1.0,
            ..Default::default()
        }, None).unwrap();
        //
        // ctx.FillRectangle(&D2D_RECT_F {
        //     left: left_top.x,
        //     top: left_top.y,
        //     right: left_top.x + length,
        //     bottom: left_top.y + length,
        // }, &green_brush);
        // ctx.DrawGeometry(&geometry, &green_brush, 5.0, None);
        ctx.FillGeometry(&geometry, &red_brush, None);
    });
}