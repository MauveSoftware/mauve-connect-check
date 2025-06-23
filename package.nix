{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  darwin,
  stdenv,
}:

rustPlatform.buildRustPackage {
  pname = "mauve-connect-check";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoHash = "sha256-yyM7Cp+zro6plF8t5oeQZjfZGlR85JpFcwwkkXX6AB8=";

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs =
    [
      openssl
    ]
    ++ lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.SystemConfiguration
    ];

  meta = with lib; {
    description = "A command-line tool for verifying DNS configuration of domains in the Mauve Cloud infrastructure";
    homepage = "https://github.com/czerwonk/net-reduce";
    license = licenses.mit;
  };
}
