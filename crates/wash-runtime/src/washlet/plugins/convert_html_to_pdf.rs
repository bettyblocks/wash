use std::{
    collections::{HashMap, HashSet},
};
use wasmtime::component::HasSelf;
use headless_chrome::{Browser, types::PrintToPdfOptions};
use base64::Engine;

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
    // wasmCloud already wraps our function in an anyhow::Result.
    async fn convert_html_to_pdf(&mut self, html: String) -> anyhow::Result<Result<Vec<u8>, String>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        let base64_encoded_html = base64::engine::general_purpose::STANDARD.encode(html.as_bytes());
        tab.navigate_to(format!("data:text/html;base64,{}", base64_encoded_html).as_str())?;
        tab.wait_until_navigated()?;

        Ok(Ok(
                /*
            tab.print_to_pdf(Some(
                    PrintToPdfOptions {
                        landscape: None,
                        display_header_footer: Some(true),
                        print_background: Some(true),
                        scale: None,
                        paper_width: None,
                        paper_height: None,
                        margin_top: None,
                        margin_bottom: None,
                        margin_left: None,
                        margin_right: None,
                        page_ranges: Some(String::from("1-10")),
                        ignore_invalid_page_ranges: None,
                        header_template: None,
                        footer_template: None,
                        prefer_css_page_size: None,
                        transfer_mode: None,
                        generate_document_outline: None,
                        generate_tagged_pdf: None,
                    })
            )?
                */
            tab.print_to_pdf(None)?
        ))
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
