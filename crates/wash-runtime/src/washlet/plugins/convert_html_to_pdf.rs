use std::{
    collections::{HashMap, HashSet},
    io::Write,
};
use std::process::Stdio;
use wasmtime::component::HasSelf;

use crate::{
    engine::{ctx::Ctx, workload::WorkloadComponent},
    plugin::HostPlugin,
    wit::{WitInterface, WitWorld},
};

mod bindings {
    wasmtime::component::bindgen!({
        world: "convert-html-to-pdf-world",
        imports: { default: async | trappable },
    });
}

use bindings::wasmcloud::runtime::convert_html_to_pdf::Host;

pub struct ConvertHtmlToPdf;

impl Host for Ctx {
    async fn convert_html_to_pdf(&mut self, html: String) -> anyhow::Result<Vec<u8>> {
        let mut child =
            std::process::Command::new("wkhtmltopdf")
                .args(["-", "-"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

        let mut stdin = child.stdin.take()?;
        stdin.write_all(html.as_bytes())?;

        let output = child.wait_with_output()?;

        Ok(Ok(output.stdout.into()))
    }
}

#[async_trait::async_trait]
impl HostPlugin for ConvertHtmlToPdf {
    fn id(&self) -> &'static str {
        "convert-html-to-pdf-host-plugin"
    }

    fn world(&self) -> WitWorld {
        let mut exports = HashSet::new();
        let mut interfaces = HashSet::new();
        interfaces.insert(String::from("convert-html-to-pdf"));

        exports.insert(WitInterface {
            namespace: String::from("wasmcloud"),
            package: String::from("runtime"),
            interfaces,
            version: Some(semver::Version::parse("0.1.0").unwrap()),
            config: HashMap::new()
        });

        WitWorld {
            imports: HashSet::new(),
            exports,
        }
    }

    async fn on_component_bind(
        &self,
        component_handle: &mut WorkloadComponent,
        interfaces: std::collections::HashSet<crate::wit::WitInterface>,
    ) -> anyhow::Result<()> {
        // Find the "wasmcloud:runtime/convert-html-to-pdf" interface, if present
        let Some(_) = interfaces.iter().find(|i| {
            i.namespace == "wasmcloud" && i.package == "runtime" && i.interfaces.contains("convert-html-to-pdf")
        }) else {
            return Ok(());
        };

        // Add `wasmcloud:runtime/convert-html-to-pdf` to the workload's linker
        bindings::wasmcloud::runtime::convert_html_to_pdf::add_to_linker::<_, HasSelf<Ctx>>(
            component_handle.linker(),
            |ctx| ctx,
        )?;

        Ok(())
    }
}
