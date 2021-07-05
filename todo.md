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
* convert all TryInto for Tokenizer to TryFrom<Tokenizer> for Type
* generalize all Tokenizers around AbstractInput trait

* break the UntypedHeader trait to 2, one for plain Untyped and one for Untyped
  headers that have typed
* also Debug trait (to_string) is in form `Header: Value..` and From<String> for Header is `Value..`
  sounds like a problem, in combination with ::new() that accepts impl Into<String>.
* move the typed headers to different folder maybe
* move tokenizers to different files
* ~~ease mutability of headers (in typed headers via_header.uri requires a move,
  maybe add a uri_ref() methods ?)~~
* ~~start using ? in tests in viska to avoid lengthy expects~~
* ~~add mut helpers in Uri and headers that include uri (from/to/via/contact etc)
  also mut helpers for headers that include params (from/to/via/contact etc)~~
* ~~check again that viska adds correct params to header params and not uri params~~


* ~~fix typed to clone as well~~
* ~~fix status code to use macros + be able to do 401.into()~~

More:
* ~~remove smart moves from tokenizers, will be more reusable~~
* ~~via url is very simple, no params or headers, need to take special case on that~~
