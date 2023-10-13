use windows::core::{ComInterface, w};
use windows::Win32::Graphics::Direct2D::{D2D1_BITMAP_OPTIONS_CANNOT_DRAW, D2D1_BITMAP_OPTIONS_TARGET, D2D1_BITMAP_PROPERTIES1, D2D1_DEVICE_CONTEXT_OPTIONS_NONE, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_UNIT_MODE_DIPS, D2D1CreateFactory, ID2D1DeviceContext, ID2D1Factory1};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoInitializeEx, STGC_DEFAULT};
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::Graphics::Imaging::{CLSID_WICImagingFactory, GUID_ContainerFormatPng, WICBitmapEncoderNoCache};
use windows::Win32::Foundation::GENERIC_WRITE;
use windows::Win32::Graphics::Direct2D::Common::{D2D1_ALPHA_MODE_IGNORE, D2D1_COLOR_F, D2D1_PIXEL_FORMAT, D2D_SIZE_U};
use windows::Win32::Graphics::Direct3D11::{D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM;
use windows::Win32::Graphics::Imaging::D2D::IWICImagingFactory2;

fn main() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
        let d2d_factory: ID2D1Factory1 = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).unwrap();
        let d3d_device = create_device();
        let device_context = create_device_context(&d2d_factory, &d3d_device);

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

        let bitmap = device_context.CreateBitmap2(
            D2D_SIZE_U {
                width: 640,
                height: 480,
            },
            None,
            0,
            &props,
        ).unwrap();

        device_context.SetTarget(&bitmap);


        device_context.BeginDraw();
        //rgba的取值范围是0-1之间
        //0-255
        //转换其实就是比如r的值是100 -> 100/255
        device_context.Clear(Some(&D2D1_COLOR_F{
            r: 1.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }));
        device_context.EndDraw(None, None).unwrap();

        let wic_factory: IWICImagingFactory2 = CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).unwrap();

        let stream = wic_factory.CreateStream().unwrap();

        stream.InitializeFromFilename(w!("output.png"), GENERIC_WRITE.0).unwrap();

        let encoder = wic_factory.CreateEncoder(&GUID_ContainerFormatPng, std::ptr::null()).unwrap();
        encoder.Initialize(&stream, WICBitmapEncoderNoCache).unwrap();
        let mut frame_encoder = None;
        encoder.CreateNewFrame(&mut frame_encoder, std::ptr::null_mut()).unwrap();
        let frame_encoder = frame_encoder.unwrap();
        frame_encoder.Initialize(None).unwrap();

        let d2d_device = device_context.GetDevice().unwrap();
        let image_encoder = wic_factory.CreateImageEncoder(&d2d_device).unwrap();
        image_encoder.WriteFrame(&bitmap, &frame_encoder, std::ptr::null()).unwrap();

        frame_encoder.Commit().unwrap();
        encoder.Commit().unwrap();
        stream.Commit(STGC_DEFAULT).unwrap();
    }
}

fn create_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Option<ID3D11Device> {
    let mut flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;

    if cfg!(debug_assertions) {
        flags |= D3D11_CREATE_DEVICE_DEBUG;
    }

    let mut device = None;

    unsafe {
        D3D11CreateDevice(
            None,
            drive_type,
            None,
            flags,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        )
            .map(|()| device.unwrap()).ok()
    }
}

fn create_device() -> ID3D11Device {
    let result = create_device_with_type(D3D_DRIVER_TYPE_HARDWARE);
    result.or_else(|| create_device_with_type(D3D_DRIVER_TYPE_WARP)).unwrap()
}

fn create_device_context(
    factory: &ID2D1Factory1,
    device: &ID3D11Device,
) -> ID2D1DeviceContext {
    unsafe {
        let d2device = factory.CreateDevice(&device.cast::<IDXGIDevice>().unwrap()).unwrap();

        let device_context = d2device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE).unwrap();

        device_context.SetUnitMode(D2D1_UNIT_MODE_DIPS);

        device_context
    }
}