# PlumBlossom
[Design Document](./DESIGN_DOC.md)

Plum Blossom is a CPU Emulator designed from the ground up to be easy to implement and relatively quick. Along side it, there is a compiler that compiles a minimal assembly language into bytes so that test code can be made much more human readable. 

It is reccomended to add the executables to your path when developing for the PlumBlossom.
Nushell Example:
```nu
alias emcc = ~/git/PlumBlossom/compiler/target/release/compiler
alias emx32 = ~/git/PlumBlossom/em-x32/target/release/em-x32
```
