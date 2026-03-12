# No-emoji

I hate how LLMs insist on putting non-ascii characters in comments and strings.
This is a super simple pre-commit hook  that rejects changes containing non-ascii characters.

Ideally the specifics of the valid/invalid character sets would be configurable, but for now it's
just based on whatever the rust [std::Char::is_ascii()](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii)
function does.


## Installation
Put a file like this in your repo's `.pre-commit-config.yaml`:
```yaml
FIXME
```
