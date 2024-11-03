# lexopt C bindings

## Installation

<dl>
<dt>Ubuntu x86-64
<dd>

```sh
wget https://github.com/jcbhmr/lexopt/releases/download/v0.1.0/lexopt-x86_64-unknown-linux-gnu.tar.gz
tar -xzvf lexopt-x86_64-unknown-linux-gnu.tar.gz
```

<dt>Ubuntu AArch64
<dd>

```sh
wget https://github.com/jcbhmr/lexopt/releases/download/v0.1.0/lexopt-aarch64-unknown-linux-gnu.tar.gz
tar -xzvf lexopt-aarch64-unknown-linux-gnu.tar.gz
```

<dt>macOS x86-64
<dd>

```sh
wget https://github.com/jcbhmr/lexopt/releases/download/v0.1.0/lexopt-x86_64-apple-darwin.tar.gz
tar -xzvf lexopt-x86_64-apple-darwin.tar.gz
```

<dt>macOS AArch64
<dd>

```sh
wget https://github.com/jcbhmr/lexopt/releases/download/v0.1.0/lexopt-aarch64-apple-darwin.tar.gz
tar -xzvf lexopt-aarch64-apple-darwin.tar.gz
```

<dt>Windows x86-64
<dd>

```pwsh
Invoke-WebRequest "https://github.com/jcbhmr/lexopt/releases/download/v0.1.0/lexopt-x86_64-pc-windows-msvc.zip"
Expand-Archive "lexopt-x86_64-pc-windows-msvc.zip"
```

<dt>Windows AArch64
<dd>

```pwsh
Invoke-WebRequest "https://github.com/jcbhmr/lexopt/releases/download/v0.1.0/lexopt-aarch64-pc-windows-msvc.zip"
Expand-Archive "lexopt-aarch64-pc-windows-msvc.zip"
```

</dl>

## Usage

```c
#include <lexopt.h>
#include <stdio.h>
#include <>

int main(int argc, char *argv[]) {

    lexopt_parser parser = lexopt_parser_from_env();
    
    return 0;
}
```