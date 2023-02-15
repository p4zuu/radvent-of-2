extern crate nix;
use nix::sched;

fn main(){

}
//  https://man7.org/linux/man-pages/man2/fork.2.html
//  "Since version 2.3.3, rather than invoking the kernel's fork()
//  system call, the glibc fork() wrapper that is provided as part of
//  the NPTL threading implementation invokes clone(2) with flags
//  that provide the same effect as the traditional system call.  (A
//  call to fork() is equivalent to a call to clone(2) specifying
//  flags as just SIGCHLD.)"
fn fork() {
    sched::clone();
}