![Marlin Oyster Logo](./logo.svg)

# Vet

Curl-like utility to make http requests over vsocks.

## Build

```bash
cargo build --release
```

## Prebuilt binaries

amd64: http://public.artifacts.marlin.pro/projects/enclaves/vet_v1.0.0_linux_amd64

arm64: http://public.artifacts.marlin.pro/projects/enclaves/vet_v1.0.0_linux_arm64

## Usage

```bash
$ ./target/release/vet --help
Usage: vet --url <URL>

Options:
  -u, --url <URL>  url to query
  -h, --help       Print help
  -V, --version    Print version
```

## Example

```
$ vet --url 3:1500/oyster/job
0x1234567812345678123456781234567812345678123456781234567812345678
```
