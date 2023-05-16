## [Release 0.10.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.10.0)

### ✨ Features

-  **search:** add advanced search filter option ([`25095b8`](https://github.com/Arrow-air/svc-storage/commit/25095b8170f8450abe8a409c84807442d8617896))
-  **search:** implement advanced search for resources ([`033b82f`](https://github.com/Arrow-air/svc-storage/commit/033b82f922269aeab8450e9ad5a52f6c80926a6d))
- add itinerary table ([`a2eadbe`](https://github.com/Arrow-air/svc-storage/commit/a2eadbe14f87145a74db82a9c6aa3dfc8b7f728a))
-  **vehicle:** add last_vertiport_id field ([`df8120c`](https://github.com/Arrow-air/svc-storage/commit/df8120cb11c7f8b4ad852f63a9134f4c7dd080b6))
-  **adsb:** add adsb service ([`8b2dcbd`](https://github.com/Arrow-air/svc-storage/commit/8b2dcbde9e3e2c96dbfd336837c822e2e57e4181))
-  **mock:** initial mock modules ([`a54d1ec`](https://github.com/Arrow-air/svc-storage/commit/a54d1ecc7a5f8e42e276e6f5ae2133a044b4c009))
- add linked resource functionality ([`a819362`](https://github.com/Arrow-air/svc-storage/commit/a819362b6810630352166a2e0c9e0c4478b4e179))

### 🐛 Fixes

-  **flight_plan:** reword fields and add macro ([`241d079`](https://github.com/Arrow-air/svc-storage/commit/241d079b497d796fbba5daa5eedf462ffa3844ef))
-  **postgres:** delete function now checks if deleted_at should be set ([`809e8a8`](https://github.com/Arrow-air/svc-storage/commit/809e8a854bcab6786c4eaba885cf9e98aabcbacb))
- fix docstring tests ([`c4d28d3`](https://github.com/Arrow-air/svc-storage/commit/c4d28d3e9166506a85ed05b2524bb5d659cbd712))
-  **vehicle:** uuid to string conversion and added example ([`104a722`](https://github.com/Arrow-air/svc-storage/commit/104a7222ec18f1b5ad8ea0fe9c6393c191cb2c0a))

### 🔥 Refactorings

-  **flightplan:** use generic traits (#27) ([`8a3c940`](https://github.com/Arrow-air/svc-storage/commit/8a3c940d50ca3dee5aa6885b7b8adbecd4687eb3))
-  **gRPC:** implement generic functions for gRPC ([`2c17079`](https://github.com/Arrow-air/svc-storage/commit/2c170795e81b068fe8bf08301cd9de16f7a25db2))
-  **gRPC:** add macro to generate From trait implementations ([`582c927`](https://github.com/Arrow-air/svc-storage/commit/582c9276b9eec46fb27c5971c6fb36bd53f3dab3))
-  **vertiport:** use generic structs and traits ([`2960fde`](https://github.com/Arrow-air/svc-storage/commit/2960fdeb37994374887f044e2e49186da2aab142))
-  **vertipad:** use generic structs and traits ([`662925a`](https://github.com/Arrow-air/svc-storage/commit/662925aeaecf09b50035b8299952881b1c4ad38c))
-  **vehicle:** use generic structs and traits ([`8af5a07`](https://github.com/Arrow-air/svc-storage/commit/8af5a07c06c30b07471f96f42d15a604d142ca22))
-  **config:** created separate config module ([`4998531`](https://github.com/Arrow-air/svc-storage/commit/4998531a87759d8fe4e16954be0a646ca2a5fb50))

### 🛠 Maintenance

- terraform provisioned file changes ([`a4d6b99`](https://github.com/Arrow-air/svc-storage/commit/a4d6b991745a53ea245449d80b69c4492f17db16))
- update release files ([`4c3cf77`](https://github.com/Arrow-air/svc-storage/commit/4c3cf772f82e0e6afbcbb5eafda0d37a9c56657f))
-  **checks:** sanity check fixes ([`3c6b032`](https://github.com/Arrow-air/svc-storage/commit/3c6b032246d8e23e77913c7bfc47d3b8decdbe3c))
- reviewer comments ([`daa9496`](https://github.com/Arrow-air/svc-storage/commit/daa9496ebff3e5398cb3f12d32ebb7c244b53ac3))
- add status field to itinerary ([`9757316`](https://github.com/Arrow-air/svc-storage/commit/97573163d85461a0dbaa847cbc4110caec03acf3))
- update examples ([`a04a1f6`](https://github.com/Arrow-air/svc-storage/commit/a04a1f61de7eb52d32e6e796b4b0e4474d3e66bb))
-  **cargo:** fix versions and package info ([`726b59b`](https://github.com/Arrow-air/svc-storage/commit/726b59bbc7b3ba290ccfd3fb9df3415c2916109b))
- remove obsolete files ([`4cff8d3`](https://github.com/Arrow-air/svc-storage/commit/4cff8d3c728963c2c83f52a97b6b69429f7a896b))
- get rid of all unwrap() calls ([`bf0dc85`](https://github.com/Arrow-air/svc-storage/commit/bf0dc8567cb93d74cb0d77ab23b75ed4e9a46202))
-  **grpc:** add server module as per updated service template ([`eb7a153`](https://github.com/Arrow-air/svc-storage/commit/eb7a15365303e90ec212c4828ec07b0e1332a678))
- address r2 review comments ([`a41ccef`](https://github.com/Arrow-air/svc-storage/commit/a41ccef3f447a4348a07d7843e4765e9bcc387e0))

### 📚 Documentation

-  **readme:** add license notice and additional info ([`623ee9f`](https://github.com/Arrow-air/svc-storage/commit/623ee9f59721365eb30db3128e4d488efaceb35d))
-  **rust:** improve server rust-doc ([`54024fb`](https://github.com/Arrow-air/svc-storage/commit/54024fb5e722f7698458e6b4a2b0c76d80c8eacb))
-  **rust:** improve grpc client rust-doc ([`773886e`](https://github.com/Arrow-air/svc-storage/commit/773886e97c1853d050bf26f85fa509bff42ff973))
-  **sdd:** update itinerary model ([`01fc188`](https://github.com/Arrow-air/svc-storage/commit/01fc1887a3a9ed493de49ad0f57dc1b410cbdac0))
- update docs to reflect latest changes ([`86d49c1`](https://github.com/Arrow-air/svc-storage/commit/86d49c1519d1776f38a1ff41f54a297f4ad9efa0))

## [Release 0.9.0-develop.1](https://github.com/Arrow-air/svc-storage/releases/tag/v0.9.0-develop.1)

### 🛠 Maintenance

-  **ci:** license - provisioned by terraform ([`9e4b1ab`](https://github.com/Arrow-air/svc-storage/commit/9e4b1ab230ad43d64484407d16367d11cb35f219))
-  **ci:** .env.base - provisioned by terraform ([`55a95a3`](https://github.com/Arrow-air/svc-storage/commit/55a95a3a5a787d0b0746bb1be029fbb018aea2b0))
-  **ci:** .make/rust.mk - provisioned by terraform ([`c16804c`](https://github.com/Arrow-air/svc-storage/commit/c16804c4881bd8c3a6340c93e096177b16d5e324))
-  **checks:** sanity check fixes ([`003554f`](https://github.com/Arrow-air/svc-storage/commit/003554ff83ea26f4d9302b5edd39f482c50ac81d))

### 📚 Documentation

-  **readme:** add license notice and additional info ([`43ab80d`](https://github.com/Arrow-air/svc-storage/commit/43ab80d2db71bb00bc8a6a87a39bc8ecd30e595d))

## [Release 0.9.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.9.0-develop.0)

### ✨ Features

-  **server:** tonic-health for grpc health checks ([`de70b1f`](https://github.com/Arrow-air/svc-storage/commit/de70b1f2fa3b5b297fcc38cc1ea0afc67e045162))
-  **backend:** add cockroachdb backend ([`d6e54ca`](https://github.com/Arrow-air/svc-storage/commit/d6e54ca576fd63316e61d90a9e768689a21e04c2))
-  **backend:** client certs for CockroachDB auth ([`b7ff562`](https://github.com/Arrow-air/svc-storage/commit/b7ff562b6a5bff35f7f3c8bd465c5e55cd54bf84))
-  **flight_plan:** add grpc and psql resource ([`65f1dda`](https://github.com/Arrow-air/svc-storage/commit/65f1ddade9ad6a1974c60f211584fe0e11d3deef))
-  **vertipad:** add grpc and psql resources (#13) ([`0e6c773`](https://github.com/Arrow-air/svc-storage/commit/0e6c77376dc82185742a012e868dca21ee7adcfb))
-  **vertiport:** add grpc and psql resources (#12) ([`d572b57`](https://github.com/Arrow-air/svc-storage/commit/d572b57ed272733112d690b14c15fb887179b710))

### 🐛 Fixes

- remove lib-router dependency (#21) ([`607d58b`](https://github.com/Arrow-air/svc-storage/commit/607d58b306a69d15c83f84490eaee050a4ca1587))

### 🛠 Maintenance

-  **init:** initial repository setup ([`1cb300c`](https://github.com/Arrow-air/svc-storage/commit/1cb300c75ab16ee38d8c95328e8fb980c0010ee8))
-  **setup:** initial modules and code ([`7040bb5`](https://github.com/Arrow-air/svc-storage/commit/7040bb558c0ed9773799a41c7b1898630d45240a))
- fix service Cargo versions for release ([`c036dcc`](https://github.com/Arrow-air/svc-storage/commit/c036dccc7c2ba50f10286040d22148eb2bb2ac3f))
-  **ci:** .make/env.mk - provisioned by terraform ([`9094c00`](https://github.com/Arrow-air/svc-storage/commit/9094c00285a58166e7b7decc0e8fbe7295f1768b))
-  **ci:** .editorconfig - provisioned by terraform ([`0cb2357`](https://github.com/Arrow-air/svc-storage/commit/0cb2357ff041f7c77444e09b1af3e132a0136fb8))
-  **ci:** .gitignore - provisioned by terraform ([`c1df37d`](https://github.com/Arrow-air/svc-storage/commit/c1df37d64d8689c65e75f830a4a055eec92c042e))
-  **ci:** .env.base - provisioned by terraform ([`f823d31`](https://github.com/Arrow-air/svc-storage/commit/f823d3165233093c09f5051517db3c5726935cfb))
-  **ci:** .make/rust.mk - provisioned by terraform ([`181f467`](https://github.com/Arrow-air/svc-storage/commit/181f4670cae57a0c45bd3755d127083b5d0132e9))
-  **ci:** .github/workflows/release.yml - provisioned by terraform ([`152a754`](https://github.com/Arrow-air/svc-storage/commit/152a754a85c3d32822f682fa18007a2696477ed9))
-  **ci:** .license - provisioned by terraform ([`1a38893`](https://github.com/Arrow-air/svc-storage/commit/1a3889300f3b24c10c08d14f1777f1ecbb7f0d0d))
-  **ci:** .github/workflows/nightly.yml - provisioned by terraform ([`3d2b472`](https://github.com/Arrow-air/svc-storage/commit/3d2b472a86c55de994e5f21d2eec4a2d69024ad9))

### 📚 Documentation

-  **icd:** adding ICD documentation ([`206f549`](https://github.com/Arrow-air/svc-storage/commit/206f549cd44b44e01f84cf88773fb459ba3055cb))
-  **sdd:** adds SDD ([`8c64265`](https://github.com/Arrow-air/svc-storage/commit/8c64265d9682af8f13451e55ed3d38566532d0d2))
-  **readme:** add section to troubleshoot macos tls ([`735ed72`](https://github.com/Arrow-air/svc-storage/commit/735ed7243faa207a637d38bc16e9b46ccffb1b97))
-  **readme:** fixing make targets in README.md ([`430bc8c`](https://github.com/Arrow-air/svc-storage/commit/430bc8c67f47e2df7f2297e1848774bf8cdf7d60))

## [Release 0.2.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.2.0)

### ✨ Features

-  **server:** tonic-health for grpc health checks ([`de70b1f`](https://github.com/Arrow-air/svc-storage/commit/de70b1f2fa3b5b297fcc38cc1ea0afc67e045162))
-  **backend:** add cockroachdb backend ([`d6e54ca`](https://github.com/Arrow-air/svc-storage/commit/d6e54ca576fd63316e61d90a9e768689a21e04c2))
-  **backend:** client certs for CockroachDB auth ([`b7ff562`](https://github.com/Arrow-air/svc-storage/commit/b7ff562b6a5bff35f7f3c8bd465c5e55cd54bf84))
-  **flight_plan:** add grpc and psql resource ([`65f1dda`](https://github.com/Arrow-air/svc-storage/commit/65f1ddade9ad6a1974c60f211584fe0e11d3deef))
-  **vertipad:** add grpc and psql resources (#13) ([`0e6c773`](https://github.com/Arrow-air/svc-storage/commit/0e6c77376dc82185742a012e868dca21ee7adcfb))
-  **vertiport:** add grpc and psql resources (#12) ([`d572b57`](https://github.com/Arrow-air/svc-storage/commit/d572b57ed272733112d690b14c15fb887179b710))

### 🐛 Fixes

- remove lib-router dependency (#21) ([`607d58b`](https://github.com/Arrow-air/svc-storage/commit/607d58b306a69d15c83f84490eaee050a4ca1587))

### 🛠 Maintenance

-  **init:** initial repository setup ([`1cb300c`](https://github.com/Arrow-air/svc-storage/commit/1cb300c75ab16ee38d8c95328e8fb980c0010ee8))
-  **setup:** initial modules and code ([`7040bb5`](https://github.com/Arrow-air/svc-storage/commit/7040bb558c0ed9773799a41c7b1898630d45240a))
- fix service Cargo versions for release ([`c036dcc`](https://github.com/Arrow-air/svc-storage/commit/c036dccc7c2ba50f10286040d22148eb2bb2ac3f))

### 📚 Documentation

-  **icd:** adding ICD documentation ([`206f549`](https://github.com/Arrow-air/svc-storage/commit/206f549cd44b44e01f84cf88773fb459ba3055cb))
-  **sdd:** adds SDD ([`8c64265`](https://github.com/Arrow-air/svc-storage/commit/8c64265d9682af8f13451e55ed3d38566532d0d2))
-  **readme:** add section to troubleshoot macos tls ([`735ed72`](https://github.com/Arrow-air/svc-storage/commit/735ed7243faa207a637d38bc16e9b46ccffb1b97))
-  **readme:** fixing make targets in README.md ([`430bc8c`](https://github.com/Arrow-air/svc-storage/commit/430bc8c67f47e2df7f2297e1848774bf8cdf7d60))
