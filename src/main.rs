use libc::*;

fn main() {

/*C++
int fn(int num) {
  return num;
}
*/
/*armv8-a clang 17.0.1 compiled assembly
    fn(int):    //@demangled fn(int)
    sub     sp, sp, #16
    str     w0, [sp, #12]
    ldr     w0, [sp, #12]
    add     sp, sp, #16
    ret
*/
    let mut code: [u32;5] = [
    0xD10043FF,
    0xB9000FE0,
    0xB9400FE0,
    0x910043FF,
    0xD65F03C0
    ];
    
    unsafe{
        let ptr_shared_mem = mmap(std::ptr::null_mut(), 4 * 5, PROT_WRITE,
        MAP_JIT //Only for Mac_OS
         | MAP_ANON | MAP_PRIVATE,-1,0);
        std::ptr::copy(code.as_mut_ptr(), ptr_shared_mem as *mut u32, 5);
        mprotect(ptr_shared_mem, 4 * 5, PROT_EXEC);
        let func = std::mem::transmute::<*mut c_void, fn(i32) -> i32>(ptr_shared_mem);
        println!("{}",func(3));
    }
    
}
