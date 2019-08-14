//! This module declares the Rust bindings to the `wasi_unstable` API.
//!
//! The raw bindings are in the `raw` submodule. They use raw pointers and
//! are unsafe. In the the top-level module, raw pointer-length pairs are
//! replaced by Rust slice types, output parameters are converted to normal
//! return values, names are translated to be more Rust-idiomatic, and the
//! functions are safe.
//!
//! TODO: Not all functions are covered yet; implement the rest of the API.

pub mod raw;

use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::num::NonZeroU16;
use raw::*;

pub type Advice = __wasi_advice_t;
pub type ClockId = __wasi_clockid_t;
pub type Device = __wasi_device_t;
pub type DirCookie = __wasi_dircookie_t;
pub type Errno = NonZeroU16;
pub type EventRwFlags = __wasi_eventrwflags_t;
pub type EventType = __wasi_eventtype_t;
pub type ExitCode = __wasi_exitcode_t;
pub type Fd = __wasi_fd_t;
pub type FdFlags = __wasi_fdflags_t;
pub type FileDelta = __wasi_filedelta_t;
pub type FileSize = __wasi_filesize_t;
pub type FileType = __wasi_filetype_t;
pub type FstFlags = __wasi_fstflags_t;
pub type Inode = __wasi_inode_t;
pub type LinkCount = __wasi_linkcount_t;
pub type LookupFlags = __wasi_lookupflags_t;
pub type OFlags = __wasi_oflags_t;
pub type PreopenType = __wasi_preopentype_t;
pub type RiFlags = __wasi_riflags_t;
pub type Rights = __wasi_rights_t;
pub type RoFlags = __wasi_roflags_t;
pub type SdFlags = __wasi_sdflags_t;
pub type SiFlags = __wasi_siflags_t;
pub type Signal = __wasi_signal_t;
pub type SubclockFlags = __wasi_subclockflags_t;
pub type Timestamp = __wasi_timestamp_t;
pub type Userdata = __wasi_userdata_t;
pub type Whence = __wasi_whence_t;
pub type Dirent = __wasi_dirent_t;
pub type FdStat = __wasi_fdstat_t;
pub type FileStat = __wasi_filestat_t;
pub type CIoVec = __wasi_ciovec_t;
pub type IoVec = __wasi_iovec_t;
pub type Subscription = __wasi_subscription_t;
pub type Event = __wasi_event_t;
pub type Prestat = __wasi_prestat_t;

pub const ADVICE_NORMAL: Advice = __WASI_ADVICE_NORMAL;
pub const ADVICE_SEQUENTIAL: Advice = __WASI_ADVICE_SEQUENTIAL;
pub const ADVICE_RANDOM: Advice = __WASI_ADVICE_RANDOM;
pub const ADVICE_WILLNEED: Advice = __WASI_ADVICE_WILLNEED;
pub const ADVICE_DONTNEED: Advice = __WASI_ADVICE_DONTNEED;
pub const ADVICE_NOREUSE: Advice = __WASI_ADVICE_NOREUSE;
pub const CLOCK_REALTIME: ClockId = __WASI_CLOCK_REALTIME;
pub const CLOCK_MONOTONIC: ClockId = __WASI_CLOCK_MONOTONIC;
pub const CLOCK_PROCESS_CPUTIME_ID: ClockId = __WASI_CLOCK_PROCESS_CPUTIME_ID;
pub const CLOCK_THREAD_CPUTIME_ID: ClockId = __WASI_CLOCK_THREAD_CPUTIME_ID;
pub const DIRCOOKIE_START: DirCookie = __WASI_DIRCOOKIE_START;

// A hucky wasy to assert that `__WASI_ESUCCESS` equals to 0
#[deny(const_err)]
const _ASSERT: u16 =  0 - __WASI_ESUCCESS;

macro_rules! errno_set {
    ($($safe_const:ident => $raw_const:ident,)*) => {
        $(
            pub const $safe_const: Errno = unsafe {
                NonZeroU16::new_unchecked($raw_const)
            };
        )*
    };
}

