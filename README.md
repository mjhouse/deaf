# DEAF: the **D**efinitive **E**lf **A**nalysis **F**ramework

This is a library for parsing *and modifying* ELF-format applications and libraries. There are many other 
libraries out there for parsing the ELF-format, like the appropriately-named [elf](https://crates.io/crates/elf)
crate, [elfy](https://crates.io/crates/elfy), and [elfkit](https://crates.io/crates/elfkit), but they are either 
not designed with modification in mind (elf/elfy) or are limited and unsupported (elfkit). To fix this situation, 
I'm writing DEAF with the explicit and primary goal of making it possible to *change anything you can see*. This 
means that if you load an ELF-format shared library and iterate through the functions defined in the dynamic symbol 
table, you should be able to- 

* Reorder
* Remove
* Replace
* Update
* Append

-symbols and then write the resulting symbol table back into a file. Existing toolkits generally don't
have this capability, or it's very limited and oriented toward cybersecurity tasks like injecting telemetry or 
redirecting relocations. If you want remove the body, symbol and relocation records of a particular function from 
a library, then shift all following functions forward, then update any relocation/init or preinit array
references, you just *can't*. DEAF exists specifically to make things like this possible.

# Notes

* Any time you see an attribute/method named `kind`, it's a synonym for `type` (e.g. Relocation::kind)
  which is a reserved keyword in the rust language.

# References

| Topic       | Url                                                                    |
|--           |--                                                                      |
| ELF Format  | http://www.skyfree.org/linux/references/ELF_Format.pdf                 |
| Symbols     | https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html |
| Relocations | https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html |