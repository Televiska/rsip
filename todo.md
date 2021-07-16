# To Do
* I need to create a nom func for token:
```
token       =  1*(alphanum / "-" / "." / "!" / "%" / "*"
                     / "_" / "+" / "`" / "'" / "~" )
```
Usefull to parse display names
* convert all TryInto for Tokenizer to TryFrom<Tokenizer> for Type
* generalize all common Tokenizers around AbstractInput trait
  * this is needed in cases where we already have utf8 Strings and we want to parse them (like in headers)
