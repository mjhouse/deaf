<a name="readme-top"></a>

# DEAF: The **D**efinitive **E**lf **A**nalysis **F**ramework


<!-- PROJECT SHIELDS -->
[![Documentation][docs-shield]][docs-url]
[![Issues][issues-shield]][issues-url]
[![Tests][tests-shield]][tests-url]
[![GPLv3][license-shield]][license-url]

[Contribution Guide](https://github.com/mjhouse/deaf/blob/master/CONTRIBUTING.md)  
[Code of Conduct](https://github.com/mjhouse/deaf/blob/master/CODE_OF_CONDUCT.md)

This is a library for parsing and modifying ELF-format binaries. There are other libraries like [elf](https://crates.io/crates/elf), 
[elfy](https://crates.io/crates/elfy), and [elfkit](https://crates.io/crates/elfkit) for parsing the format, but they are either 
not able to modify the ELF binary (elf and elfy) or are limited/unmaintained (elfkit). DEAF is written with the explicit goal of
allowing users to modify anything that they can see in the binary- you should be able to add symbols to the symbol tables, remove 
the body of a function, or change the name of a section, easily and intuitively. Some of the changes you can make will probably break 
the binary. For example, if you remove the body of a function then relative branch instructions that after the gap will no longer 
point to the correct location.

As the development of the library progresses, there should be fewer and fewer ways that your changes break the ELF binary, and the 
end goal is to even handle updating branch instructions when code is removed from an executable section.

## Examples

## Contributions

## 

<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/mjhouse/deaf.svg?style=for-the-badge
[contributors-url]: https://github.com/mjhouse/deaf/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/mjhouse/deaf.svg?style=for-the-badge
[forks-url]: https://github.com/mjhouse/deaf/network/members

[stars-shield]: https://img.shields.io/github/stars/mjhouse/deaf.svg?style=for-the-badge
[stars-url]: https://github.com/mjhouse/deaf/stargazers

[issues-shield]: https://img.shields.io/github/issues/mjhouse/deaf.svg?style=for-the-badge
[issues-url]: https://github.com/mjhouse/deaf/issues

[docs-shield]: https://img.shields.io/github/actions/workflow/status/mjhouse/deaf/docs.yaml?branch=documentation&style=for-the-badge&label=Documentation&message=latest
[docs-url]: https://mjhouse.github.io/deaf/

[tests-shield]: https://img.shields.io/github/actions/workflow/status/mjhouse/deaf/test.yaml?branch=testing&style=for-the-badge&label=Tests
[tests-url]: https://github.com/mjhouse/deaf/actions/workflows/test.yaml

[license-shield]: https://img.shields.io/github/license/mjhouse/deaf.svg?style=for-the-badge
[license-url]: https://github.com/mjhouse/deaf/blob/master/LICENSE

[crate-shield]: https://img.shields.io/crates/d/deaf.svg?style=for-the-badge
[crate-url]: https://crates.io/crates/deaf
