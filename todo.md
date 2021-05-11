# To Do
* I need to create a nom func for token:
```
token       =  1*(alphanum / "-" / "." / "!" / "%" / "*"
                     / "_" / "+" / "`" / "'" / "~" )
```
Usefull to parse display names
* ~~in types that are optional, we need to make sure to fail as soon as possible
for instance in schema we should fail as soon as we find a char that is not alpha.
maybe permutation(take_while(alpha), take_until(":"))~~
* ~~in URI: do alt(((tag_no_case("sip:"), tag_no_case("sips:"), ((take_until("://"), tag("://"))))~~
* expand the idea of Utf8Tokenizer
* remove some tech debt around organization, b"".as_ref() -> as_bytes etc
* now we have impl only TryInto for Tokenizer, but we should impl instead TryFrom<Tokenizer>

ASAP TODO:
* expand tokenizer in other uri params, probably through a macro
* add tests for ^^ and add parser + tests for ^^
* remove smart moves from tokenizers, will be more reusable
* continue on display traits
* via url is very simple, no params or headers, need to take special case on that
