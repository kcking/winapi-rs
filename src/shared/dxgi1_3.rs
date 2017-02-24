// Copyright © 2015; Dmitry Roschin
// Licensed under the MIT License <LICENSE.md>
//! Mappings for the contents of dxgi1_3.h
use shared::dxgi::{IDXGIOutput, IDXGIResource};
use shared::dxgi1_2::{
    IDXGISwapChain1, IDXGIFactory2, IDXGIFactory2Vtbl, IDXGIDevice2, IDXGIDevice2Vtbl,
    IDXGISwapChain1Vtbl, IDXGIOutput1, IDXGIOutput1Vtbl, DXGI_SWAP_CHAIN_DESC1
};
use shared::dxgiformat::{DXGI_FORMAT};
use shared::minwindef::{UINT, BOOL, FLOAT};
use shared::windef::{RECT};
use um::unknwnbase::{IUnknown, IUnknownVtbl};
use um::winnt::{HRESULT, HANDLE, LARGE_INTEGER};

ENUM!{ enum DXGI_FRAME_PRESENTATION_MODE {
    DXGI_FRAME_PRESENTATION_MODE_COMPOSED = 0,
    DXGI_FRAME_PRESENTATION_MODE_OVERLAY = 1,
    DXGI_FRAME_PRESENTATION_MODE_NONE = 2,
    DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE = 3,
}}

ENUM!{ enum DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE = 0x1,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709 = 0x2,
    DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC = 0x4,
}}

ENUM!{ enum DXGI_OVERLAY_SUPPORT_FLAG {
    DXGI_OVERLAY_SUPPORT_FLAG_DIRECT = 0x1,
    DXGI_OVERLAY_SUPPORT_FLAG_SCALING = 0x2,
}}

STRUCT!{struct DXGI_DECODE_SWAP_CHAIN_DESC {
    Flags: UINT,
}}

STRUCT!{struct DXGI_FRAME_STATISTICS_MEDIA {
    PresentCount: UINT,
    PresentRefreshCount: UINT,
    SyncRefreshCount: UINT,
    SyncQPCTime: LARGE_INTEGER,
    SyncGPUTime: LARGE_INTEGER,
    CompositionMode: DXGI_FRAME_PRESENTATION_MODE,
    ApprovedPresentDuration: UINT,
}}

STRUCT!{struct DXGI_MATRIX_3X2_F {
    _11: FLOAT,
    _12: FLOAT,
    _21: FLOAT,
    _22: FLOAT,
    _31: FLOAT,
    _32: FLOAT,
}}

RIDL!(#[uuid(0x2633066b, 0x4514, 0x4c7a, 0x8f, 0xd8, 0x12, 0xea, 0x98, 0x05, 0x9d, 0x18)]
interface IDXGIDecodeSwapChain(IDXGIDecodeSwapChainVtbl): IUnknown(IUnknownVtbl) {
    fn PresentBuffer(
        BufferToPresent: UINT, SyncInterval: UINT, Flags: UINT
    ) -> HRESULT,
    fn SetSourceRect(pRect: *const RECT) -> HRESULT,
    fn SetTargetRect(pRect: *const RECT) -> HRESULT,
    fn SetDestSize(Width: UINT, Height: UINT) -> HRESULT,
    fn GetSourceRect(pRect: *mut RECT) -> HRESULT,
    fn GetTargetRect(pRect: *mut RECT) -> HRESULT,
    fn GetDestSize(
        pWidth: *mut UINT, pHeight: *mut UINT
    ) -> HRESULT,
    fn SetColorSpace(
        ColorSpace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
    ) -> HRESULT,
    fn GetColorSpace() -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS
});

RIDL!(#[uuid(0x6007896c, 0x3244, 0x4afd, 0xbf, 0x18, 0xa6, 0xd3, 0xbe, 0xda, 0x50, 0x23)]
interface IDXGIDevice3(IDXGIDevice3Vtbl): IDXGIDevice2(IDXGIDevice2Vtbl) {
    fn Trim() -> ()
});

