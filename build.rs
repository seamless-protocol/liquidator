fn main() {
    cynic_codegen::register_schema("aave_v3")
        .from_sdl_file("src/graphql/aave_v3_schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}