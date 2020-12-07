with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day6";
  buildInputs = [ cargo rustc ];
}
