# t8bar

A Screen Sharing touch bar utility.

## State

Relatively simple but arguably bad code is working - this was mostly an
experiment of building something that's actually useful using Rust. Code
improvements and bug fixes will come - eventually.

## Usage

Create your configuration file with local hostnames and GitHub names of your
 mates, run the binary and enjoy.

In `~/.config/t8bar/config.json`:
```json
[
  {
    "hostname": "cunha.local",
    "github": "drgmr"
  }
]
```

## License

The MIT License (MIT)

Copyright (c) 2019 Eduardo Cunha

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
