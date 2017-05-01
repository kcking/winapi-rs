// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Headers for user mode only
pub mod gl;
#[cfg(feature = "audioclient")] pub mod audioclient;
#[cfg(feature = "audiosessiontypes")] pub mod audiosessiontypes;
#[cfg(feature = "avrt")] pub mod avrt;
#[cfg(feature = "cfgmgr32")] pub mod cfgmgr32;
#[cfg(feature = "cguid")] pub mod cguid;
#[cfg(feature = "combaseapi")] pub mod combaseapi;
#[cfg(feature = "coml2api")] pub mod coml2api;
#[cfg(feature = "commctrl")] pub mod commctrl;
#[cfg(feature = "commdlg")] pub mod commdlg;
#[cfg(feature = "consoleapi")] pub mod consoleapi;
#[cfg(feature = "corsym")] pub mod corsym;
#[cfg(feature = "d2d1")] pub mod d2d1;
#[cfg(feature = "d2d1_1")] pub mod d2d1_1;
#[cfg(feature = "d2d1effectauthor")] pub mod d2d1effectauthor;
#[cfg(feature = "d2d1effects")] pub mod d2d1effects;
#[cfg(feature = "d2dbasetypes")] pub mod d2dbasetypes;
#[cfg(feature = "d3d10")] pub mod d3d10;
#[cfg(feature = "d3d10shader")] pub mod d3d10shader;
#[cfg(feature = "d3d11")] pub mod d3d11;
#[cfg(feature = "d3d11on12")] pub mod d3d11on12;
#[cfg(feature = "d3d11shader")] pub mod d3d11shader;
#[cfg(feature = "d3d12")] pub mod d3d12;
#[cfg(feature = "d3d12sdklayers")] pub mod d3d12sdklayers;
#[cfg(feature = "d3d12shader")] pub mod d3d12shader;
#[cfg(feature = "d3dcommon")] pub mod d3dcommon;
#[cfg(feature = "d3dcompiler")] pub mod d3dcompiler;
#[cfg(feature = "dbghelp")] pub mod dbghelp;
#[cfg(feature = "dcommon")] pub mod dcommon;
#[cfg(feature = "docobj")] pub mod docobj;
#[cfg(feature = "documenttarget")] pub mod documenttarget;
#[cfg(feature = "dpapi")] pub mod dpapi;
#[cfg(feature = "dsgetdc")] pub mod dsgetdc;
#[cfg(feature = "dsound")] pub mod dsound;
#[cfg(feature = "dsrole")] pub mod dsrole;
#[cfg(feature = "dwmapi")] pub mod dwmapi;
#[cfg(feature = "dwrite")] pub mod dwrite;
#[cfg(feature = "dwrite_1")] pub mod dwrite_1;
#[cfg(feature = "dwrite_2")] pub mod dwrite_2;
#[cfg(feature = "dwrite_3")] pub mod dwrite_3;
#[cfg(feature = "errhandlingapi")] pub mod errhandlingapi;
#[cfg(feature = "fileapi")] pub mod fileapi;
#[cfg(feature = "handleapi")] pub mod handleapi;
#[cfg(feature = "heapapi")] pub mod heapapi;
#[cfg(feature = "http")] pub mod http;
#[cfg(feature = "imm")] pub mod imm;
#[cfg(feature = "libloaderapi")] pub mod libloaderapi;
#[cfg(feature = "lmaccess")] pub mod lmaccess;
#[cfg(feature = "lmdfs")] pub mod lmdfs;
#[cfg(feature = "lmerrlog")] pub mod lmerrlog;
#[cfg(feature = "lmjoin")] pub mod lmjoin;
#[cfg(feature = "lsalookup")] pub mod lsalookup;
#[cfg(feature = "memoryapi")] pub mod memoryapi;
#[cfg(feature = "minschannel")] pub mod minschannel;
#[cfg(feature = "minwinbase")] pub mod minwinbase;
#[cfg(feature = "mmdeviceapi")] pub mod mmdeviceapi;
#[cfg(feature = "mmsystem")] pub mod mmsystem;
#[cfg(feature = "mscat")] pub mod mscat;
#[cfg(feature = "mssip")] pub mod mssip;
#[cfg(feature = "ncrypt")] pub mod ncrypt;
#[cfg(feature = "oaidl")] pub mod oaidl;
#[cfg(feature = "objbase")] pub mod objbase;
#[cfg(feature = "objidl")] pub mod objidl;
#[cfg(feature = "objidlbase")] pub mod objidlbase;
#[cfg(feature = "ocidl")] pub mod ocidl;
#[cfg(feature = "oleauto")] pub mod oleauto;
#[cfg(feature = "pdh")] pub mod pdh;
#[cfg(feature = "processsnapshot")] pub mod processsnapshot;
#[cfg(feature = "processthreadsapi")] pub mod processthreadsapi;
#[cfg(feature = "propidl")] pub mod propidl;
#[cfg(feature = "propkeydef")] pub mod propkeydef;
#[cfg(feature = "propsys")] pub mod propsys;
#[cfg(feature = "prsht")] pub mod prsht;
#[cfg(feature = "psapi")] pub mod psapi;
#[cfg(feature = "setupapi")] pub mod setupapi;
#[cfg(feature = "shellapi")] pub mod shellapi;
#[cfg(feature = "spapidef")] pub mod spapidef;
#[cfg(feature = "strmif")] pub mod strmif;
#[cfg(feature = "synchapi")] pub mod synchapi;
#[cfg(feature = "unknwnbase")] pub mod unknwnbase;
#[cfg(feature = "urlhist")] pub mod urlhist;
#[cfg(feature = "usbspec")] pub mod usbspec;
#[cfg(feature = "vsserror")] pub mod vsserror;
#[cfg(feature = "winbase")] pub mod winbase;
#[cfg(feature = "wincred")] pub mod wincred;
#[cfg(feature = "wincodec")] pub mod wincodec;
#[cfg(feature = "wincon")] pub mod wincon;
#[cfg(feature = "wincrypt")] pub mod wincrypt;
#[cfg(feature = "winevt")] pub mod winevt;
#[cfg(feature = "wingdi")] pub mod wingdi;
#[cfg(feature = "wininet")] pub mod wininet;
#[cfg(feature = "winineti")] pub mod winineti;
#[cfg(feature = "winioctl")] pub mod winioctl;
#[cfg(feature = "winnt")] pub mod winnt;
#[cfg(feature = "winreg")] pub mod winreg;
#[cfg(feature = "winscard")] pub mod winscard;
#[cfg(feature = "winsock2")] pub mod winsock2;
#[cfg(feature = "winusb")] pub mod winusb;
#[cfg(feature = "winusbio")] pub mod winusbio;
#[cfg(feature = "winuser")] pub mod winuser;
#[cfg(feature = "xinput")] pub mod xinput;