errno_set!(
    E2BIG => __WASI_E2BIG,
    EACCES => __WASI_EACCES,
    EADDRINUSE => __WASI_EADDRINUSE,
    EADDRNOTAVAIL => __WASI_EADDRNOTAVAIL,
    EAFNOSUPPORT => __WASI_EAFNOSUPPORT,
    EAGAIN => __WASI_EAGAIN,
    EALREADY => __WASI_EALREADY,
    EBADF => __WASI_EBADF,
    EBADMSG => __WASI_EBADMSG,
    EBUSY => __WASI_EBUSY,
    ECANCELED => __WASI_ECANCELED,
    ECHILD => __WASI_ECHILD,
    ECONNABORTED => __WASI_ECONNABORTED,
    ECONNREFUSED => __WASI_ECONNREFUSED,
    ECONNRESET => __WASI_ECONNRESET,
    EDEADLK => __WASI_EDEADLK,
    EDESTADDRREQ => __WASI_EDESTADDRREQ,
    EDOM => __WASI_EDOM,
    EDQUOT => __WASI_EDQUOT,
    EEXIST => __WASI_EEXIST,
    EFAULT => __WASI_EFAULT,
    EFBIG => __WASI_EFBIG,
    EHOSTUNREACH => __WASI_EHOSTUNREACH,
    EIDRM => __WASI_EIDRM,
    EILSEQ => __WASI_EILSEQ,
    EINPROGRESS => __WASI_EINPROGRESS,
    EINTR => __WASI_EINTR,
    EINVAL => __WASI_EINVAL,
    EIO => __WASI_EIO,
    EISCONN => __WASI_EISCONN,
    EISDIR => __WASI_EISDIR,
    ELOOP => __WASI_ELOOP,
    EMFILE => __WASI_EMFILE,
    EMLINK => __WASI_EMLINK,
    EMSGSIZE => __WASI_EMSGSIZE,
    EMULTIHOP => __WASI_EMULTIHOP,
    ENAMETOOLONG => __WASI_ENAMETOOLONG,
    ENETDOWN => __WASI_ENETDOWN,
    ENETRESET => __WASI_ENETRESET,
    ENETUNREACH => __WASI_ENETUNREACH,
    ENFILE => __WASI_ENFILE,
    ENOBUFS => __WASI_ENOBUFS,
    ENODEV => __WASI_ENODEV,
    ENOENT => __WASI_ENOENT,
    ENOEXEC => __WASI_ENOEXEC,
    ENOLCK => __WASI_ENOLCK,
    ENOLINK => __WASI_ENOLINK,
    ENOMEM => __WASI_ENOMEM,
    ENOMSG => __WASI_ENOMSG,
    ENOPROTOOPT => __WASI_ENOPROTOOPT,
    ENOSPC => __WASI_ENOSPC,
    ENOSYS => __WASI_ENOSYS,
    ENOTCONN => __WASI_ENOTCONN,
    ENOTDIR => __WASI_ENOTDIR,
    ENOTEMPTY => __WASI_ENOTEMPTY,
    ENOTRECOVERABLE => __WASI_ENOTRECOVERABLE,
    ENOTSOCK => __WASI_ENOTSOCK,
    ENOTSUP => __WASI_ENOTSUP,
    ENOTTY => __WASI_ENOTTY,
    ENXIO => __WASI_ENXIO,
    EOVERFLOW => __WASI_EOVERFLOW,
    EOWNERDEAD => __WASI_EOWNERDEAD,
    EPERM => __WASI_EPERM,
    EPIPE => __WASI_EPIPE,
    EPROTO => __WASI_EPROTO,
    EPROTONOSUPPORT => __WASI_EPROTONOSUPPORT,
    EPROTOTYPE => __WASI_EPROTOTYPE,
    ERANGE => __WASI_ERANGE,
    EROFS => __WASI_EROFS,
    ESPIPE => __WASI_ESPIPE,
    ESRCH => __WASI_ESRCH,
    ESTALE => __WASI_ESTALE,
    ETIMEDOUT => __WASI_ETIMEDOUT,
    ETXTBSY => __WASI_ETXTBSY,
    EXDEV => __WASI_EXDEV,
    ENOTCAPABLE => __WASI_ENOTCAPABLE,
);

