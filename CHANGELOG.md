## [Release 0.7.0-develop.1](https://github.com/Arrow-air/svc-storage/releases/tag/v0.7.0-develop.1)

### 🛠 Maintenance

-  **ci:** .gitignore - provisioned by terraform ([`b9e1a69`](https://github.com/Arrow-air/svc-storage/commit/b9e1a690f3a2ecdac60ace877eff713b4236a32e))
-  **ci:** .make/rust.mk - provisioned by terraform ([`8ee4e90`](https://github.com/Arrow-air/svc-storage/commit/8ee4e905c13ba88b244e8a127e678c303f7a468f))

### 📚 Documentation

-  **sdd:** adds SDD ([`1a80d4b`](https://github.com/Arrow-air/svc-storage/commit/1a80d4b96b417845a2a0e87ebca2f3bc14dfdcdd))

## [Release 0.7.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.7.0-develop.0)

### ✨ Features

-  **vertipad:** add grpc and psql resources ([`c520f2d`](https://github.com/Arrow-air/svc-storage/commit/c520f2db67576f04887a080b4079f855bd36f0a3))
-  **example:** add vertipad scenario ([`7427e22`](https://github.com/Arrow-air/svc-storage/commit/7427e2221d22f1a87fda0bb308307aed0099f755))

### 🐛 Fixes

-  **flight_plan:** refactored return values and search ([`8f93cd4`](https://github.com/Arrow-air/svc-storage/commit/8f93cd419b82b7ab14c3aea3f68ed73bcaffa574))

## [Release 0.6.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.6.0-develop.0)

### ✨ Features

-  **psql:** add flight_plan psql functions ([`b067d7c`](https://github.com/Arrow-air/svc-storage/commit/b067d7c1f4683fdad8977360589a4cd8b16883ab))
-  **flight_plan:** psql <-> grpc conversions ([`6d4e026`](https://github.com/Arrow-air/svc-storage/commit/6d4e026542331b6cc551f5ba9af7ed75342cfb54))
-  **env:** add repo specific .env file ([`84597c4`](https://github.com/Arrow-air/svc-storage/commit/84597c46530eb531109e1a1aad3333dbe2cb9899))

## [Release 0.5.0-develop.1](https://github.com/Arrow-air/svc-storage/releases/tag/v0.5.0-develop.1)

### 📚 Documentation

-  **icd:** adding icd documentation ([`28de630`](https://github.com/Arrow-air/svc-storage/commit/28de63002ce33c2bbe779f2a54a520cb8565c0bf))
-  **icd:** added links and note about APIs ([`fd8adfc`](https://github.com/Arrow-air/svc-storage/commit/fd8adfc5cf128bc1c4bf3081184147d5b5f76c30))

## [Release 0.5.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.5.0-develop.0)

### ✨ Features

-  **ssl:** use client certificates for auth ([`e10161e`](https://github.com/Arrow-air/svc-storage/commit/e10161eb54bb47d7cec7056515198789b44747aa))

## [Release 0.4.0-develop.2](https://github.com/Arrow-air/svc-storage/releases/tag/v0.4.0-develop.2)

### 🛠 Maintenance

-  **ssl:** fixed SSL breaking in docker ([`9f21e56`](https://github.com/Arrow-air/svc-storage/commit/9f21e565334c3918f6300d5d3d9d9203691db84d))

## [Release 0.4.0-develop.1](https://github.com/Arrow-air/svc-storage/releases/tag/v0.4.0-develop.1)

### 🛠 Maintenance

-  **ci:** publish dry run for now ([`f89a9f9`](https://github.com/Arrow-air/svc-storage/commit/f89a9f9ce8e8d69a6996774ee1f5ba1f8f7ba84e))

## [Release 0.4.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.4.0-develop.0)

### ✨ Features

- added sample data and vertiport/pad client ([`42a4b6d`](https://github.com/Arrow-air/svc-storage/commit/42a4b6d0c18778c8159af716954b523b27e5f038))

### 🛠 Maintenance

-  **ci:** fix release workflow ([`7961fb4`](https://github.com/Arrow-air/svc-storage/commit/7961fb425253c53c9076ea3c6ee763a17bea79f5))

## [Release 0.3.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.3.0-develop.0)

### ✨ Features

- add cockroachdb backend ([`280c05f`](https://github.com/Arrow-air/svc-storage/commit/280c05f4cb513c3a8bc8ac38477cb2859832c590))
-  **grpc:** implemented additional fields for grpc ([`e722fb1`](https://github.com/Arrow-air/svc-storage/commit/e722fb189299be5e2547cac1e5f4310503067828))
-  **vehicle:** add schedule field ([`19b1c05`](https://github.com/Arrow-air/svc-storage/commit/19b1c05d4968b29c5d7666442ee03421b90036a3))

### 🛠 Maintenance

-  **grpc:** split up resources ([`1d21efa`](https://github.com/Arrow-air/svc-storage/commit/1d21efae4f9299885c08585c0c93f3b12b2d81b4))

## [Release 0.2.0-develop.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.2.0-develop.0)

### ✨ Features

-  **server:** tonic-health for grpc health checks ([`10cab5c`](https://github.com/Arrow-air/svc-storage/commit/10cab5cd839c899deb084a0696de45b466636afe))

### 🐛 Fixes

-  **example:** use grpc health for example runs ([`5788963`](https://github.com/Arrow-air/svc-storage/commit/5788963b3ba944bfa66a96d66bf6b993cd8a95d1))

### 🛠 Maintenance

-  **proto:** rename proto file and package ([`94e0334`](https://github.com/Arrow-air/svc-storage/commit/94e03345dcb445fae4d1efbb3964eb9dd1cc9802))
-  **client:** rename client to client-grpc ([`7198591`](https://github.com/Arrow-air/svc-storage/commit/7198591255ec65fa8553072e51481f500ab44f7b))
- adding missing files + cleanup ([`204169e`](https://github.com/Arrow-air/svc-storage/commit/204169e3ccbf523991356f38a0b28701b852e3f6))
-  **changelog:** add CHANGELOG.md file ([`c8f06c4`](https://github.com/Arrow-air/svc-storage/commit/c8f06c415981a9e463d756ec98a939789de29b2d))
