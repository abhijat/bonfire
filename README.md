#### What this tool does

It helps converts [marshmallow](https://github.com/marshmallow-code/marshmallow) schema definitions to [json-schema](http://json-schema.org/) by reducing some of the boilerplate work.

#### How to run it

Point it to a file containing marshmallow schema definitions. It will print out the corresponding JSON schema.

```shell script
~/dev/rust/bonfire  (master) 
 abhijat $ ./target/debug/bonfire ./data/schemas.py 
{"name":"foo-schema","properties":[{"type":{"type":"string"}},{"key":{"type":"string"}},{"value":{"type":"string"}}]}
{"name":"bar-schema","properties":[{"type":{"type":"string"}},{"session_key":{"type":"string"}},{"url":{"type":"string"}},{"method":{"type":"string"}},{"parameters":{"type":"array"}}]}
{"name":"abc-def-schema","properties":[{"st":{"type":"string"}},{"d":{"type":"object"}},{"user_id":{"type":"int"}},{"request":{"type":"nested"}},{"tenant_id":{"type":"tenantid"}}]}
{"name":"some-important-stuff","properties":[{"sc":{"type":"int"}},{"rtt":{"type":"string"}},{"rdd":{"type":"string"}}]}
{"name":"mega-important-schema","properties":[{"fn":{"type":"string"}},{"afarg":{"type":"object"}},{"file":{"type":"string"}},{"lno":{"type":"int"}},{"turtles":{"type":"string"}}]}
{"name":"some-big-events-schema","properties":[{"family":{"type":"string"}},{"neighborhood":{"type":"string"}},{"pincode":{"type":"string"}}]}
{"name":"event-schema","properties":[{"timestamp":{"type":"datetime"}},{"bus_size":{"type":"string"}},{"data":{"type":"object"}},{"addresses":{"type":"nested"}}]}
``` 

#### What it does not do (yet)

1. The output is very primitive. It does not contain the full schema with definition nodes etc.
2. Nested schema are not resolved yet.
3. Special types such as UUID are not resolved yet.
4. Union types (nullable strings etc) are not supported yet.

The output has to be changed for all of the above special cases.

The wrapping JSON objects also have to be created by hand. All of the above _can_ be done.

#### How it works

It uses the python parser created by the [RustPython](https://github.com/RustPython/RustPython) project to parse the AST and uses it to produce JSON schema.
