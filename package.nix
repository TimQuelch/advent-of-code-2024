{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "advent-of-code-2024";
  version = "0.1.0";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
