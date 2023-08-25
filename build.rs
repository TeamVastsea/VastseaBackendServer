extern crate embed_resource;

fn main() -> shadow_rs::SdResult<()> {
    embed_resource::compile("icon.rc", embed_resource::NONE);
    shadow_rs::new()
}