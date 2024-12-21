pub const EPOLL_CTL_ADD: i32 = 1;

/// read operations on file handle
pub const EPOLLIN: i32 = 0x1;

/// getting events notified with epoll set to and edge-triggered mode
pub const EPOLLET: i32 = 1 << 31;

#[link(name = "c")]
extern "C" {
    /// syscall to create epoll queue
    pub fn epoll_create(size: i32) -> i32;

    /// syscall to close file descriptor
    pub fn close(fd: i32) -> i32;

    /// control interface to perform operations on epoll instance.
    /// registers interest in events on source
    /// supports: add, modify or delete
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;

    /// syscall that blocks current thread and wait until:
    ///     - we receive a notification that event has occured
    ///     - time outs
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

/// structure to communicate to operating system in epoll_ctl
/// operating system uses the same structure to communicate with us in epoll_wait
#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub events: u32,
    pub epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}