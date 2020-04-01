const STACK_SIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}