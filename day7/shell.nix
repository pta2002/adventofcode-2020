with (import <nixpkgs> {});
mkShell {
  name = "aoc20-day7";
  buildInputs = [ cargo rustc ];
}
