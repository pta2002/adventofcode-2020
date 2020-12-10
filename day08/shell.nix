with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day8";
  buildInputs = [ cargo rustc ];
}