pub const EVENT_FD_READWRITE_HANGUP: EventRwFlags = __WASI_EVENT_FD_READWRITE_HANGUP;
pub const EVENTTYPE_CLOCK: EventType = __WASI_EVENTTYPE_CLOCK;
pub const EVENTTYPE_FD_READ: EventType = __WASI_EVENTTYPE_FD_READ;
pub const EVENTTYPE_FD_WRITE: EventType = __WASI_EVENTTYPE_FD_WRITE;
pub const FDFLAG_APPEND: FdFlags = __WASI_FDFLAG_APPEND;
pub const FDFLAG_DSYNC: FdFlags = __WASI_FDFLAG_DSYNC;
pub const FDFLAG_NONBLOCK: FdFlags = __WASI_FDFLAG_NONBLOCK;
pub const FDFLAG_RSYNC: FdFlags = __WASI_FDFLAG_RSYNC;
pub const FDFLAG_SYNC: FdFlags = __WASI_FDFLAG_SYNC;
pub const FILETYPE_UNKNOWN: FileType = __WASI_FILETYPE_UNKNOWN;
pub const FILETYPE_BLOCK_DEVICE: FileType = __WASI_FILETYPE_BLOCK_DEVICE;
pub const FILETYPE_CHARACTER_DEVICE: FileType = __WASI_FILETYPE_CHARACTER_DEVICE;
pub const FILETYPE_DIRECTORY: FileType = __WASI_FILETYPE_DIRECTORY;
pub const FILETYPE_REGULAR_FILE: FileType = __WASI_FILETYPE_REGULAR_FILE;
pub const FILETYPE_SOCKET_DGRAM: FileType = __WASI_FILETYPE_SOCKET_DGRAM;
pub const FILETYPE_SOCKET_STREAM: FileType = __WASI_FILETYPE_SOCKET_STREAM;
pub const FILETYPE_SYMBOLIC_LINK: FileType = __WASI_FILETYPE_SYMBOLIC_LINK;
pub const FILESTAT_SET_ATIM: FstFlags = __WASI_FILESTAT_SET_ATIM;
pub const FILESTAT_SET_ATIM_NOW: FstFlags = __WASI_FILESTAT_SET_ATIM_NOW;
pub const FILESTAT_SET_MTIM: FstFlags = __WASI_FILESTAT_SET_MTIM;
pub const FILESTAT_SET_MTIM_NOW: FstFlags = __WASI_FILESTAT_SET_MTIM_NOW;
pub const LOOKUP_SYMLINK_FOLLOW: LookupFlags = __WASI_LOOKUP_SYMLINK_FOLLOW;
pub const O_CREAT: OFlags = __WASI_O_CREAT;
pub const O_DIRECTORY: OFlags = __WASI_O_DIRECTORY;
pub const O_EXCL: OFlags = __WASI_O_EXCL;
pub const O_TRUNC: OFlags = __WASI_O_TRUNC;
pub const PREOPENTYPE_DIR: PreopenType = __WASI_PREOPENTYPE_DIR;
pub const SOCK_RECV_PEEK: RiFlags = __WASI_SOCK_RECV_PEEK;
pub const SOCK_RECV_WAITALL: RiFlags = __WASI_SOCK_RECV_WAITALL;
pub const RIGHT_FD_DATASYNC: Rights = __WASI_RIGHT_FD_DATASYNC;
pub const RIGHT_FD_READ: Rights = __WASI_RIGHT_FD_READ;
pub const RIGHT_FD_SEEK: Rights = __WASI_RIGHT_FD_SEEK;
pub const RIGHT_FD_FDSTAT_SET_FLAGS: Rights = __WASI_RIGHT_FD_FDSTAT_SET_FLAGS;
pub const RIGHT_FD_SYNC: Rights = __WASI_RIGHT_FD_SYNC;
pub const RIGHT_FD_TELL: Rights = __WASI_RIGHT_FD_TELL;
pub const RIGHT_FD_WRITE: Rights = __WASI_RIGHT_FD_WRITE;
pub const RIGHT_FD_ADVISE: Rights = __WASI_RIGHT_FD_ADVISE;
pub const RIGHT_FD_ALLOCATE: Rights = __WASI_RIGHT_FD_ALLOCATE;
pub const RIGHT_PATH_CREATE_DIRECTORY: Rights = __WASI_RIGHT_PATH_CREATE_DIRECTORY;
pub const RIGHT_PATH_CREATE_FILE: Rights = __WASI_RIGHT_PATH_CREATE_FILE;
pub const RIGHT_PATH_LINK_SOURCE: Rights = __WASI_RIGHT_PATH_LINK_SOURCE;
pub const RIGHT_PATH_LINK_TARGET: Rights = __WASI_RIGHT_PATH_LINK_TARGET;
pub const RIGHT_PATH_OPEN: Rights = __WASI_RIGHT_PATH_OPEN;
pub const RIGHT_FD_READDIR: Rights = __WASI_RIGHT_FD_READDIR;
pub const RIGHT_PATH_READLINK: Rights = __WASI_RIGHT_PATH_READLINK;
pub const RIGHT_PATH_RENAME_SOURCE: Rights = __WASI_RIGHT_PATH_RENAME_SOURCE;
pub const RIGHT_PATH_RENAME_TARGET: Rights = __WASI_RIGHT_PATH_RENAME_TARGET;
pub const RIGHT_PATH_FILESTAT_GET: Rights = __WASI_RIGHT_PATH_FILESTAT_GET;
pub const RIGHT_PATH_FILESTAT_SET_SIZE: Rights = __WASI_RIGHT_PATH_FILESTAT_SET_SIZE;
pub const RIGHT_PATH_FILESTAT_SET_TIMES: Rights = __WASI_RIGHT_PATH_FILESTAT_SET_TIMES;
pub const RIGHT_FD_FILESTAT_GET: Rights = __WASI_RIGHT_FD_FILESTAT_GET;
pub const RIGHT_FD_FILESTAT_SET_SIZE: Rights = __WASI_RIGHT_FD_FILESTAT_SET_SIZE;
pub const RIGHT_FD_FILESTAT_SET_TIMES: Rights = __WASI_RIGHT_FD_FILESTAT_SET_TIMES;
pub const RIGHT_PATH_SYMLINK: Rights = __WASI_RIGHT_PATH_SYMLINK;
pub const RIGHT_PATH_REMOVE_DIRECTORY: Rights = __WASI_RIGHT_PATH_REMOVE_DIRECTORY;
pub const RIGHT_PATH_UNLINK_FILE: Rights = __WASI_RIGHT_PATH_UNLINK_FILE;
pub const RIGHT_POLL_FD_READWRITE: Rights = __WASI_RIGHT_POLL_FD_READWRITE;
pub const RIGHT_SOCK_SHUTDOWN: Rights = __WASI_RIGHT_SOCK_SHUTDOWN;
pub const SOCK_RECV_DATA_TRUNCATED: RoFlags = __WASI_SOCK_RECV_DATA_TRUNCATED;
pub const SHUT_RD: SdFlags = __WASI_SHUT_RD;
pub const SHUT_WR: SdFlags = __WASI_SHUT_WR;
pub const SIGHUP: Signal = __WASI_SIGHUP;
pub const SIGINT: Signal = __WASI_SIGINT;
pub const SIGQUIT: Signal = __WASI_SIGQUIT;
pub const SIGILL: Signal = __WASI_SIGILL;
pub const SIGTRAP: Signal = __WASI_SIGTRAP;
pub const SIGABRT: Signal = __WASI_SIGABRT;
pub const SIGBUS: Signal = __WASI_SIGBUS;
pub const SIGFPE: Signal = __WASI_SIGFPE;
pub const SIGKILL: Signal = __WASI_SIGKILL;
pub const SIGUSR1: Signal = __WASI_SIGUSR1;
pub const SIGSEGV: Signal = __WASI_SIGSEGV;
pub const SIGUSR2: Signal = __WASI_SIGUSR2;
pub const SIGPIPE: Signal = __WASI_SIGPIPE;
pub const SIGALRM: Signal = __WASI_SIGALRM;
pub const SIGTERM: Signal = __WASI_SIGTERM;
pub const SIGCHLD: Signal = __WASI_SIGCHLD;
pub const SIGCONT: Signal = __WASI_SIGCONT;
pub const SIGSTOP: Signal = __WASI_SIGSTOP;
pub const SIGTSTP: Signal = __WASI_SIGTSTP;
pub const SIGTTIN: Signal = __WASI_SIGTTIN;
pub const SIGTTOU: Signal = __WASI_SIGTTOU;
pub const SIGURG: Signal = __WASI_SIGURG;
pub const SIGXCPU: Signal = __WASI_SIGXCPU;
pub const SIGXFSZ: Signal = __WASI_SIGXFSZ;
pub const SIGVTALRM: Signal = __WASI_SIGVTALRM;
pub const SIGPROF: Signal = __WASI_SIGPROF;
pub const SIGWINCH: Signal = __WASI_SIGWINCH;
pub const SIGPOLL: Signal = __WASI_SIGPOLL;
pub const SIGPWR: Signal = __WASI_SIGPWR;
pub const SIGSYS: Signal = __WASI_SIGSYS;
pub const SUBSCRIPTION_CLOCK_ABSTIME: SubclockFlags = __WASI_SUBSCRIPTION_CLOCK_ABSTIME;
pub const WHENCE_CUR: Whence = __WASI_WHENCE_CUR;
pub const WHENCE_END: Whence = __WASI_WHENCE_END;
pub const WHENCE_SET: Whence = __WASI_WHENCE_SET;

