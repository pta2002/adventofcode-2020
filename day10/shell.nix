with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day10";
  buildInputs = [ cargo rustc ];
}
