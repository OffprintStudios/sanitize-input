//! A native module for sanitizing user inputs and QuillJS Deltas.

mod delta;

use neon::prelude::*;

use ammonia::clean;

fn sanitize_input(mut ctx: FunctionContext) -> JsResult<JsString> {
    let input_text = ctx.argument::<JsString>(0)?.value();
    Ok(ctx.string(clean(&input_text)))
}

register_module!(mut cx, {
    cx.export_function("sanitizeInput", sanitize_input)?;
    Ok(())
});
