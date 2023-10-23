use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F};
use windows::Win32::Graphics::Direct2D::{D2D1_CAP_STYLE_FLAT, D2D1_CAP_STYLE_ROUND, D2D1_CAP_STYLE_SQUARE, D2D1_CAP_STYLE_TRIANGLE, D2D1_DASH_STYLE_CUSTOM, D2D1_DASH_STYLE_DASH, D2D1_DASH_STYLE_DASH_DOT, D2D1_DASH_STYLE_DASH_DOT_DOT, D2D1_DASH_STYLE_DOT, D2D1_STROKE_STYLE_PROPERTIES};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let style = unsafe {
        graphic.d2d_factory.CreateStrokeStyle(&D2D1_STROKE_STYLE_PROPERTIES {
            // startCap: D2D1_CAP_STYLE_ROUND,
            // endCap: D2D1_CAP_STYLE_TRIANGLE,
            // dashCap: D2D1_CAP_STYLE_SQUARE,
            // lineJoin: ,
            // miterLimit: ,
            dashStyle: D2D1_DASH_STYLE_CUSTOM,
            // dashOffset: 1.0, //比例,倍数，strokeWidth * 1.5
            ..Default::default()
        }, Some(&[0.0, 2.0])).unwrap()
    };

    let style2 = unsafe {
        graphic.d2d_factory.CreateStrokeStyle(&D2D1_STROKE_STYLE_PROPERTIES {
            // startCap: ,
            // endCap: ,
            // dashCap: ,
            // lineJoin: ,
            // miterLimit: ,
            // dashStyle: D2D1_DASH_STYLE_DASH,
            // dashOffset: ,
            ..Default::default()
        }, None).unwrap()
    };

    graphic.draw_and_save((640, 480), w!("output.png"), |ctx| unsafe {
        ctx.Clear(Some(&D2D1_COLOR_F {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }));

        let start = D2D_POINT_2F {
            x: 100.0,
            y: 100.0,
        };

        let end = D2D_POINT_2F {
            x: 600.0,
            y: 100.0,
        };

        let brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }, None).unwrap();

        ctx.DrawLine(start, end, &brush, 10.0, &style);

        let start = D2D_POINT_2F {
            x: 100.0,
            y: 100.0 + 20.0,
        };

        let end = D2D_POINT_2F {
            x: 600.0,
            y: 100.0 + 20.0,
        };

        //strokeWidth * 0.5
        ctx.DrawLine(start, end, &brush, 10.0, &style2);
    });
}