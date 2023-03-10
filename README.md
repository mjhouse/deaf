# DEAF: The **D**efinitive **E**lf **A**nalysis **F**ramework

[![Tests](https://github.com/mjhouse/deaf/actions/workflows/test.yaml/badge.svg?branch=testing)](https://github.com/mjhouse/deaf/actions/workflows/test.yaml)
[![Docs](https://github.com/mjhouse/deaf/actions/workflows/docs.yaml/badge.svg?branch=documentation)](https://mjhouse.github.io/deaf/)
[![GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

[Contribution Guide](https://github.com/mjhouse/deaf/blob/master/CONTRIBUTING.md)  
[Code of Conduct](https://github.com/mjhouse/deaf/blob/master/CODE_OF_CONDUCT.md)

This is a library for parsing *and modifying* ELF-format applications and libraries. There are many other 
libraries out there for parsing the ELF format, like the appropriately-named [elf](https://crates.io/crates/elf), 
[elfy](https://crates.io/crates/elfy), and [elfkit](https://crates.io/crates/elfkit) crates, but they are either 
not designed with modification in mind (elf/elfy) or are very limited (and now unmaintained) (elfkit). To fix this 
situation, I'm writing DEAF with the explicit and primary goal of making it possible to *change anything you can 
see*. This means that if you load a shared library and parse the dynamic symbol table, you should be able to- 

* Reorder
* Remove
* Replace
* Update
* Append

-symbols and then write the resulting symbol table back into a file. Existing toolkits generally don't
have this capability, or it's very limited and oriented toward cybersecurity tasks like injecting telemetry or 
inserting malicious relocation records. If you want remove the body, symbol and relocation records of a 
particular function from a library, then shift all following functions forward, then update any relocation/init 
or preinit array references, you just *can't*.  

DEAF exists specifically to make things like this possible.

# Notes

* The name "DEAF" is a back-ronym- it doesn't mean anything. It's just a short word that reminded me of 
  the DEADBEEF test value used in a lot of hex examples. A friend suggested "Definitive Elf Analysis Framework"
  as the meaning after I already had the name, and I couldn't think of anything better.
* Any time you see an attribute/method named `kind`, it's a synonym for `type` (e.g. Relocation::kind)
  which is a reserved keyword in the rust language, but is used for a lot of fields in the ELF format.

# References

| Topic       | Url                                                                    |
|--           |--                                                                      |
| ELF Format  | [ELF_Format.pdf](http://www.skyfree.org/linux/references/ELF_Format.pdf)                 |
| Symbols     | [Oracle Docs: Symbols](https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html) |
| Relocations | [Oracle Docs: Relocations](https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-54839.html) |
| Init Array  | [Oracle Docs: Init](https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter3-8.html) |