/* automatically generated by rust-bindgen 0.70.1 */

pub const __NEWLIB_H__: u32 = 1;
pub const _NEWLIB_VERSION_H__: u32 = 1;
pub const _NEWLIB_VERSION: &[u8; 6] = b"4.4.0\0";
pub const __NEWLIB__: u32 = 4;
pub const __NEWLIB_MINOR__: u32 = 4;
pub const __NEWLIB_PATCHLEVEL__: u32 = 0;
pub const _ATEXIT_DYNAMIC_ALLOC: u32 = 1;
pub const _FSEEK_OPTIMIZATION: u32 = 1;
pub const _FVWRITE_IN_STREAMIO: u32 = 1;
pub const _HAVE_CC_INHIBIT_LOOP_TO_LIBCALL: u32 = 1;
pub const _HAVE_INITFINI_ARRAY: u32 = 1;
pub const _HAVE_LONG_DOUBLE: u32 = 1;
pub const _LDBL_EQ_DBL: u32 = 1;
pub const _MB_CAPABLE: u32 = 1;
pub const _MB_LEN_MAX: u32 = 8;
pub const _REENT_CHECK_VERIFY: u32 = 1;
pub const _UNBUF_STREAM_OPT: u32 = 1;
pub const _WANT_IO_C99_FORMATS: u32 = 1;
pub const _WANT_IO_LONG_LONG: u32 = 1;
pub const _WANT_REGISTER_FINI: u32 = 1;
pub const _WANT_USE_GDTOA: u32 = 1;
pub const _WIDE_ORIENT: u32 = 1;
pub const __OBSOLETE_MATH_DEFAULT: u32 = 1;
pub const __OBSOLETE_MATH: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __ATFILE_VISIBLE: u32 = 1;
pub const __BSD_VISIBLE: u32 = 1;
pub const __GNU_VISIBLE: u32 = 0;
pub const __ISO_C_VISIBLE: u32 = 2011;
pub const __LARGEFILE_VISIBLE: u32 = 0;
pub const __MISC_VISIBLE: u32 = 1;
pub const __POSIX_VISIBLE: u32 = 200809;
pub const __SVID_VISIBLE: u32 = 1;
pub const __XSI_VISIBLE: u32 = 0;
pub const __SSP_FORTIFY_LEVEL: u32 = 0;
pub const _POSIX_MONOTONIC_CLOCK: u32 = 200112;
pub const _POSIX_TIMERS: u32 = 1;
pub const __RAND_MAX: u32 = 2147483647;
pub const __have_longlong64: u32 = 1;
pub const __have_long32: u32 = 1;
pub const ___int8_t_defined: u32 = 1;
pub const ___int16_t_defined: u32 = 1;
pub const ___int32_t_defined: u32 = 1;
pub const ___int64_t_defined: u32 = 1;
pub const ___int_least8_t_defined: u32 = 1;
pub const ___int_least16_t_defined: u32 = 1;
pub const ___int_least32_t_defined: u32 = 1;
pub const ___int_least64_t_defined: u32 = 1;
pub const __int20: u32 = 2;
pub const __int20__: u32 = 2;
pub const __INT8: &[u8; 3] = b"hh\0";
pub const __INT16: &[u8; 2] = b"h\0";
pub const __INT64: &[u8; 3] = b"ll\0";
pub const __FAST8: &[u8; 3] = b"hh\0";
pub const __FAST16: &[u8; 2] = b"h\0";
pub const __FAST64: &[u8; 3] = b"ll\0";
pub const __LEAST8: &[u8; 3] = b"hh\0";
pub const __LEAST16: &[u8; 2] = b"h\0";
pub const __LEAST64: &[u8; 3] = b"ll\0";
pub const __int8_t_defined: u32 = 1;
pub const __int16_t_defined: u32 = 1;
pub const __int32_t_defined: u32 = 1;
pub const __int64_t_defined: u32 = 1;
pub const __int_least8_t_defined: u32 = 1;
pub const __int_least16_t_defined: u32 = 1;
pub const __int_least32_t_defined: u32 = 1;
pub const __int_least64_t_defined: u32 = 1;
pub const __int_fast8_t_defined: u32 = 1;
pub const __int_fast16_t_defined: u32 = 1;
pub const __int_fast32_t_defined: u32 = 1;
pub const __int_fast64_t_defined: u32 = 1;
pub const WINT_MIN: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __alignas_is_defined: u32 = 1;
pub const __alignof_is_defined: u32 = 1;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const WHB_SERVER_BUFFER_SIZE: u32 = 1024;
extern "C" {
    pub fn __assert(
        arg1: *const ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: *const ::core::ffi::c_char,
    ) -> !;
}
extern "C" {
    pub fn __assert_func(
        arg1: *const ::core::ffi::c_char,
        arg2: ::core::ffi::c_int,
        arg3: *const ::core::ffi::c_char,
        arg4: *const ::core::ffi::c_char,
    ) -> !;
}
pub type wchar_t = ::core::ffi::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::core::ffi::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of max_align_t"][::core::mem::size_of::<max_align_t>() - 32usize];
    ["Alignment of max_align_t"][::core::mem::align_of::<max_align_t>() - 16usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce1"]
        [::core::mem::offset_of!(max_align_t, __clang_max_align_nonce1) - 0usize];
    ["Offset of field: max_align_t::__clang_max_align_nonce2"]
        [::core::mem::offset_of!(max_align_t, __clang_max_align_nonce2) - 16usize];
};
pub type __int8_t = ::core::ffi::c_schar;
pub type __uint8_t = ::core::ffi::c_uchar;
pub type __int16_t = ::core::ffi::c_short;
pub type __uint16_t = ::core::ffi::c_ushort;
pub type __int32_t = ::core::ffi::c_int;
pub type __uint32_t = ::core::ffi::c_uint;
pub type __int64_t = ::core::ffi::c_longlong;
pub type __uint64_t = ::core::ffi::c_ulonglong;
pub type __int_least8_t = ::core::ffi::c_schar;
pub type __uint_least8_t = ::core::ffi::c_uchar;
pub type __int_least16_t = ::core::ffi::c_short;
pub type __uint_least16_t = ::core::ffi::c_ushort;
pub type __int_least32_t = ::core::ffi::c_int;
pub type __uint_least32_t = ::core::ffi::c_uint;
pub type __int_least64_t = ::core::ffi::c_longlong;
pub type __uint_least64_t = ::core::ffi::c_ulonglong;
pub type __intmax_t = ::core::ffi::c_longlong;
pub type __uintmax_t = ::core::ffi::c_ulonglong;
pub type __intptr_t = ::core::ffi::c_long;
pub type __uintptr_t = ::core::ffi::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type int_least8_t = __int_least8_t;
pub type uint_least8_t = __uint_least8_t;
pub type int_least16_t = __int_least16_t;
pub type uint_least16_t = __uint_least16_t;
pub type int_least32_t = __int_least32_t;
pub type uint_least32_t = __uint_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::core::ffi::c_schar;
pub type uint_fast8_t = ::core::ffi::c_uchar;
pub type int_fast16_t = ::core::ffi::c_short;
pub type uint_fast16_t = ::core::ffi::c_ushort;
pub type int_fast32_t = ::core::ffi::c_int;
pub type uint_fast32_t = ::core::ffi::c_uint;
pub type int_fast64_t = ::core::ffi::c_longlong;
pub type uint_fast64_t = ::core::ffi::c_ulonglong;
pub type BOOL = i32;
extern "C" {
    pub fn WHBProcInit();
}
extern "C" {
    pub fn WHBProcShutdown();
}
extern "C" {
    pub fn WHBProcStopRunning();
}
extern "C" {
    pub fn WHBProcIsRunning() -> BOOL;
}
pub type LogHandlerFn =
    ::core::option::Option<unsafe extern "C" fn(msg: *const ::core::ffi::c_char)>;