macro_rules! unsafe_wrap0 {
    {$f:expr} => {
        unsafe {
            if let Some(code) = NonZeroU16::new($f) {
                Err(code)
            } else {
                Ok(())
            }
        }
    };
}

macro_rules! unsafe_wrap1 {
    {$var:ident, $f:expr,} => {
        unsafe {
            if let Some(code) = NonZeroU16::new($f) {
                Err(code)
            } else {
                Ok($var.assume_init())
            }
        }
    };
}

pub fn clock_res_get(clock_id: ClockId) -> Result<Timestamp, Errno> {
    let mut resolution = MaybeUninit::<Timestamp>::uninit();
    unsafe_wrap1!{
        resolution,
        __wasi_clock_res_get(clock_id, resolution.as_mut_ptr()),
    }
}

pub fn clock_time_get(clock_id: ClockId, precision: Timestamp) -> Result<Timestamp, Errno> {
    let mut time = MaybeUninit::<Timestamp>::uninit();
    unsafe_wrap1!{
        time,
        __wasi_clock_time_get(clock_id, precision, time.as_mut_ptr()),
    }
}

pub fn fd_pread(fd: Fd, iovs: &[IoVec], offset: FileSize) -> Result<usize, Errno> {
    let mut nread = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        nread,
        __wasi_fd_pread(fd, iovs.as_ptr(), iovs.len(), offset, nread.as_mut_ptr()),
    }
}

