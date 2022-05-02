# rust_string_overheads

Provide information on the overheads of different Rust string types on the current processor architecture.

On a x64 system, this shows that:
* `&str` has an overhead of 16 bytes
* `Box<&str>` has an overhead of 24 bytes
* `String` has an overhead of 24 bytes
* `Box<String>` has an overhead of 32 bytes

Types based on `str` also have the benefit of being immutable. `String` is mutable.

## Usage

* Install Rust using rustup <https://rustup.rs/>
* `cargo run`

## Example Output

Example output for x64 (8 byte usize and pointers):

```text
    str 'Hello World' (internally: 1 byte per UTF-8 character for English)
    [src/main.rs:12] size_of_val::<str>(DATA) = 11
    
    &str (internally: pointer+length)
    [src/main.rs:18] size_of_val::<&str>(&DATA) = 16
    
    Box<&str> (internally: pointer+&str)
    [src/main.rs:25] size_of_val::<Box<&str>>(&boxed_str_ref) = 8
    Boxed<&str> overhead is 8+16=24 bytes
    
    String (internally: pointer+length+capacity)
    [src/main.rs:38] size_of_val::<String>(&string1) = 24
    
    Box<String> (internally: pointer+String)
    [src/main.rs:45] size_of_val::<Box<String>>(&boxed_string) = 8
    Box<String> overhead is 8+24=32 bytes
```

Using `gdb` to dump the layout of `print_holder` shows the difference between `Box<&str>` and `Box<String>`:

```text
    (gdb) ptype /o value
    /* offset      |    size */  type = struct rust_string_overheads::Holder {
    /*	0      |       8 */    field_box_of_str: *mut &str,
    /*	8      |       8 */    field_box_of_string: *mut alloc::string::String,
    
                                   /* total size (bytes):   16 */
                                 }

    (gdb) ptype /o *value.field_box_of_str
    /* offset      |    size */  type = struct &str {
    /*	0      |       8 */    data_ptr: *mut u8,
    /*	8      |       8 */    length: usize,
    
                                   /* total size (bytes):   16 */
                                 }
                                     
    (gdb) ptype /o *value.field_box_of_string
    /* offset      |    size */  type = struct alloc::string::String {
    /*	0      |      24 */    vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
    
                                   /* total size (bytes):   24 */
                                 }
```

## License

MIT permissive license. See LICENSE for full license details.

## Source Code Repository

<https://github.com/thomasbratt/rust_string_overheads>
