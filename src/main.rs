mod global;
mod loader;

use deno_core::error::AnyError;
use deno_core::extension;
use std::rc::Rc;

extension! {
    engine,
    ops = [
        global::set_timeout
    ]
}

async fn execute(entrypoint: &str) -> Result<(), AnyError> {
    let main = deno_core::resolve_path(entrypoint, std::env::current_dir()?.as_path())?;

    let mut jsrt = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(loader::TsModuleLoader)),
        extensions: vec![engine::init_ops()],
        ..Default::default()
    });

    jsrt.execute_script("[jsrt:rt.js]", include_str!("./runtime/embed.js"))
        .unwrap();

    let id = jsrt.load_main_es_module(&main).await?;
    let res = jsrt.mod_evaluate(id);

    jsrt.run_event_loop(Default::default()).await?;

    res.await
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    if let Err(error) = rt.block_on(execute("./example/index.ts")) {
        eprintln!("error: {}", error);
    }
}