RIDL!(#[uuid(0x25483823, 0xcd46, 0x4c7d, 0x86, 0xca, 0x47, 0xaa, 0x95, 0xb8, 0x37, 0xbd)]
interface IDXGIFactory3(IDXGIFactory3Vtbl): IDXGIFactory2(IDXGIFactory2Vtbl) {
    fn GetCreationFlags() -> UINT
});

RIDL!(#[uuid(0x41e7d1f2, 0xa591, 0x4f7b, 0xa2, 0xe5, 0xfa, 0x9c, 0x84, 0x3e, 0x1c, 0x12)]
interface IDXGIFactoryMedia(IDXGIFactoryMediaVtbl): IUnknown(IUnknownVtbl) {
    fn CreateSwapChainForCompositionSurfaceHandle(
        pDevice: *mut IUnknown, hSurface: HANDLE,
        pDesc: *const DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut IDXGIOutput,
        ppSwapChain: *mut *mut IDXGISwapChain1
    ) -> HRESULT,
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(
        pDevice: *mut IUnknown, hSurface: HANDLE,
        pDesc: *mut DXGI_DECODE_SWAP_CHAIN_DESC, pYuvDecodeBuffers: *mut IDXGIResource,
        pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut *mut IDXGIDecodeSwapChain
    ) -> HRESULT
});

RIDL!(#[uuid(0x595e39d1, 0x2724, 0x4663, 0x99, 0xb1, 0xda, 0x96, 0x9d, 0xe2, 0x83, 0x64)]
interface IDXGIOutput2(IDXGIOutput2Vtbl): IDXGIOutput1(IDXGIOutput1Vtbl) {
    fn SupportsOverlays() -> BOOL
});

RIDL!(#[uuid(0x8a6bb301, 0x7e7e, 0x41f4, 0xa8, 0xe0, 0x5b, 0x32, 0xf7, 0xf9, 0x9b, 0x18)]
interface IDXGIOutput3(IDXGIOutput3Vtbl): IDXGIOutput2(IDXGIOutput2Vtbl) {
    fn CheckOverlaySupport(
        EnumFormat: DXGI_FORMAT, pConcernedDevice: *mut IUnknown,
        pFlags: *mut UINT
    ) -> HRESULT
});

RIDL!(#[uuid(0xa8be2ac4, 0x199f, 0x4946, 0xb3, 0x31, 0x79, 0x59, 0x9f, 0xb9, 0x8d, 0xe7)]
interface IDXGISwapChain2(IDXGISwapChain2Vtbl): IDXGISwapChain1(IDXGISwapChain1Vtbl) {
    fn SetSourceSize(Width: UINT, Height: UINT) -> HRESULT,
    fn GetSourceSize(
        pWidth: *mut UINT, pHeight: *mut UINT
    ) -> HRESULT,
    fn SetMaximumFrameLatency(MaxLatency: UINT) -> HRESULT,
    fn GetMaximumFrameLatency(pMaxLatency: *mut UINT) -> HRESULT,
    fn GetFrameLatencyWaitableObject() -> HANDLE,
    fn SetMatrixTransform(
        pMatrix: *const DXGI_MATRIX_3X2_F
    ) -> HRESULT,
    fn GetMatrixTransform(
        pMatrix: *mut DXGI_MATRIX_3X2_F
    ) -> HRESULT
});

RIDL!(#[uuid(0xdd95b90b, 0xf05f, 0x4f6a, 0xbd, 0x65, 0x25, 0xbf, 0xb2, 0x64, 0xbd, 0x84)]
interface IDXGISwapChainMedia(IDXGISwapChainMediaVtbl): IUnknown(IUnknownVtbl) {
    fn GetFrameStatisticsMedia(
        pStats: *mut DXGI_FRAME_STATISTICS_MEDIA
    ) -> HRESULT,
    fn SetPresentDuration(Duration: UINT) -> HRESULT,
    fn CheckPresentDurationSupport(
        DesiredPresentDuration: UINT, pClosestSmallerPresentDuration: *mut UINT,
        pClosestLargerPresentDuration: *mut UINT
    ) -> HRESULT
});

pub const DXGI_CREATE_FACTORY_DEBUG: UINT = 0x1;