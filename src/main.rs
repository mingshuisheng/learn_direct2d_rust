use windows::core::w;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_BEGIN_HOLLOW, D2D1_FIGURE_END_CLOSED, D2D1_FIGURE_END_OPEN, D2D1_FILL_MODE_ALTERNATE, D2D1_FILL_MODE_WINDING, D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN, D2D1_PATH_SEGMENT_FORCE_UNSTROKED, D2D1_PATH_SEGMENT_NONE, D2D_POINT_2F};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let geometry = unsafe {
        let geometry = graphic.d2d_factory.CreatePathGeometry().unwrap();
        let sink = geometry.Open().unwrap();
        // sink.SetFillMode(D2D1_FILL_MODE_ALTERNATE);
        // sink.SetFillMode(D2D1_FILL_MODE_WINDING);
        // sink.SetSegmentFlags(D2D1_PATH_SEGMENT_NONE);
        // sink.SetSegmentFlags(D2D1_PATH_SEGMENT_FORCE_UNSTROKED);
        // sink.SetSegmentFlags(D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN);

        //正方形嵌套
        {
            //--------------------外正方形--------------------
            let left_top = D2D_POINT_2F {
                x: 50.0,
                y: 140.0,
            };
            let length = 200.0;

            sink.BeginFigure(left_top, D2D1_FIGURE_BEGIN_FILLED);

            sink.AddLine(D2D_POINT_2F {
                x: left_top.x + length,
                ..left_top
            });

            sink.AddLine(D2D_POINT_2F {
                x: left_top.x + length,
                y: left_top.y + length,
            });

            sink.AddLine(D2D_POINT_2F {
                y: left_top.y + length,
                ..left_top
            });

            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            //--------------------外正方形--------------------

            //--------------------内正方形--------------------
            let left_top = D2D_POINT_2F {
                x: left_top.x + length / 4.0,
                y: left_top.y + length / 4.0,
            };

            let length = length / 2.0;

            sink.BeginFigure(left_top, D2D1_FIGURE_BEGIN_FILLED);

            sink.AddLine(D2D_POINT_2F {
                x: left_top.x + length,
                ..left_top
            });

            sink.AddLine(D2D_POINT_2F {
                x: left_top.x + length,
                y: left_top.y + length,
            });

            sink.AddLine(D2D_POINT_2F {
                y: left_top.y + length,
                ..left_top
            });
            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            //--------------------内正方形--------------------

            //--------------------三层正方形--------------------
            let left_top = D2D_POINT_2F {
                x: left_top.x + length / 4.0,
                y: left_top.y + length / 4.0,
                // y: left_top.y - 50.0,
            };

            let length = length / 2.0;

            sink.BeginFigure(left_top, D2D1_FIGURE_BEGIN_FILLED);

            sink.AddLine(D2D_POINT_2F {
                x: left_top.x + length,
                ..left_top
            });

            sink.AddLine(D2D_POINT_2F {
                x: left_top.x + length,
                y: left_top.y + length,
            });

            sink.AddLine(D2D_POINT_2F {
                y: left_top.y + length,
                ..left_top
            });
            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            //--------------------三层正方形--------------------

        }

        //三角形嵌套
        {
            //--------------------外三角形--------------------
            //上顶点
            let top = D2D_POINT_2F {
                x: 430.0,
                y: 150.0,
            };
            let length = 220.0f32;
            //三角形是依据顶点top和length计算的，并且我画的是等边三角形，所以三个角是60°，边长是length
            //假设top的坐标是(x1, y1)，假设三角形右下点是(x2, y2)
            //那么根据点斜式方程可得：y2 - y1 = tan60° * (x2 - x1)
            //根据举例公式可以得到： (y2 - y1)^2 + (x2 - x1)^2 = length^2
            //解这两个方程组，就可以得到
            //x2 = x1 ± (length / (1 + tan60° ^ 2))
            //y2 = y1 ± tan60° * (length / (1 + tan60° ^ 2))
            //至于下面代码中具体的正负符号，我只是在调试过程中测试出来的

            sink.BeginFigure(top, D2D1_FIGURE_BEGIN_FILLED);
            let tana = 120.0f32.to_radians().tan();
            let tana_square = tana * tana;
            let sqrt_one_plus_tana_square = (1.0 + tana_square).sqrt();
            let len_div_sqrt_one_plus_tana_square = -length / sqrt_one_plus_tana_square;

            sink.AddLine(D2D_POINT_2F {
                x: top.x + len_div_sqrt_one_plus_tana_square,
                y: top.y + tana * len_div_sqrt_one_plus_tana_square,
            });

            let tana = 60.0f32.to_radians().tan();
            let tana_square = tana * tana;
            let sqrt_one_plus_tana_square = (1.0 + tana_square).sqrt();
            let len_div_sqrt_one_plus_tana_square = length / sqrt_one_plus_tana_square;

            sink.AddLine(D2D_POINT_2F {
                x: top.x + len_div_sqrt_one_plus_tana_square,
                y: top.y + tana * len_div_sqrt_one_plus_tana_square,
            });

            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            //--------------------外三角形--------------------


            //--------------------内三角形--------------------
            let top = D2D_POINT_2F {
                x: top.x,
                y: top.y + length / 2.0 * 0.6,
            };
            let length = length / 2.0;

            sink.BeginFigure(top, D2D1_FIGURE_BEGIN_FILLED);
            let tana = 120.0f32.to_radians().tan();
            let tana_square = tana * tana;
            let sqrt_one_plus_tana_square = (1.0 + tana_square).sqrt();
            let len_div_sqrt_one_plus_tana_square = -length / sqrt_one_plus_tana_square;

            sink.AddLine(D2D_POINT_2F {
                x: top.x + len_div_sqrt_one_plus_tana_square,
                y: top.y + tana * len_div_sqrt_one_plus_tana_square,
            });

            let tana = 60.0f32.to_radians().tan();
            let tana_square = tana * tana;
            let sqrt_one_plus_tana_square = (1.0 + tana_square).sqrt();
            let len_div_sqrt_one_plus_tana_square = length / sqrt_one_plus_tana_square;

            sink.AddLine(D2D_POINT_2F {
                x: top.x + len_div_sqrt_one_plus_tana_square,
                y: top.y + tana * len_div_sqrt_one_plus_tana_square,
            });
            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            //--------------------内三角形--------------------


            //--------------------三层三角形--------------------
            let top = D2D_POINT_2F {
                x: top.x,
                y: top.y + length / 2.0 * 0.6,
            };
            let length = length / 2.0;

            sink.BeginFigure(top, D2D1_FIGURE_BEGIN_FILLED);
            let tana = 120.0f32.to_radians().tan();
            let tana_square = tana * tana;
            let sqrt_one_plus_tana_square = (1.0 + tana_square).sqrt();
            let len_div_sqrt_one_plus_tana_square = -length / sqrt_one_plus_tana_square;

            sink.AddLine(D2D_POINT_2F {
                x: top.x + len_div_sqrt_one_plus_tana_square,
                y: top.y + tana * len_div_sqrt_one_plus_tana_square,
            });

            let tana = 60.0f32.to_radians().tan();
            let tana_square = tana * tana;
            let sqrt_one_plus_tana_square = (1.0 + tana_square).sqrt();
            let len_div_sqrt_one_plus_tana_square = length / sqrt_one_plus_tana_square;

            sink.AddLine(D2D_POINT_2F {
                x: top.x + len_div_sqrt_one_plus_tana_square,
                y: top.y + tana * len_div_sqrt_one_plus_tana_square,
            });
            sink.EndFigure(D2D1_FIGURE_END_CLOSED);
            //--------------------三层三角形--------------------
        }

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

        ctx.FillGeometry(&geometry, &red_brush, None);
        ctx.DrawGeometry(&geometry, &green_brush, 5.0, None);
    });
}