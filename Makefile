.PHONY: generate
generate: openapi-generator generate -g rust -i ./openapi/na_bambora_v1.yaml -o ./crates/openapi --additional-properties=packageName=opeanapi,packageVersion=0.0.1