pub fn fd_pwrite(fd: Fd, iovs: &[CIoVec], offset: FileSize) -> Result<usize, Errno> {
    let mut nwritten = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        nwritten,
        __wasi_fd_pwrite(fd, iovs.as_ptr(), iovs.len(), offset, nwritten.as_mut_ptr()),
    }
}

pub fn random_get(buf: &mut [u8]) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_random_get(buf.as_mut_ptr() as *mut c_void, buf.len()) }
}

pub fn fd_close(fd: Fd) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_close(fd) }
}

pub fn fd_datasync(fd: Fd) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_datasync(fd) }
}

pub fn fd_read(fd: Fd, iovs: &[IoVec]) -> Result<usize, Errno> {
    let mut nread = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        nread,
        __wasi_fd_read(fd, iovs.as_ptr(), iovs.len(), nread.as_mut_ptr()),
    }
}

pub fn fd_renumber(from: Fd, to: Fd) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_renumber(from, to) }
}

pub fn fd_seek(fd: Fd, offset: FileDelta, whence: Whence) -> Result<FileSize, Errno> {
    let mut newoffset = MaybeUninit::<FileSize>::uninit();
    unsafe_wrap1!{
        newoffset,
        __wasi_fd_seek(fd, offset, whence, newoffset.as_mut_ptr()),
    }
}

pub fn fd_tell(fd: Fd) -> Result<FileSize, Errno> {
    let mut newoffset = MaybeUninit::<FileSize>::uninit();
    unsafe_wrap1!{
        newoffset,
        __wasi_fd_tell(fd, newoffset.as_mut_ptr()),
    }
}

