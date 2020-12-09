with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day9";
  buildInputs = [ cargo rustc ];
}
