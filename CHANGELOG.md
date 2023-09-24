## [Release 0.11.0](https://github.com/Arrow-air/svc-storage/releases/tag/v0.11.0)

### ✨ Features

- add feature flags for clients ([`3f7493b`](https://github.com/Arrow-air/svc-storage/commit/3f7493b401f3e66688e0ff258ab112f9973d5ecd))
- add parcel resource ([`f5ce584`](https://github.com/Arrow-air/svc-storage/commit/f5ce584884a27561e8b685d44e795cade1f935eb))
- add parcel implementation ([`4bfbabe`](https://github.com/Arrow-air/svc-storage/commit/4bfbabe29ec28964d91a14f889bf3ad3c02360a2))
- add owner_id to parcel ([`4d195a5`](https://github.com/Arrow-air/svc-storage/commit/4d195a5de8bbca59e598f3f5d745c5df9c05b7fb))
- add parcel to postgres init ([`4c08f86`](https://github.com/Arrow-air/svc-storage/commit/4c08f868a470151ce8c46f2829039240bfa19cfa))
- add scanner type ([`21543b1`](https://github.com/Arrow-air/svc-storage/commit/21543b1126a9fa2005a86d9ce5dbc873012c9257))
- implement geometry types ([`8c245bf`](https://github.com/Arrow-air/svc-storage/commit/8c245bf488a9415cf844dcc88f45e98f36a0b2c6))
- add scanner type to grpc client ([`affc6bb`](https://github.com/Arrow-air/svc-storage/commit/affc6bb5e6f15fb0532dcffa30da10411529bfcd))
- add parcel_scan table ([`65805f9`](https://github.com/Arrow-air/svc-storage/commit/65805f9b0b0bdbfced26871bfd17ac97e30c814a))
-  **user:** add new user resource ([`0dd8d50`](https://github.com/Arrow-air/svc-storage/commit/0dd8d50f8f119c2a8229c39d312abeeba0d5bd35))
-  **group:** add new group resource ([`ffa0186`](https://github.com/Arrow-air/svc-storage/commit/ffa0186a9fd9b1e832b12823a629f06887547b02))
-  **user_group:** add new user_group resource ([`2fe31d5`](https://github.com/Arrow-air/svc-storage/commit/2fe31d58bcbca9373919246d959a9f4783a31f24))
-  **group_user:** add new group_user resource ([`52fd0b2`](https://github.com/Arrow-air/svc-storage/commit/52fd0b26f1aba4ed88f3445853c68ab5fa239488))
-  **grpc-client:** use serializable Timestamp ([`37d509b`](https://github.com/Arrow-air/svc-storage/commit/37d509bd21b8395fbd16c226c5120d7f3622e6f8))
- add path field to flight plan ([`74be4ee`](https://github.com/Arrow-air/svc-storage/commit/74be4ee82ac9a7cba7b0d465cbcc381aabd9611f))
- add option to mark fields as read_only ([`07daa18`](https://github.com/Arrow-air/svc-storage/commit/07daa18e8d2830078d81e2cea2743fb5a8df2a56))
-  **vertiport:** expose created_at and updated_at as read_only ([`682b45d`](https://github.com/Arrow-air/svc-storage/commit/682b45dfb20d8e078a1dc2dace5942428447a28a))
-  **vehicle:** expose created_at and updated_at as read_only ([`68a7b6b`](https://github.com/Arrow-air/svc-storage/commit/68a7b6bc0f7b7f7f3376422bf507ef5f1cb971f4))
-  **vertipad:** expose created_at and updated_at as read_only ([`84949fd`](https://github.com/Arrow-air/svc-storage/commit/84949fd4cb39ed82086c99f62d23eb54b8b22b07))
- add search mock function for stub features ([`99bbe1c`](https://github.com/Arrow-air/svc-storage/commit/99bbe1c1ca05275bdb3d9e0847403d78122ca5ac))
- add support for linked table simple service ([`2e98aac`](https://github.com/Arrow-air/svc-storage/commit/2e98aac3947a6b7643ff5d53bb4e1d10b1fab413))
- add flight plan parcel link table ([`4ec0751`](https://github.com/Arrow-air/svc-storage/commit/4ec0751309837bf794615eb3f488e0ff4d097bb7))
- add created_at field to parcel_scan Data ([`9107b98`](https://github.com/Arrow-air/svc-storage/commit/9107b98b74c6c3d011391ccbdc0609ec1e747c4d))
- add is_ready call for all resources ([`cfac135`](https://github.com/Arrow-air/svc-storage/commit/cfac1350b822912fa9c7c90fe99601fe468e6969))

### 🐛 Fixes

- flight_plan mock data including tests ([`c546a3f`](https://github.com/Arrow-air/svc-storage/commit/c546a3f3b73e450596eeb1369e896d1674ab75d3))
- correct spellings and remove owner_id ([`040a7d8`](https://github.com/Arrow-air/svc-storage/commit/040a7d8240c82d98fe0c74686a6d562d098e8e63))
-  **dev:** log output and example target fixes ([`ac86ca9`](https://github.com/Arrow-air/svc-storage/commit/ac86ca94cfc59433a98164f8a12d920caac28985))
-  **cargo:** restructure dependencies and fix feature tests ([`b93c3e3`](https://github.com/Arrow-air/svc-storage/commit/b93c3e38c8039462da69cb16e9be413b6a77da82))
- deleted_at fields should not be mandatory ([`5653644`](https://github.com/Arrow-air/svc-storage/commit/5653644a0275c6814b897a9aeee2931dc9e0d2ff))
-  **flight_plan:** i64 field types should be u32 ([`df00526`](https://github.com/Arrow-air/svc-storage/commit/df00526c2950903b4c582c4f50ca98ca2c40c764))
- revert rust.mk to reach parity ([`bc841fd`](https://github.com/Arrow-air/svc-storage/commit/bc841fd2cde8162e64eb95462e1a0c0b6c0fa8c4))
- address pr comments ([`233fffc`](https://github.com/Arrow-air/svc-storage/commit/233fffc54eb3ac2d92aed3adb031b9957c83623f))
-  **cargo:** correcting features and clearify usage ([`cc58cfb`](https://github.com/Arrow-air/svc-storage/commit/cc58cfb604b1788e0dc2670f2bdb7e4d48197528))
-  **geo_types:** coords x,y correction to lat,long ([`9afa6e3`](https://github.com/Arrow-air/svc-storage/commit/9afa6e3c20cab99700e29e1d82b4278d73ab265d))
-  **client-grpc:** add empty Clients struct when no resource feature enabled ([`c9e771f`](https://github.com/Arrow-air/svc-storage/commit/c9e771f767437606a64c3ee0e35e83b39f423f92))
- use quoted columns for all queries ([`9295e26`](https://github.com/Arrow-air/svc-storage/commit/9295e2662e644f8356142a2b017c68f4060a0dcf))
-  **server:** postgres module improvements ([`0d613fb`](https://github.com/Arrow-air/svc-storage/commit/0d613fb06389775289d43b208d6b9df370b71d44))
-  **client-grpc:** use utoipa namespaced names for simple resource types ([`f02b869`](https://github.com/Arrow-air/svc-storage/commit/f02b869ecb3d1d718099be709af7830040a31694))
-  **grcp-geo-types:** add support for use with utoipa ([`f57c369`](https://github.com/Arrow-air/svc-storage/commit/f57c369b4527ff690f83003a83d656b77bc3d6bc))
- remove new_internal from parcel_scan created_at ([`b3d61c0`](https://github.com/Arrow-air/svc-storage/commit/b3d61c0f92757a7587a46080f5a0ff534733b119))
-  **parcel:** parcel related bug fixes ([`b91e6d9`](https://github.com/Arrow-air/svc-storage/commit/b91e6d910b020076df0e79b59fbc23b61c5927cc))

### 🔥 Refactorings

-  **client-grpc:** use lib-common traits and add mock features ([`f793c96`](https://github.com/Arrow-air/svc-storage/commit/f793c966a6dc455cd850621457875b924e71bc9b))
-  **server:** use lib-common traits and add mock features ([`52f17e8`](https://github.com/Arrow-air/svc-storage/commit/52f17e806a2ec1b77e9b2f7728f11e70927b2710))

### 🛠 Maintenance

- terraform provisioned file changes ([`1b3c3d0`](https://github.com/Arrow-air/svc-storage/commit/1b3c3d025a3756a56851975d986b38609c4fdd9c))
- remove obsolete function get_all_with_filter ([`ef26f77`](https://github.com/Arrow-air/svc-storage/commit/ef26f77331c1eade7eeca04ec924e25ef9d47b23))
- terraform provisioned file changes ([`03f98cc`](https://github.com/Arrow-air/svc-storage/commit/03f98cc4f52f40b086df72dec0253aeddfb10d8e))
- remove duplicate functionality enum from_str ([`75dee10`](https://github.com/Arrow-air/svc-storage/commit/75dee10b3bb7e4a89112471a3b56aa43a5b9acb9))
- expose geo_types to dependend services ([`7cdcecb`](https://github.com/Arrow-air/svc-storage/commit/7cdcecb67b257ff00705a3d0d9abba3b2cc5fca7))
- mock data should retain insert order ([`5168e92`](https://github.com/Arrow-air/svc-storage/commit/5168e92a02e03c7ff52178bc73f0921e807232a7))
- update lib common release tag ([`1015faf`](https://github.com/Arrow-air/svc-storage/commit/1015faf21787656f7397f6833aff91b2e848974d))
- update TODOs for new release ([`f94fa71`](https://github.com/Arrow-air/svc-storage/commit/f94fa71dc8d81be292936b12c72a66eb1bb7a3de))
- remove unneeded dependency ([`cebc20f`](https://github.com/Arrow-air/svc-storage/commit/cebc20f68115badef9c4f1ebd8b240f29ddf78a4))
- handle unwrap() calls in stub functions ([`82700b4`](https://github.com/Arrow-air/svc-storage/commit/82700b4af82b98c51b0688ae6e4187c64e5336a3))
- fix debug prefixes and punctuations ([`8be2f20`](https://github.com/Arrow-air/svc-storage/commit/8be2f20fc1a796c0b04609d7653df112874f901c))

### ✅ Tests

- adds loads of unit tests ([`1165c65`](https://github.com/Arrow-air/svc-storage/commit/1165c657d3c872e78ea186c0c2472b03589800f5))
-  **vertiport:** client-grpc integration test scenario ([`ec52171`](https://github.com/Arrow-air/svc-storage/commit/ec521712d175f0ef4ca7388e4e7e04100e502c7f))
-  **vertipad:** client-grpc integration test scenario ([`1cfa6db`](https://github.com/Arrow-air/svc-storage/commit/1cfa6dbcd72bcac6c9bb893214ece8cf0cffee2d))
-  **vehicle:** client-grpc integration test scenario ([`6f054e6`](https://github.com/Arrow-air/svc-storage/commit/6f054e6faa039b118aca81cbb7ff03059918e7d8))
-  **adsb:** client-grpc integration test scenario ([`609a36a`](https://github.com/Arrow-air/svc-storage/commit/609a36a068dcf70c85e1401523f57806de631fb3))
-  **client-grpc:** fix integration tests ([`1736fd7`](https://github.com/Arrow-air/svc-storage/commit/1736fd77d50b4d5177b7f77bb72486caeadf4fb2))
- add logging capabilities ([`9fccc39`](https://github.com/Arrow-air/svc-storage/commit/9fccc39926d719944222cddfdb4de2e68efd46b3))

### 📚 Documentation

-  **readme:** update README files badges ([`a5adc00`](https://github.com/Arrow-air/svc-storage/commit/a5adc00c7bd9fc1620101fd8620f8e9fd17128e8))
- update doc banners ([`38576d4`](https://github.com/Arrow-air/svc-storage/commit/38576d48a86d9ffe762a5649cc5c96894a436268))
-  **readme:** add new resource creation guide ([`e95c6d5`](https://github.com/Arrow-air/svc-storage/commit/e95c6d5b92e86f2c8c7967b23907a3cb6110183a))
-  **sdd:** update datamodel with latest resources ([`7efba42`](https://github.com/Arrow-air/svc-storage/commit/7efba429e254906a99a7526c7f7b3f48f7692836))
- fix headings and add icons ([`9c3453d`](https://github.com/Arrow-air/svc-storage/commit/9c3453d2d1086e150c2ae23b09cddad59ab59f38))
- fix rust-doc proto files ([`a2d6e42`](https://github.com/Arrow-air/svc-storage/commit/a2d6e42cb3fc3769a350db36031fc6adc8e8eba6))
-  **rust-doc:** provide rust-docs for Client traits ([`d98bf61`](https://github.com/Arrow-air/svc-storage/commit/d98bf61398e7f02360b1dfecc86a0e6fdae9e719))

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