pub fn fd_fdstat_get(fd: Fd) -> Result<FdStat, Errno> {
    let mut buf = MaybeUninit::<FdStat>::uninit();
    unsafe_wrap1!{
        buf,
        __wasi_fd_fdstat_get(fd, buf.as_mut_ptr()),
    }
}

pub fn fd_fdstat_set_flags(fd: Fd, flags: FdFlags) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_fdstat_set_flags(fd, flags) }
}

pub fn fd_fdstat_set_rights(fd: Fd, fs_rights_base: Rights, fs_rights_inheriting: Rights) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_fdstat_set_rights(fd, fs_rights_base, fs_rights_inheriting) }
}

pub fn fd_sync(fd: Fd) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_sync(fd) }
}

pub fn fd_write(fd: Fd, iovs: &[CIoVec]) -> Result<usize, Errno> {
    let mut nwritten = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        nwritten,
        __wasi_fd_write(fd, iovs.as_ptr(), iovs.len(), nwritten.as_mut_ptr()),
    }
}

pub fn fd_advise(fd: Fd, offset: FileSize, len: FileSize, advice: Advice) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_advise(fd, offset, len, advice) }
}

pub fn fd_allocate(fd: Fd, offset: FileSize, len: FileSize) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_allocate(fd, offset, len) }
}

pub fn path_create_directory(fd: Fd, path: &[u8]) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_path_create_directory(fd, path.as_ptr(), path.len()) }
}

pub fn path_link(
    old_fd: Fd,
    old_flags: LookupFlags,
    old_path: &[u8],
    new_fd: Fd,
    new_path: &[u8],
) -> Result<(), Errno> {
    unsafe_wrap0!{
        __wasi_path_link(
            old_fd,
            old_flags,
            old_path.as_ptr(),
            old_path.len(),
            new_fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

pub fn path_open(
    dirfd: Fd,
    dirflags: LookupFlags,
    path: &[u8],
    oflags: OFlags,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
    fs_flags: FdFlags,
) -> Result<Fd, Errno> {
    let mut fd = MaybeUninit::<Fd>::uninit();
    unsafe_wrap1!{
        fd,
        __wasi_path_open(
            dirfd,
            dirflags,
            path.as_ptr(),
            path.len(),
            oflags,
            fs_rights_base,
            fs_rights_inheriting,
            fs_flags,
            fd.as_mut_ptr(),
        ),
    }
}

pub fn fd_readdir(fd: Fd, buf: &mut [u8], cookie: DirCookie) -> Result<usize, Errno> {
    let mut bufused = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        bufused,
        __wasi_fd_readdir(
            fd,
            buf.as_mut_ptr() as *mut c_void,
            buf.len(),
            cookie,
            bufused.as_mut_ptr(),
        ),
    }
}

pub fn path_readlink(fd: Fd, path: &[u8], buf: &mut [u8]) -> Result<usize, Errno> {
    let mut bufused = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        bufused,
        __wasi_path_readlink(
            fd,
            path.as_ptr(),
            path.len(),
            buf.as_mut_ptr(),
            buf.len(),
            bufused.as_mut_ptr(),
        ),
    }
}

pub fn path_rename(old_fd: Fd, old_path: &[u8], new_fd: Fd, new_path: &[u8]) -> Result<(), Errno> {
    unsafe_wrap0!{
        __wasi_path_rename(
            old_fd,
            old_path.as_ptr(),
            old_path.len(),
            new_fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

pub fn fd_filestat_get(fd: Fd) -> Result<FileStat, Errno> {
    let mut buf = MaybeUninit::<FileStat>::uninit();
    unsafe_wrap1!{
        buf,
        __wasi_fd_filestat_get(fd, buf.as_mut_ptr()),
    }
}

pub fn fd_filestat_set_times(
    fd: Fd,
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fstflags: FstFlags,
) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_filestat_set_times(fd, st_atim, st_mtim, fstflags) }
}

pub fn fd_filestat_set_size(fd: Fd, st_size: FileSize) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_filestat_set_size(fd, st_size) }
}

