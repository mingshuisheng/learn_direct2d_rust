use windows::Win32::Graphics::Direct2D::{D2D1_BITMAP_OPTIONS_CANNOT_DRAW, D2D1_BITMAP_OPTIONS_TARGET, D2D1_BITMAP_PROPERTIES1, ID2D1Bitmap1, ID2D1Device, ID2D1DeviceContext, ID2D1Factory1};
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use windows::Win32::Graphics::Imaging::D2D::IWICImagingFactory2;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE};
use windows::Win32::Graphics::Direct2D::Common::{D2D1_ALPHA_MODE_IGNORE, D2D1_PIXEL_FORMAT, D2D_SIZE_U};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Imaging::{GUID_ContainerFormatPng, GUID_WICPixelFormat32bppPBGRA, WICBitmapDitherTypeNone, WICBitmapEncoderNoCache, WICBitmapPaletteTypeMedianCut, WICDecodeMetadataCacheOnLoad};
use windows::Win32::System::Com::STGC_DEFAULT;
use crate::d2d;

pub struct Graphic {
    d2d_factory: ID2D1Factory1,
    wic_factory: IWICImagingFactory2,
    d3d_device: ID3D11Device,
    d2d_device: ID2D1Device,
    device_context: ID2D1DeviceContext,
}

impl Graphic {
    pub fn new() -> Self {
        let d2d_factory: ID2D1Factory1 = d2d::create_d2d_factory();
        let wic_factory: IWICImagingFactory2 = d2d::create_wic_factory();
        let d3d_device = d2d::create_d3d_device();
        let d2d_device = d2d::create_d2d_device(&d2d_factory, &d3d_device);
        let device_context = d2d::create_device_context(&d2d_device);

        Self {
            d2d_factory,
            wic_factory,
            d3d_device,
            d2d_device,
            device_context,
        }
    }

    pub fn draw_and_save(&self, size: (u32, u32), filename: PCWSTR, draw: impl FnOnce(&ID2D1DeviceContext)) {
        //1.创建bitmap
        let bitmap = self.create_bitmap(size);
        //2.绘制bitmap
        self.draw_bitmap(&bitmap, draw);
        //3.保存bitmap
        self.save_bitmap(&bitmap, filename);
    }
    fn create_bitmap(&self, size: (u32, u32)) -> ID2D1Bitmap1 {
        let props = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_IGNORE,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
            ..Default::default()
        };

        unsafe {
            self.device_context.CreateBitmap2(
                D2D_SIZE_U {
                    width: size.0,
                    height: size.1,
                },
                None,
                0,
                &props,
            ).unwrap()
        }
    }
    fn draw_bitmap(&self, bitmap: &ID2D1Bitmap1, draw: impl FnOnce(&ID2D1DeviceContext)) {
        unsafe {
            //1.将这个bitmap作为渲染目标
            self.device_context.SetTarget(bitmap);
            //2.开始绘图
            self.device_context.BeginDraw();
            //3.执行draw函数
            draw(&self.device_context);
            //4.结束绘图
            self.device_context.EndDraw(None, None).unwrap();
        }
    }
    fn save_bitmap(&self, bitmap: &ID2D1Bitmap1, filename: PCWSTR) {
        unsafe {
            let stream = self.wic_factory.CreateStream().unwrap();

            stream.InitializeFromFilename(filename, GENERIC_WRITE.0).unwrap();

            let encoder = self.wic_factory.CreateEncoder(&GUID_ContainerFormatPng, std::ptr::null()).unwrap();
            encoder.Initialize(&stream, WICBitmapEncoderNoCache).unwrap();
            let mut frame_encoder = None;
            encoder.CreateNewFrame(&mut frame_encoder, std::ptr::null_mut()).unwrap();
            let frame_encoder = frame_encoder.unwrap();
            frame_encoder.Initialize(None).unwrap();

            let image_encoder = self.wic_factory.CreateImageEncoder(&self.d2d_device).unwrap();
            image_encoder.WriteFrame(bitmap, &frame_encoder, std::ptr::null()).unwrap();

            frame_encoder.Commit().unwrap();
            encoder.Commit().unwrap();
            stream.Commit(STGC_DEFAULT).unwrap();
        }
    }

    pub fn load_image(&self,  filename: PCWSTR) -> ID2D1Bitmap1 {
        unsafe {
            let decoder = self.wic_factory.CreateDecoderFromFilename(filename, None, GENERIC_READ, WICDecodeMetadataCacheOnLoad).unwrap();
            let frame = decoder.GetFrame(0).unwrap();
            let converter = self.wic_factory.CreateFormatConverter().unwrap();
            converter.Initialize(&frame, &GUID_WICPixelFormat32bppPBGRA, WICBitmapDitherTypeNone, None, 0.0, WICBitmapPaletteTypeMedianCut).unwrap();
            self.device_context.CreateBitmapFromWicBitmap2(&converter, None).unwrap()
        }
    }
}