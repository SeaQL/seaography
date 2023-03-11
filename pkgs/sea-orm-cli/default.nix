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
        description = "Command line utility for SeaORM";
        homepage = "https://github.com/SeaQL/sea-orm/tree/master/sea-orm-cli";
        license = with licenses; [ mit asl20 ];
    };
}