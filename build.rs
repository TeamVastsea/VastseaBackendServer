extern crate embed_resource;

fn main() -> shadow_rs::SdResult<()> {
    embed_resource::compile("icon.rc");
    shadow_rs::new()
}