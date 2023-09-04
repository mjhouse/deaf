<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<div align="center">

[![Tests][tests-shield]][tests-url]
[![Docs][docs-shield]][docs-url]
[![GPL v3][license-shield]][license-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
    <h3 align="center">DEAF</h3>
    <p align="center">
        A Rust library for parsing and modifying ELF binaries
        <br />
        <a href="https://github.com/mjhouse/deaf/blob/master/CONTRIBUTING.md">Contributions</a>
        ·
        <a href="https://github.com/mjhouse/deaf/blob/master/CODE_OF_CONDUCT.md">Code Of Conduct</a>
        ·
        <a href="https://github.com/mjhouse/deaf/issues/new?labels=bug&template=bug_report.md">Report Bug</a>
        ·
        <a href="https://github.com/mjhouse/deaf/issues/new?labels=enhancement&template=feature_request.md">Request Feature</a>
    </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#introduction">Introduction</a></li>
        <li><a href="#getting-started">Getting Started</a></li>
        <li><a href="#roadmap">Roadmap</a></li>
        <li><a href="#contributing">Contributing</a></li>
        <li><a href="#contact">Contact</a></li>
    </ol>
</details>

## Introduction

This is a library for parsing and modifying ELF-format binaries. There are other libraries like [elf](https://crates.io/crates/elf), 
[elfy](https://crates.io/crates/elfy), and [elfkit](https://crates.io/crates/elfkit) for parsing the format, but they are either 
not able to modify the ELF binary (elf and elfy) or are limited/unmaintained (elfkit). DEAF is written with the explicit goal of
allowing users to modify anything that they can see in the binary- you should be able to add symbols to the symbol tables, remove 
the body of a function, or change the name of a section, easily and intuitively. Some of the changes you can make will probably break 
the binary. For example, if you remove the body of a function then relative branch instructions that after the gap will no longer 
point to the correct location.

As the development of the library progresses, there should be fewer and fewer ways that your changes break the ELF binary, and the 
end goal is to even handle updating branch instructions when code is removed from an executable section.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Anyone is welcome to contribute to DEAF, just try to follow the [code of conduct](https://github.com/mjhouse/deaf/blob/master/CODE_OF_CONDUCT.md) 
and the [contribution guidelines](https://github.com/mjhouse/deaf/blob/master/CONTRIBUTING.md). If something is unclear
or not covered in the guides, create an issue describing the problem and someone will get back to you as soon as possible.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contact

[Create an issue](https://github.com/mjhouse/deaf/issues) and @mjhouse to get my attention, or email me at mjhouse@protonmail.com.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
[docs-shield]: https://github.com/mjhouse/deaf/actions/workflows/docs.yaml/badge.svg?branch=documentation&style=for-the-badge
[docs-url]: https://mjhouse.github.io/deaf/

[tests-shield]: https://github.com/mjhouse/deaf/actions/workflows/test.yaml/badge.svg?branch=testing&style=for-the-badge
[tests-url]: https://github.com/mjhouse/deaf/actions/workflows/test.yaml

[license-shield]: https://img.shields.io/badge/License-GPLv3-blue.svg&style=for-the-badge
[license-url]: https://www.gnu.org/licenses/gpl-3.0
