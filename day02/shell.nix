with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day1";
  buildInputs = [ cargo rustc rustfmt ];
}
