use windows::core::ComInterface;
use windows::Win32::Graphics::Direct2D::{D2D1_DEVICE_CONTEXT_OPTIONS_NONE, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_UNIT_MODE_DIPS, D2D1CreateFactory, ID2D1Device, ID2D1DeviceContext, ID2D1Factory1};
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP};
use windows::Win32::Graphics::Direct3D11::{D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::Graphics::Imaging::CLSID_WICImagingFactory;
use windows::Win32::Graphics::Imaging::D2D::IWICImagingFactory2;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance, COINIT_MULTITHREADED, CoInitializeEx};

pub fn init_com() {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
    }
}

pub fn create_d2d_factory() -> ID2D1Factory1 {
    unsafe {
        D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).unwrap()
    }
}

pub fn create_wic_factory() -> IWICImagingFactory2 {
    unsafe {
        CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER).unwrap()
    }
}

fn create_d3d_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Option<ID3D11Device> {
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

pub fn create_d3d_device() -> ID3D11Device {
    let result = create_d3d_device_with_type(D3D_DRIVER_TYPE_HARDWARE);
    result.or_else(|| create_d3d_device_with_type(D3D_DRIVER_TYPE_WARP)).unwrap()
}

pub fn create_d2d_device(factory: &ID2D1Factory1, device: &ID3D11Device) -> ID2D1Device {
    unsafe {
        factory.CreateDevice(&device.cast::<IDXGIDevice>().unwrap()).unwrap()
    }
}

pub fn create_device_context(d2d_device: &ID2D1Device) -> ID2D1DeviceContext {
    unsafe {

        let device_context = d2d_device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE).unwrap();

        device_context.SetUnitMode(D2D1_UNIT_MODE_DIPS);

        device_context
    }
}
