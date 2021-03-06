# Pass

This is password cli manager.

TODO explain how it works

## Requirements

+Rust

## Installation

### Fedora
1. Download the rpm package `curl -o pass-latast.rpm 'https://pass.maurofilipemaia.dev/rpm/latast'`
    1. For a specific version use: `curl -o pass-x.y.z.rpm 'https://pass.maurofilipemaia.dev/rpm/pass-x.y.z.rpm'`
2. check integrity`sha256sum  pass-latast.rpm`
    1. Must match: `...` full list in the end of the README.md file. 
3. Install the package `dnf install pass-latast.rpm`
   
Then test it `pass --help`.