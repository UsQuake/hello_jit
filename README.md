# Rust ARM binary executor for Apple Silicon

 ## Introduction
 
  This project demonstrates generating and executing ARM binary codes in Rust on Apple silicon.

 ## Setup and Requirements
 
 - Rust version 1.71.1 [2021 edition]
 - Using libc crate[version 0.2.149].
 - For now, only Apple-silicon Mac is supported.

 ## Step
  ### 1. Generate executable binary code
  - I'll use below C++ code for execution
  - ```C++
    int fn(int num) {
       return num;
    }
    ```

  ### 2.Compile codes in assembly.
  - below is armv8-a clang 17.0.1 compiled assembly for upper C++ code. 
  - I used [Compiler Explorer](https://godbolt.org/) to compile as assembly-code.
  - ```ASM
    fn(int):    //@demangled fn(int)
    sub     sp, sp, #16
    str     w0, [sp, #12]
    ldr     w0, [sp, #12]
    add     sp, sp, #16
    ret
    ```
 ### 3.Convert assembly-code into ARM-instruction binary code.
  - I used [armconverter](https://armconverter.com/) to convert armv8_a asm to hex(binary) code.
  - ```
     0xD10043FF
     0xB9000FE0
     0xB9400FE0
     0x910043FF
     0xD65F03C0
    ```
 ### 4.Save binary code on executor's executable region and execute it.

  - ```Rust

    //Below is compiled binaries of 'int fn(int num)' function
    
    let mut code: [u32;5] = [
    0xD10043FF,
    0xB9000FE0,
    0xB9400FE0,
    0x910043FF,
    0xD65F03C0
    ];
    
    unsafe{
        //Allocate executable memory region with mmap().
        let ptr_shared_mem = mmap(std::ptr::null_mut(), 4 * 5, PROT_WRITE,
        MAP_JIT //Only for Mac_OS
         | MAP_ANON | MAP_PRIVATE,-1,0);

        //Use std::ptr::copy() which is alternative of memcpy() in libc,
        //But, it's actually runs like memmove() in libc.
        std::ptr::copy(code.as_mut_ptr(), ptr_shared_mem as *mut u32, 5);
       //Be done to write executable binary on executable memory region.

       //Now, Change the permissions of the region from writeable to executable.
       mprotect(ptr_shared_mem, 4 * 5, PROT_EXEC);

       //WARNING: Don't give permission of exectuable and writeable at same time as calling mmap().
       //If you give permission of exectuable and writeable at same time,
       //It will be super big security problem.
       //Because, What we are doing is actually code injection on runtime,
       //so, If we give permission of exectuable and writeable at same time, It is vulnerable to shellcode injection attack.

        //Finally, convert pointer of executable region into function pointer and call it!
        let func = std::mem::transmute::<*mut c_void, fn(i32) -> i32>(ptr_shared_mem);
        println!("{}",func(3));
    }
    ```
## Acknowledge

 - [simple jit in c](https://blog.reverberate.org/2012/12/hello-jit-world-joy-of-simple-jits.html)

  
