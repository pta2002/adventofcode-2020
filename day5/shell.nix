with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day5";
  buildInputs = [ cargo rustc ];
}