extern "C" {
    pub fn WHBAddLogHandler(fn_: LogHandlerFn) -> BOOL;
}
extern "C" {
    pub fn WHBRemoveLogHandler(fn_: LogHandlerFn) -> BOOL;
}
extern "C" {
    pub fn WHBLogWrite(str_: *const ::core::ffi::c_char) -> BOOL;
}
extern "C" {
    pub fn WHBLogPrint(str_: *const ::core::ffi::c_char) -> BOOL;
}
extern "C" {
    pub fn WHBLogWritef(fmt: *const ::core::ffi::c_char, ...) -> BOOL;
}
extern "C" {
    pub fn WHBLogPrintf(fmt: *const ::core::ffi::c_char, ...) -> BOOL;
}
extern "C" {
    pub fn WHBLogConsoleInit() -> BOOL;
}
extern "C" {
    pub fn WHBLogConsoleFree();
}
extern "C" {
    pub fn WHBLogConsoleSetColor(color: u32);
}
extern "C" {
    pub fn WHBLogConsoleDraw();
}
pub type DisassemblyPrintFn =
    ::core::option::Option<unsafe extern "C" fn(fmt: *const ::core::ffi::c_char, ...)>;
pub type DisassemblyFindSymbolFn = ::core::option::Option<
    unsafe extern "C" fn(
        addr: u32,
        symbolNameBuf: *mut ::core::ffi::c_char,
        symbolNameBufSize: u32,
    ) -> u32,
