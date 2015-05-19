extern crate libc;

use std::str;
use std::ffi::CStr;

mod ffi;
use ffi::*;

pub use ffi::formats::*;

pub type GBMDevice = *mut Struct_gbm_device;
pub type GBMBufferObject = *mut Struct_gbm_bo;
pub type GBMSurface = *mut Struct_gbm_surface;

pub use ffi::Union_gbm_bo_handle as GBMBufferObjectHandle;

pub use ffi::Enum_gbm_bo_format as GBMBufferObjectFormat;
//pub use ffi::GBM_BO_FORMAT_XRGB8888;
//pub use ffi::GBM_BO_FORMAT_ARGB8888;

pub use ffi::Enum_gbm_bo_flags as GBMBufferObjectFlags;
//pub use ffi::GBM_BO_USE_SCANOUT;
//pub use ffi::GBM_BO_USE_CURSOR;
//pub use ffi::GBM_BO_USE_CURSOR_64X64;
//pub use ffi::GBM_BO_USE_RENDERING;
//pub use ffi::GBM_BO_USE_WRITE;

pub fn device_get_fd(gbm: &GBMDevice) -> i32 {
    unsafe {
        gbm_device_get_fd(*gbm)
    }
}

pub fn device_get_backend_name<'a>(gbm: &GBMDevice) -> &'a str {
    //Is the memory allocator going to free the underlying C string??
    let bytes: &'a [u8] = unsafe {
        let c_str: &'a CStr = CStr::from_ptr(gbm_device_get_backend_name(*gbm));
        c_str.to_bytes()
    };
    str::from_utf8(bytes).unwrap()
}

pub fn device_is_format_supported(gbm: &GBMDevice, format: u32,
                                  usage: u32) -> bool {
    match unsafe { gbm_device_is_format_supported(*gbm, format, usage) } {
        0 => false,
        _ => true
    }
}

pub fn device_destroy(gbm: GBMDevice) -> () {
    unsafe {
        gbm_device_destroy(gbm);
    }
}

pub fn create_device(fd: i32) -> Option<GBMDevice> {
    match unsafe { gbm_create_device(fd) } as usize {
        0 => None,
        x => Some(x as GBMDevice)
    }    
}

pub fn bo_create(gbm: &GBMDevice, width: u32, height: u32,
                 format: u32, flags: u32) -> Option<GBMBufferObject> {
    assert!(*gbm as usize != 0);
    match unsafe {
        gbm_bo_create(*gbm, width, height, format, flags)
    } as usize {
        0 => None,
        x => Some(x as GBMBufferObject)
    }
}

pub fn bo_get_width(bo: &GBMBufferObject) -> u32 {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_width(*bo)
    }
}

pub fn bo_get_height(bo: &GBMBufferObject) -> u32 {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_height(*bo)
    }
}

pub fn bo_get_stride(bo: &GBMBufferObject) -> u32 {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_stride(*bo)
    }
}

pub fn bo_get_format(bo: &GBMBufferObject) -> u32 {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_format(*bo)
    }
}

pub fn bo_get_device(bo: &GBMBufferObject) -> GBMDevice {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_device(*bo)
    }
}

pub fn bo_get_handle(bo: &GBMBufferObject) -> GBMBufferObjectHandle {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_handle(*bo)
    }
}

pub fn bo_get_fd(bo: &GBMBufferObject) -> i32 {
    assert!(*bo as usize != 0);
    unsafe {
        gbm_bo_get_fd(*bo)
    }
}

pub fn bo_destroy(bo: GBMBufferObject) -> () {
    assert!(bo as usize != 0);
    unsafe{
        gbm_bo_destroy(bo);
    }
}

pub fn surface_create(gbm: &GBMDevice, width: u32, height: u32,
                      format: u32, flags: u32) -> Option<GBMSurface> {
    assert!(*gbm as usize != 0);
    match unsafe {
        gbm_surface_create(*gbm, width, height, format, flags)
    } as usize {
        0 => None,
        x => Some(x as GBMSurface)
    }
}

//The implementation of this function isn't in the mesa source with the
//rest of GBM, so I'm not sure how to make this more expressive.
pub fn surface_needs_lock_front_buffer(surface: &GBMSurface) -> i32 {
    assert!(*surface as usize != 0);
    unsafe {
        gbm_surface_needs_lock_front_buffer(*surface)
    }
}

pub fn surface_lock_front_buffer(surface: &GBMSurface) -> Option<GBMBufferObject> {
    assert!(*surface as usize != 0);
    match unsafe {
        gbm_surface_lock_front_buffer(*surface)
    } as usize {
        0 => None,
        x => Some(x as GBMBufferObject)
    }
}

pub fn surface_release_buffer(surface: &GBMSurface,
                              bo: GBMBufferObject) -> () {
    assert!(*surface as usize != 0);
    assert!(bo as usize != 0);
    unsafe {
        gbm_surface_release_buffer(*surface, bo);
    }
}

pub fn surface_has_free_buffers(surface: &GBMSurface) -> bool {
    assert!(*surface as usize != 0);
    match unsafe { gbm_surface_has_free_buffers(*surface) } {
        0 => false,
        _ => true
    }
}

pub fn surface_destroy(surface: GBMSurface) -> () {
    assert!(surface as usize != 0);
    unsafe {
        gbm_surface_destroy(surface);
    }
}
