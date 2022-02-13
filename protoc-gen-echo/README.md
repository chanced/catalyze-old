# protoc-gen-echo

protoc-gen-echo is a protoc plugin which simply echos the incoming `CodeGeneratorRequest` to stderr.

```bash
$ protoc -I proto --echo_out=. proto/example.proto

[echo start]
# CodeGeneratorRequest will be available here to read in.
[echo end]
```

## LICENSE

MIT or Apache 2.0. Pick whichever.
