use windows::core::w;
use windows::Win32::Graphics::Direct2D::{D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_RENDER_TARGET_PROPERTIES, D2D1CreateFactory, ID2D1Factory};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoInitialize, CoUninitialize};
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::Graphics::Imaging::{CLSID_WICImagingFactory, GUID_ContainerFormatPng, GUID_WICPixelFormat32bppBGR, GUID_WICPixelFormatDontCare, IWICBitmapFrameEncode, IWICImagingFactory, WICBitmapCacheOnLoad, WICBitmapEncoderNoCache};
use windows::Win32::Foundation::GENERIC_WRITE;

fn main() {
    unsafe {
        CoInitialize(None).unwrap();
        let wic_factory: IWICImagingFactory = CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).unwrap();
        let d2d_factory: ID2D1Factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).unwrap();

        let bitmap = wic_factory.CreateBitmap(640, 480, &GUID_WICPixelFormat32bppBGR, WICBitmapCacheOnLoad).unwrap();

        let rt = d2d_factory.CreateWicBitmapRenderTarget(
            &bitmap,
            &D2D1_RENDER_TARGET_PROPERTIES::default(),
        ).unwrap();


        rt.BeginDraw();
        rt.Clear(None);
        rt.EndDraw(None, None).unwrap();

        let stream = wic_factory.CreateStream().unwrap();
        let mut format = GUID_WICPixelFormatDontCare;

        stream.InitializeFromFilename(w!("output.png"), GENERIC_WRITE.0).unwrap();

        let encoder = wic_factory.CreateEncoder(&GUID_ContainerFormatPng, std::ptr::null()).unwrap();
        encoder.Initialize(&stream, WICBitmapEncoderNoCache).unwrap();
        let mut frame_encoder: Option<IWICBitmapFrameEncode> = None;
        encoder.CreateNewFrame(&mut frame_encoder, std::ptr::null_mut()).unwrap();
        let frame_encoder = frame_encoder.unwrap();

        frame_encoder.Initialize(None).unwrap();
        frame_encoder.SetSize(640, 480).unwrap();
        frame_encoder.SetPixelFormat(&mut format).unwrap();

        frame_encoder.WriteSource(&bitmap, std::ptr::null()).unwrap();
        frame_encoder.Commit().unwrap();
        encoder.Commit().unwrap();

        // CoUninitialize();
    }
}
