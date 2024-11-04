// SH4 KallistiOS C definitions

pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;

pub type c_size_t = usize;
pub type c_ssize_t = isize;

pub type wchar_t = ::c_long;

pub type u_register_t = ::uintptr_t;
pub type u_char = ::c_uchar;
pub type u_short = ::c_ushort;
pub type u_int = ::c_uint;
pub type u_long = c_ulong;
pub type ushort = ::c_ushort;
pub type uint = ::c_uint;
pub type ulong = c_ulong;
pub type clock_t = c_ulong;
pub type daddr_t = c_long;
pub type caddr_t = *mut c_char;
pub type sbintime_t = ::c_longlong;
pub type sigset_t = ::c_ulong;

s! {
    // d_name is a flexible array member so this definition is
    // inaccurate but the std lib doesn't care. It uses a macro
    // to take the address and treat it correctly.
    pub struct dirent {
        pub d_ino: ::c_int,
        pub d_off: ::off_t,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_name: [::c_char; 256usize],
    }

    pub struct hostent {
        pub h_name: *mut ::c_char,
        pub h_aliases: *mut *mut ::c_char,
        pub h_addrtype: ::c_int,
        pub h_length: ::c_int,
        pub h_addr_list: *mut *mut ::c_char,
    }

    pub struct sched_param {
        pub sched_priority: ::c_int,
    }

    pub struct semaphore_t {
        pub initialized: ::c_int,
        pub count: ::c_int,
    }

    pub struct sockaddr {
        pub sa_family: ::sa_family_t,
        pub sa_data: [::c_char; 26usize],
    }

    pub struct sockaddr_storage {
        pub ss_family: ::sa_family_t,
        pub __ss_pad1: [::c_char; 7usize],
        pub __ss_align: u64,
        pub __ss_pad2: [::c_char; 112usize],
    }

    pub struct sockaddr_in {
        pub sin_family: ::sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [::c_uchar; 8usize],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: ::sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    // KallistiOS doesn't support Unix sockets, but including
    // this definition to allow things to build...
    pub struct sockaddr_un {
        pub sun_len: ::c_uchar,
        pub sun_family: ::sa_family_t,
        pub sun_path: [::c_char; 104usize],
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atim: ::timespec,
        pub st_mtim: ::timespec,
        pub st_ctim: ::timespec,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_spare4: [::c_long; 2usize],
    }
}

pub const SIGEV_NONE: ::c_int = 1;
pub const SIGEV_SIGNAL: ::c_int = 2;
pub const SIGEV_THREAD: ::c_int = 3;
pub const SA_NOCLDSTOP: ::c_int = 1;
pub const MINSIGSTKSZ: ::c_int = 2048;
pub const SIGSTKSZ: ::c_int = 8192;
pub const SS_ONSTACK: ::c_int = 1;
pub const SS_DISABLE: ::c_int = 2;
pub const SIG_SETMASK: ::c_int = 0;
pub const SIG_BLOCK: ::c_int = 1;
pub const SIG_UNBLOCK: ::c_int = 2;
pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGTRAP: ::c_int = 5;
pub const SIGABRT: ::c_int = 6;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGBUS: ::c_int = 10;
pub const SIGSEGV: ::c_int = 11;
pub const SIGSYS: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;
pub const SIGURG: ::c_int = 16;
pub const SIGSTOP: ::c_int = 17;
pub const SIGTSTP: ::c_int = 18;
pub const SIGCONT: ::c_int = 19;
pub const SIGCHLD: ::c_int = 20;
pub const SIGCLD: ::c_int = 20;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGIO: ::c_int = 23;
pub const SIGPOLL: ::c_int = 23;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGLOST: ::c_int = 29;
pub const SIGUSR1: ::c_int = 30;
pub const SIGUSR2: ::c_int = 31;

pub const WNOHANG: ::c_int = 1;

pub const POLLIN: ::c_short = 3;
pub const POLLPRI: ::c_short = 4;
pub const POLLOUT: ::c_short = 8;
pub const POLLRDNORM: ::c_short = 1;
pub const POLLWRNORM: ::c_short = 8;
pub const POLLRDBAND: ::c_short = 2;
pub const POLLWRBAND: ::c_short = 16;
pub const POLLERR: ::c_short = 32;
pub const POLLHUP: ::c_short = 64;
pub const POLLNVAL: ::c_short = 128;

pub const EAI_AGAIN: ::c_int = 1;
pub const EAI_BADFLAGS: ::c_int = 2;
pub const EAI_FAIL: ::c_int = 3;
pub const EAI_SERVICE: ::c_int = 7;
pub const EAI_SYSTEM: ::c_int = 9;
pub const EAI_OVERFLOW: ::c_int = 10;

pub const NSIG: ::c_int = 32;
pub const CLOCK_ENABLED: ::c_int = 1;
pub const CLOCK_DISABLED: ::c_int = 0;
pub const CLOCK_ALLOWED: ::c_int = 1;
pub const CLOCK_DISALLOWED: ::c_int = 0;
pub const TIMER_ABSTIME: ::c_int = 4;
pub const SOL_SOCKET: ::c_int = 1;
pub const MSG_OOB: ::c_int = 8;
pub const MSG_PEEK: ::c_int = 16;
pub const MSG_DONTWAIT: ::c_int = 128;
pub const MSG_DONTROUTE: ::c_int = 2;
pub const MSG_WAITALL: ::c_int = 64;
pub const MSG_EOR: ::c_int = 4;
pub const MSG_TRUNC: ::c_int = 32;
pub const MSG_CTRUNC: ::c_int = 1;
pub const AT_FDCWD: ::c_int = -2;
pub const AT_EACESS: ::c_int = 1;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 2;
pub const AT_SYMLINK_FOLLOW: ::c_int = 4;
pub const AT_REMOVEDIR: ::c_int = 8;

pub const _SC_PAGESIZE: ::c_int = 8;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 51;

pub const PTHREAD_STACK_MIN: ::size_t = 256;

pub const IOV_MAX: ::size_t = 1024;

pub const AF_INET6: ::c_int = 2;

// Not in KOS, but defined to allow compilation
pub const AF_UNIX: ::c_int = 1;
// Not in KOS, but defined to allow compilation
pub const FIONBIO: ::c_ulong = 1;

pub const O_DIRECTORY: ::c_int = 0x200000;
pub const O_NOFOLLOW: ::c_int = 0x100000;

pub const SOMAXCONN: ::c_int = 32;

// For pthread get/setschedparam
pub const SCHED_FIFO: ::c_int = 1;
pub const SCHED_RR: ::c_int = 2;

// Not in KOS, but defined to satisfy sys/pal/unix/weak.rs
pub const RTLD_DEFAULT: *mut ::c_void = 0 as *mut ::c_void;

extern "C" {
    pub fn arc4random_buf(buf: *mut ::c_void, nbytes: ::size_t);
    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;
    pub fn getentropy(ptr: *mut ::c_void, len: ::size_t) -> ::c_int;
    pub fn gethostid() -> ::c_long;

    pub fn pthread_create(
        native: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(_: *mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;

    pub fn pthread_attr_getschedparam(
        attr: *const ::pthread_attr_t,
        param: *mut sched_param,
    ) -> ::c_int;

    pub fn pthread_attr_setschedparam(
        attr: *mut ::pthread_attr_t,
        param: *const sched_param,
    ) -> ::c_int;

    pub fn pthread_condattr_getclock(
        attr: *const ::pthread_condattr_t,
        clock_id: *mut ::clockid_t,
    ) -> ::c_int;

    pub fn pthread_condattr_setclock(
        attr: *mut ::pthread_condattr_t,
        clock_id: ::clockid_t,
    ) -> ::c_int;
}
