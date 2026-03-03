use wasm_bindgen::prelude::*;
mod renderer;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    renderer::run().await?;

    Ok(())
}