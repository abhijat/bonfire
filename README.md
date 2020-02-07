#### What this tool does

It helps converts [marshmallow](https://github.com/marshmallow-code/marshmallow) schema definitions to [json-schema](http://json-schema.org/) by reducing some of the boilerplate work.

#### How to run it

Point it to a file containing marshmallow schema definitions. It will print out the corresponding JSON schema.

```shell script
~/dev/rust/bonfire  (master) 
 abhijat $ ./target/debug/bonfire ./data/schemas.py 
{"name":"foo-schema","properties":{"key":{"type":"string"},"type":{"type":"string"},"value":{"type":"string"}}}
{"name":"bar-schema","properties":{"method":{"type":"string"},"parameters":{"type":"array"},"session_key":{"type":"string"},"type":{"type":"string"},"url":{"type":"string"}}}
{"name":"abc-def-schema","properties":{"d":{"type":"object"},"request":{"type":"nested"},"st":{"type":"string"},"tenant_id":{"type":"tenantid"},"user_id":{"type":"int"}}}
{"name":"some-important-stuff","properties":{"rdd":{"type":"string"},"rtt":{"type":"string"},"sc":{"type":"int"}}}
{"name":"mega-important-schema","properties":{"afarg":{"type":"object"},"file":{"type":"string"},"fn":{"type":"string"},"lno":{"type":"int"},"turtles":{"type":"string"}}}
{"name":"some-big-events-schema","properties":{"family":{"type":"string"},"neighborhood":{"type":"string"},"pincode":{"type":"string"}}}
{"name":"event-schema","properties":{"addresses":{"type":"nested"},"bus_size":{"type":"string"},"data":{"type":"object"},"timestamp":{"type":"datetime"}}}
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


#### Help

```shell script
~/dev/rust/bonfire  (master) 
 abhijat $ ./target/debug/bonfire --help
schema-translator 0.1.0
A small utility to convert marshmallow schema to json-schema

USAGE:
    bonfire [FLAGS] <roast>

FLAGS:
    -h, --help        Prints help information
    -p, --prettify    
    -V, --version     Prints version information

ARGS:
    <roast>    

```
