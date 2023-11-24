use windows::core::w;
use windows::Win32::Graphics::Direct2D::{D2D1_EXTEND_MODE_CLAMP, D2D1_GAMMA_2_2, D2D1_GRADIENT_STOP, D2D1_IMAGE_BRUSH_PROPERTIES, D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR, D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES, D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES, ID2D1Bitmap1, ID2D1DeviceContext};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F};

mod d2d;
mod graphic;

fn main() {
    d2d::init_com();
    let graphic = graphic::Graphic::new();

    let image_bitmap = graphic.load_image(w!("1.png"));

    let width = 640;
    let height = 480;

    graphic.draw_and_save((width, height), w!("output.png"), |ctx| unsafe {
        ctx.Clear(None);

        solid_brush(ctx, width, height);

        linear_brush(ctx, width, height);

        radia_brush(ctx, width, height);

        bitmap_brush(ctx, &image_bitmap, width, height);
    });
}

//纯色笔刷
unsafe fn solid_brush(ctx: &ID2D1DeviceContext, width: u32, height: u32) {
    let solid_brush = ctx.CreateSolidColorBrush(&D2D1_COLOR_F {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    }, None).unwrap();

    let rect = D2D_RECT_F {
        left: 0.0,
        top: 0.0,
        right: width as f32 / 2.0,
        bottom: height as f32 / 2.0,
    };
    ctx.FillRectangle(&rect, &solid_brush);
}

//线性渐变笔刷
unsafe fn linear_brush(ctx: &ID2D1DeviceContext, width: u32, height: u32) {
    let stop0 = D2D1_GRADIENT_STOP {
        position: 0.0,
        color: from_str("#f00"),
    };

    let stop1 = D2D1_GRADIENT_STOP {
        position: 1.0,
        color: from_str("#0f0"),
    };

    let collection = ctx.CreateGradientStopCollection(&[
        stop0,
        stop1
    ], D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP).unwrap();

    let linear_brush = ctx.CreateLinearGradientBrush(&D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
        startPoint: D2D_POINT_2F {
            x: 0.0,
            y: 0.0,
        },
        endPoint: D2D_POINT_2F {
            x: 0.0,
            y: height as f32 / 2.0,
        },
    }, None, Some(&collection)).unwrap();

    let rect = D2D_RECT_F {
        left: width as f32 / 2.0,
        top: 0.0,
        right: width as f32,
        bottom: height as f32 / 2.0,
    };
    ctx.FillRectangle(&rect, &linear_brush);
    // ctx.DrawRectangle(&D2D_RECT_F {
    //     left: rect.left + 50.0,
    //     top: rect.top + 50.0,
    //     right: rect.right - 50.0,
    //     bottom: rect.bottom - 50.0,
    // }, &linear_brush, 10.0, None);
}

//径向渐变笔刷
unsafe fn radia_brush(ctx: &ID2D1DeviceContext, width: u32, height: u32) {
    let stop0 = D2D1_GRADIENT_STOP {
        position: 0.0,
        color: from_str("#0f0"),
    };

    let stop1 = D2D1_GRADIENT_STOP {
        position: 1.0,
        color: from_str("#00f"),
    };

    let collection = ctx.CreateGradientStopCollection(&[
        stop0,
        stop1
    ], D2D1_GAMMA_2_2, D2D1_EXTEND_MODE_CLAMP).unwrap();

    let min_length = height.min(width) as f32 / 2.0;

    let radia_brush = ctx.CreateRadialGradientBrush(&D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
        center: D2D_POINT_2F {
            x: width as f32 * 1.0 / 4.0,
            y: height as f32 * 3.0 / 4.0,
        },
        gradientOriginOffset: D2D_POINT_2F {
            x: 0.0,
            y: 0.0,
        },
        radiusX: min_length,
        radiusY: min_length,
    }, None, Some(&collection)).unwrap();

    let rect = D2D_RECT_F {
        left: 0.0,
        top: height as f32 / 2.0,
        right: width as f32 / 2.0,
        bottom: height as f32,
    };
    ctx.FillRectangle(&rect, &radia_brush);

    // ctx.DrawRectangle(&D2D_RECT_F {
    //     left: rect.left + 50.0,
    //     top: rect.top + 50.0,
    //     right: rect.right - 50.0,
    //     bottom: rect.bottom - 50.0,
    // }, &radia_brush, 10.0, None);
}

//位图笔刷
unsafe fn bitmap_brush(ctx: &ID2D1DeviceContext, image_bitmap: &ID2D1Bitmap1, width: u32, height: u32) {
    let rect = D2D_RECT_F {
        left: width as f32 / 2.0,
        top: height as f32 / 2.0,
        right: width as f32,
        bottom: height as f32,
    };

    // let rect = D2D_RECT_F {
    //     left: 0.0,
    //     top: 0.0,
    //     right: width as f32,
    //     bottom: height as f32,
    // };

    // let bitmap_brush = ctx.CreateBitmapBrush(image_bitmap, None, None).unwrap();
    // let bitmap_brush = ctx.CreateBitmapBrush2(image_bitmap, None, None).unwrap();

    let bitmap_size = image_bitmap.GetSize();

    let bitmap_brush = ctx.CreateImageBrush(image_bitmap, &D2D1_IMAGE_BRUSH_PROPERTIES {
        sourceRectangle: D2D_RECT_F {
            left: 0.0,
            top: 0.0,
            right: bitmap_size.width,
            bottom: bitmap_size.height,
        },
        extendModeX: D2D1_EXTEND_MODE_CLAMP,
        extendModeY: D2D1_EXTEND_MODE_CLAMP,
        interpolationMode: D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    }, None).unwrap();

    // 将图片放大两倍
    // let matrix = Matrix3x2::identity() * 2.0;
    // bitmap_brush.SetTransform(&matrix);

    ctx.FillRectangle(&rect, &bitmap_brush);

    // ctx.DrawRectangle(&D2D_RECT_F {
    //     left: rect.left + 50.0,
    //     top: rect.top + 50.0,
    //     right: rect.right - 50.0,
    //     bottom: rect.bottom - 50.0,
    // }, &bitmap_brush, 10.0, None);
}

fn from_str(s: &str) -> D2D1_COLOR_F {
    let s = s.trim_start_matches('#');
    let (r, g, b, a) = {
        if s.len() == 3 {
            (to_u8(&s[0..1].repeat(2)), to_u8(&s[1..2].repeat(2)), to_u8(&s[2..3].repeat(2)), 255)
        } else if s.len() == 4 {
            (to_u8(&s[0..1].repeat(2)), to_u8(&s[1..2].repeat(2)), to_u8(&s[2..3].repeat(2)), to_u8(&s[3..4].repeat(2)))
        } else if s.len() == 6 {
            (to_u8(&s[0..2]), to_u8(&s[2..4]), to_u8(&s[4..6]), 255)
        } else if s.len() == 8 {
            (to_u8(&s[0..2]), to_u8(&s[2..4]), to_u8(&s[4..6]), to_u8(&s[6..8]))
        } else {
            (0, 0, 0, 255)
        }
    };

    D2D1_COLOR_F {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    }
}

fn to_u8(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap()
}