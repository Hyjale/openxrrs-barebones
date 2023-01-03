use xrrs::{ app::App };

#[cfg_attr(target_os = "android", ndk_glue::main)]
fn main() {
    let app = App::new()
    .run();
}
