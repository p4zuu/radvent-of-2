use nix::{Error, sched};
use nix::libc::getuid;
use nix::sched::CloneFlags;
use nix::sys::signal::Signal;
use nix::unistd::Pid;

fn main(){
    println!("main pid: {:?}", std::process::id());

    println!("child pid: {:?}", fork().unwrap());

    println!("chimera: stack leak: 0x{:x}", chimera());

    println!("real thread in a new thread group: pid: {:?}", create_real_thread_in_same_group().unwrap());

    let _pid = match create_new_user_namespace() {
        Ok(p) => p,
        Err(err) => panic!("{:?}", err),
    };
}
//  https://man7.org/linux/man-pages/man2/fork.2.html
//  "Since version 2.3.3, rather than invoking the kernel's fork()
//  system call, the glibc fork() wrapper that is provided as part of
//  the NPTL threading implementation invokes clone(2) with flags
//  that provide the same effect as the traditional system call.  (A
//  call to fork() is equivalent to a call to clone(2) specifying
//  flags as just SIGCHLD.)"
fn hello() -> isize {
    println!("Hello from child");
    0
}

fn fork() -> Result<Pid, Error> {
    const STACK_SIZE: usize = 0x1000;
    let mut stack = [0u8; STACK_SIZE];
    let cb = Box::new(|| hello());

    sched::clone(cb, &mut stack, CloneFlags::empty(), Some(Signal::SIGCHLD as i32))
}

fn do_nothing() -> isize {
    0
}

fn chimera() -> usize {
    const STACK_SIZE: usize = 0x1000;
    let mut stack = [0u8; STACK_SIZE];
    let cb = Box::new(|| do_nothing());

    // sched::clone() is currently broken with CLONE_VM, unless CLONE_VFORK is used with
    match sched::clone(cb, &mut stack, CloneFlags::CLONE_VM | CloneFlags::CLONE_VFORK, Some(Signal::SIGCHLD as i32)) {
        Ok(p) => p,
        Err(err) => panic!("failed to clone: {:?}", err),
    };

    // leak the string allocated in the child stack
    let mut vec = stack.to_vec();
    vec.reverse();

    let mut leak: usize = 0;
    for (i, b) in vec[10..16].to_vec().into_iter().enumerate() {
        leak += (b as usize) << (8 * (5-i));
    }

    return leak
}

fn create_real_thread_in_same_group() -> Result<Pid, Error> {
    const STACK_SIZE: usize = 0x1000;
    let mut stack = [0u8; STACK_SIZE];
    let cb = Box::new(|| do_nothing());

    // sched::clone() is currently broken with CLONE_VM, unless CLONE_VFORK is used with
    let flags = CloneFlags::CLONE_VM | CloneFlags::CLONE_VFORK | CloneFlags::CLONE_THREAD | CloneFlags::CLONE_SIGHAND;
    sched::clone(cb, &mut stack, flags, None)
}

fn get_uid() -> isize {
    println!("forked process uid in the new namespace: {:?}", unsafe {getuid()});
    0
}

fn create_new_user_namespace() -> Result<Pid, Error> {
    const STACK_SIZE: usize = 0x1000;
    let mut stack = [0u8; STACK_SIZE];
    let cb = Box::new(|| get_uid());

    // sched::clone() is currently broken with CLONE_VM, unless CLONE_VFORK is used with
    let flags = CloneFlags::CLONE_NEWUSER;
    sched::clone(cb, &mut stack, flags, Some(Signal::SIGCHLD as i32))
}