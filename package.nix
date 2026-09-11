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
  version = "0.1.1";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
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