pub fn path_filestat_get(fd: Fd, flags: LookupFlags, path: &[u8]) -> Result<FileStat, Errno> {
    let mut buf = MaybeUninit::<FileStat>::uninit();
    unsafe_wrap1!{
        buf,
        __wasi_path_filestat_get(fd, flags, path.as_ptr(), path.len(), buf.as_mut_ptr()),
    }
}

pub fn path_filestat_set_times(
    fd: Fd,
    flags: LookupFlags,
    path: &[u8],
    st_atim: Timestamp,
    st_mtim: Timestamp,
    fstflags: FstFlags,
) -> Result<(), Errno> {
    unsafe_wrap0!{
        __wasi_path_filestat_set_times(
            fd,
            flags,
            path.as_ptr(),
            path.len(),
            st_atim,
            st_mtim,
            fstflags,
        )
    }
}

pub fn path_symlink(old_path: &[u8], fd: Fd, new_path: &[u8]) -> Result<(), Errno> {
    unsafe_wrap0!{
        __wasi_path_symlink(
            old_path.as_ptr(),
            old_path.len(),
            fd,
            new_path.as_ptr(),
            new_path.len(),
        )
    }
}

pub fn path_unlink_file(fd: Fd, path: &[u8]) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_path_unlink_file(fd, path.as_ptr(), path.len()) }
}

pub fn path_remove_directory(fd: Fd, path: &[u8]) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_path_remove_directory(fd, path.as_ptr(), path.len()) }
}

pub fn poll_oneoff(in_: &[Subscription], out: &mut [Event]) -> Result<usize, Errno> {
    assert!(out.len() >= in_.len());
    let mut nevents = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        nevents,
        __wasi_poll_oneoff(
            in_.as_ptr(),
            out.as_mut_ptr(),
            in_.len(),
            nevents.as_mut_ptr(),
        ),
    }
}

pub fn proc_exit(rval: ExitCode) {
    unsafe { __wasi_proc_exit(rval) }
}

pub fn proc_raise(sig: Signal) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_proc_raise(sig) }
}

pub fn sock_recv(sock: Fd, ri_data: &[IoVec], ri_flags: RiFlags) -> Result<(usize, RoFlags), Errno> {
    let mut ro_datalen = MaybeUninit::<usize>::uninit();
    let mut ro_flags = MaybeUninit::<RoFlags>::uninit();

    unsafe {
        let r = __wasi_sock_recv(
            sock,
            ri_data.as_ptr(),
            ri_data.len(),
            ri_flags,
            ro_datalen.as_mut_ptr(),
            ro_flags.as_mut_ptr(),
        );
        if let Some(code) = NonZeroU16::new(r) {
            Err(code)
        } else {
            Ok((ro_datalen.assume_init(), ro_flags.assume_init()))
        }
    }
}

pub fn sock_send(sock: Fd, si_data: &[CIoVec], si_flags: SiFlags) -> Result<usize, Errno> {
    let mut so_datalen = MaybeUninit::<usize>::uninit();
    unsafe_wrap1!{
        so_datalen,
        __wasi_sock_send(
            sock,
            si_data.as_ptr(),
            si_data.len(),
            si_flags,
            so_datalen.as_mut_ptr(),
        ),
    }
}

pub fn sock_shutdown(sock: Fd, how: SdFlags) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_sock_shutdown(sock, how) }
}

pub fn sched_yield() -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_sched_yield() }
}

pub fn fd_prestat_get(fd: Fd) -> Result<Prestat, Errno> {
    let mut buf = MaybeUninit::<Prestat>::uninit();
    unsafe_wrap1!{
        buf,
        __wasi_fd_prestat_get(fd, buf.as_mut_ptr()),
    }
}

pub fn fd_prestat_dir_name(fd: Fd, path: &mut [u8]) -> Result<(), Errno> {
    unsafe_wrap0!{ __wasi_fd_prestat_dir_name(fd, path.as_mut_ptr(), path.len()) }
}

// TODO: Safe interfaces to the args and environ functions
/*
pub fn args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> Errno {}
pub fn args_sizes_get(argc: *mut usize, argv_buf_size: *mut usize) -> Errno {}
pub fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> Errno {}
pub fn environ_sizes_get(environ_count: *mut usize, environ_buf_size: *mut usize) -> Errno {}
*/
