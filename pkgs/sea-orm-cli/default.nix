{ lib, fetchCrate, rustPlatform, openssl, pkg-config }:
rustPlatform.buildRustPackage rec {
    pname = "sea-orm-cli";
    version = "0.11.0";

    src = fetchCrate {
        inherit pname version;
        sha256 = "sha256-nFSouV23t0+CCvBXT/WhaTGoFtABnt9KjF9U0kb+zqI=";
    };

    nativeBuildInputs = [ pkg-config ];
    buildInputs = [ openssl ];

    cargoSha256 = "sha256-qJ+G5+KyoNsl3VKsHDdFD6l1MagqHd7N7TahK7B5ky8=";

    meta = with lib; {
        description = "Rapidly scaffold out a new tauri app project.";
        homepage = "https://github.com/tauri-apps/create-tauri-app";
        license = with licenses; [ mit asl20 ];
    };
}