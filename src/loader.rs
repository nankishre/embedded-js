// TODO: Errors aren't really useful yet. Add stack, traces.

use deno_ast::{MediaType, ParseParams};
use deno_core::{error::AnyError, ModuleLoadResponse, ModuleSource, ModuleSourceCode, ModuleType};

pub struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, AnyError> {
        //if specifier.starts_with("file://") {
        deno_core::resolve_import(specifier, referrer).map_err(|e| e.into())
        //} else {
        // TODO: read package.json and resolve import

        //   Err(AnyError::msg("Failed to load module!"))
        // }
    }

    fn load(
        &self,
        module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();

        let module_load = Box::pin(async move {
            let path = module_specifier
                .to_file_path()
                .map_err(|_| AnyError::msg("Invalid file path"))?;

            let (module_type, should_transpile) =
                match path.extension().and_then(|ext| ext.to_str()) {
                    Some("ts") => (ModuleType::JavaScript, true),
                    Some("js") => (ModuleType::JavaScript, false),
                    Some("json") => (ModuleType::Json, false),
                    _ => {
                        return Err(AnyError::msg(format!(
                            "Unsupported file extension for file: {:?}.",
                            path.to_str().as_slice().concat()
                        )))
                    }
                };

            let code = match std::fs::read_to_string(&path) {
                Ok(r) => r,
                Err(_) => {
                    return Err(AnyError::msg(format!(
                        "Failed to load: {:?}. Does the file exist?",
                        path.to_str().as_slice().concat(),
                    )))
                }
            };
            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.clone(),
                    text: code.into(),
                    media_type: MediaType::TypeScript,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })?;

                let res = parsed
                    .transpile(&Default::default(), &Default::default())?
                    .into_source();

                String::from_utf8(res.source)?
            } else {
                code
            };

            let module = ModuleSource::new(
                module_type,
                ModuleSourceCode::String(code.into()),
                &module_specifier,
                None,
            );
            Ok(module)
        });

        ModuleLoadResponse::Async(module_load)
    }
}
