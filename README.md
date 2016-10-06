# lib_uapi_linux
Generates bindings to your local &lt;linux/*.h> files!

This library currently does not work. When it does it will read your local `/usr/include/linux/` file. Finding all header files in this location it will then extract structures, enums, and macro definations. The goal is to create a relatively simple to interact with method of pulling kernel files. 

This is a hold over until more work.

This crate is unusable as it is blocked by bindgen.
