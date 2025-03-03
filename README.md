# Arithmetic coding in Rust
A simple arithmetic coder implementation that should be easy to read. Includes a driver binary and a library. By default a binary is built.
## Encoding from the command line
Run `target/release/simple-arithmetic-coding -e <input from stdin>` or wherever you have compiled the binary. For example on Linux, you can pipe your file using cat, and then direct the output to file: `cat war_and_peace.txt | target/release/simple-arithmetic-coding -e > output.bin`
## Decoding from the command line
Similarly to encoding, run `target/release/simple-arithmetic-coding -d <input from stdin>`. Only the command line switch is different.
## As a lib
The library version provides two functions and two iterators. The functions are very much self-explanatory: there is the `encoding_routine` and the `decoding_routine`. Their respective argument names are self-documenting and, with an IDE or a text editor, you can see their trait bounds. The iterators work similarly, except that they only need to wrap inputs. The result that you get is an iterator over `u8`s for both iterators.
## Updates in version 0.2.1
- Fixed a silly bug regarding subtraction overflow.
- Added some smart integration tests.
- Added a trivial example.
## Useful notes
- The encoder introduces noticeable drift with certain inputs where the same byte repeats at least a little over 187,000 times.
- War and Peace from Project Gutenberg does not include a single zero byte.
