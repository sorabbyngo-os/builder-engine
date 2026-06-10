<div align="center">
<br/>

# builder-engine

### The Sorabbyngo OS image build system — rootfs, ISO, OTA packages, and patch bundles.

<br/>

[![Division](https://img.shields.io/badge/Division-Sorabbyngo_OS-6C3FC4?style=for-the-badge)](https://github.com/sorabbyngo-os)
[![Lang](https://img.shields.io/badge/Lang-Rust-AA66FF?style=for-the-badge)](https://github.com/sorabbyngo-os/builder-engine)
[![Status](https://img.shields.io/badge/Status-In_Development-FF6B35?style=for-the-badge)](https://github.com/sorabbyngo-os/builder-engine)

</div>

---

## What It Does

builder-engine is the build orchestration system for all Sorabbyngo OS images. It produces bootable ISOs, rootfs archives, OTA update packages, and patch bundles — reproducibly, from a declarative build manifest.

---

## Features

| Feature | Description |
|---|---|
| **Reproducible Builds** | Hermetic build environments — same inputs, same outputs, always |
| **Multi-surface** | Builds byngo-os, edge-os, novela-os, sentinel-os, server-os from one toolchain |
| **OTA Packages** | Signed, delta OTA update packages for fleet deployment |
| **Patch Bundles** | Security patch bundles for sorabbyngo-patches |
| **Image Signing** | All outputs signed with Dilithium-3 (post-quantum) |
| **Build Cache** | Content-addressable build cache — skip unchanged layers |
| **CI Integration** | GitHub Actions integration for automated nightly and release builds |

---

## Architecture

```
builder-engine/
├── src/
│   ├── manifest/     # Build manifest parser and validator
│   ├── rootfs/       # Rootfs construction and customisation
│   ├── iso/          # ISO image assembly
│   ├── ota/          # OTA package generation and delta computation
│   ├── signing/      # Post-quantum image signing (Dilithium-3)
│   ├── cache/        # Content-addressable build cache
│   └── ci/           # CI/CD integration helpers
├── manifests/        # Example build manifests per OS surface
├── Cargo.toml
└── README.md
```

---

## Build Manifests

```toml
# manifests/edge-os.toml
[image]
name = "edge-os"
version = "2026.2.0"
target_arch = "aarch64"

[kernel]
source = "sorabbyngo-os/sorabbyngo-patches"
config = "configs/edge_defconfig"

[packages]
include = ["byngonet", "byngox-runtime", "soraxis-platform"]
exclude = ["wayland", "pulseaudio"]

[signing]
key = "env:SORABBYNGO_SIGNING_KEY"
algorithm = "dilithium3"
```

---

## Stack

- **Language:** Rust
- **Build isolation:** Linux namespaces / containers
- **Signing:** Dilithium-3 (ML-DSA, FIPS 204)
- **Dependencies:** `sorabbyngo-patches`, `sorabbyngo-build`, `identity-core`

---

*Sorabbyngo OS · Kisii, Kenya · 2026*
