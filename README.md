![Marlin Oyster Logo](./logo.svg)

# Vet

Curl-like utility to make http requests over vsocks.

## Build

```bash
cargo build --release
```

### Reproducible builds

Reproducible builds can be done using a Rust Docker image to standardize the build environment:

```bash
# For amd64
docker run --rm -v `pwd`:/code rust@sha256:ed7795c6eaccae53be35939e883e8c3de0197b21e8eddbd9f04b0c4bc757c094 /code/build-amd64.sh

# For arm64
docker run --rm -v `pwd`:/code rust@sha256:c428882ff081342a9661fb13a1d059ecdc0b6e979ffec64b80371cf20a2088b0 /code/build-arm64.sh
```

The prebuilt binaries are then compressed using `upx` version 4.2.4. Expected sha256 checksums are available along with the links to the prebuilt binaries.

## Prebuilt binaries

amd64: https://artifacts.marlin.org/oyster/binaries/vet_v1.0.0_linux_amd64 \
checksum: cc232f2bbf4a808638ddf54ed19e79ebfcba558a7fb902c02d7a8f92562231a9

arm64: https://artifacts.marlin.org/oyster/binaries/vet_v1.0.0_linux_arm64 \
checksum: f052d9f257caf5212c9b65e8c7cd44bfd00fe38f2596cc7a9b6d8f06ecfeff4a

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