>;
pub const DisassemblePPCFlags_DISASSEMBLE_PPC_FLAGS_NONE: DisassemblePPCFlags = 0;
pub type DisassemblePPCFlags = ::core::ffi::c_uint;
extern "C" {
    pub fn OSConsoleWrite(msg: *const ::core::ffi::c_char, size: u32);
}
extern "C" {
    pub fn __OSConsoleWrite(msg: *const ::core::ffi::c_char, size: u32);
}
extern "C" {
    pub fn OSReport(fmt: *const ::core::ffi::c_char, ...);
}
extern "C" {
    pub fn OSReportVerbose(fmt: *const ::core::ffi::c_char, ...);
}
extern "C" {
    pub fn OSReportInfo(fmt: *const ::core::ffi::c_char, ...);
}
extern "C" {
    pub fn OSReportWarn(fmt: *const ::core::ffi::c_char, ...);
}
extern "C" {
    pub fn OSPanic(
        file: *const ::core::ffi::c_char,
        line: u32,
        fmt: *const ::core::ffi::c_char,
        ...
    );
}
extern "C" {
    pub fn OSFatal(msg: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn OSGetSymbolName(
        addr: u32,
        symbolNameBuf: *mut ::core::ffi::c_char,
        symbolNameBufSize: u32,
    ) -> u32;
}
extern "C" {
    pub fn OSGetUPID() -> u32;
}
extern "C" {
    pub fn OSIsDebuggerInitialized() -> BOOL;
}
extern "C" {
    pub fn OSIsDebuggerPresent() -> BOOL;
}
extern "C" {
    pub fn OSIsECOBoot() -> BOOL;
}
extern "C" {
    pub fn OSIsECOMode() -> BOOL;
}
extern "C" {
    pub fn __OSSetCrashRecovery(crashRecovery: u32);
}
extern "C" {
    pub fn __OSGetCrashRecovery() -> u32;
}
extern "C" {
    pub fn DisassemblePPCOpcode(
        opcode: *mut u32,
        buffer: *mut ::core::ffi::c_char,
        bufferSize: u32,
        findSymbolFn: DisassemblyFindSymbolFn,
        flags: DisassemblePPCFlags,
    ) -> BOOL;
}
extern "C" {
    pub fn DisassemblePPCRange(
        start: *mut ::core::ffi::c_void,
        end: *mut ::core::ffi::c_void,
        printFn: DisassemblyPrintFn,
        findSymbolFn: DisassemblyFindSymbolFn,
        flags: DisassemblePPCFlags,
    );
}
