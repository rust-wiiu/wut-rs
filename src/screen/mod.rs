//! Simple graphics library
//!
//! [...]  is much more straightforward than GX2, which makes it appealing for situations that do not require complex graphics. It can draw text and pixels (one at a time!) to both the Gamepad and TV.
//!
//! This clashes with Stdout::Console on the main screen (TV).

mod color;
mod position;

use crate::bindings as c_wut;
use crate::sync::{ResourceGuard, Rrc};
use crate::{GlobalAlloc, Layout, GLOBAL_ALLOCATOR};
use alloc::{ffi::CString, string::String};
pub use color::{Color, ColorParseError};
use core::fmt::Debug;
use core::{ffi, marker::PhantomData, slice};
use position::TextPosition;

pub(crate) static OSSCREEN: Rrc<fn(), fn()> = Rrc::new(
    || unsafe {
        c_wut::OSScreenInit();
    },
    || unsafe {
        c_wut::OSScreenShutdown();
    },
);

pub struct TV;
pub struct DRC;

pub trait DisplayType {
    fn id() -> u32;
}

impl DisplayType for TV {
    fn id() -> u32 {
        c_wut::SCREEN_TV
    }
}

impl DisplayType for DRC {
    fn id() -> u32 {
        c_wut::SCREEN_DRC
    }
}

pub struct Screen<'a, Display: DisplayType> {
    display: PhantomData<Display>,
    buffer: FrameBuffer<'a>,
    resource: ResourceGuard<'a>,
}

impl<Display: DisplayType> Screen<'_, Display> {
    fn id(&self) -> u32 {
        Display::id()
    }

    pub fn fill(&self, color: Color) {
        unsafe {
            c_wut::OSScreenClearBufferEx(self.id(), color.into());
        }
    }

    pub fn update(&mut self) {
        self.buffer.flush();
        // FIXME: THIS CRASHES CEMU
        unsafe {
            c_wut::OSScreenFlipBuffersEx(self.id());
        }
    }

    pub fn text(&self, text: &str, position: impl Into<TextPosition>) {
        let text = String::from(text);
        let position: TextPosition = position.into();

        for (line, column, row) in position.format(&text) {
            crate::println!("\"{}\" - {} x {}", line, column, row);
            unsafe {
                c_wut::OSScreenPutFontEx(
                    self.id(),
                    column,
                    row,
                    CString::new(line).unwrap().as_c_str().as_ptr(),
                );
            }
        }
    }

    pub fn pixel(&self) {
        todo!()
    }

    /// Get underlying memory
    pub fn as_ref(&self) -> &[u8] {
        self.buffer.0.as_ref()
    }

    /// Get underlying memory
    pub fn as_mut(&mut self) -> &mut [u8] {
        self.buffer.0.as_mut()
    }
}

impl<Display: DisplayType> Drop for Screen<'_, Display> {
    fn drop(&mut self) {
        unsafe {
            c_wut::OSScreenEnableEx(self.id(), 0);
        }
    }
}

impl<'a> Screen<'a, TV> {
    pub fn tv() -> Screen<'a, TV> {
        let mut s = Screen {
            display: PhantomData,
            buffer: FrameBuffer::new(TV::id()),
            resource: OSSCREEN.acquire(),
        };
        unsafe {
            c_wut::OSScreenSetBufferEx(s.id(), s.buffer.as_mut_ptr());
            c_wut::OSScreenEnableEx(s.id(), 1);
        }
        s
    }
}

impl<'a> Screen<'a, DRC> {
    pub fn drc() -> Screen<'a, DRC> {
        crate::println!("drc - 1");
        let mut s = Screen {
            display: PhantomData,
            buffer: FrameBuffer::new(DRC::id()),
            resource: OSSCREEN.acquire(),
        };
        crate::println!("drc - 2");
        unsafe {
            c_wut::OSScreenSetBufferEx(s.id(), s.buffer.as_mut_ptr());
            c_wut::OSScreenEnableEx(s.id(), 1);
        }
        crate::println!("drc - 3");
        s
    }
}
struct FrameBuffer<'a>(&'a mut [u8]);

impl FrameBuffer<'_> {
    fn new(screen: c_wut::OSScreenID) -> Self {
        unsafe {
            let size = c_wut::OSScreenGetBufferSizeEx(screen) as usize;
            let layout = Layout::from_size_align(size, 0x100).unwrap();
            let data = GLOBAL_ALLOCATOR.alloc_zeroed(layout);

            if data.is_null() {
                panic!("Framebuffer allocation failed!");
            } else {
                Self(slice::from_raw_parts_mut(data, size))
            }
        }
    }

    // fn as_ptr(&self) -> *const ffi::c_void {
    //     self.0.as_ptr() as *const _
    // }

    fn as_mut_ptr(&mut self) -> *mut ffi::c_void {
        self.0.as_mut_ptr() as *mut _
    }

    fn flush(&mut self) {
        unsafe {
            c_wut::DCFlushRange(self.as_mut_ptr(), self.0.len() as u32);
        }
    }
}

impl Drop for FrameBuffer<'_> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.0.len(), 0x100).unwrap();
            GLOBAL_ALLOCATOR.dealloc(self.0.as_mut_ptr(), layout);
        }
    }
}
