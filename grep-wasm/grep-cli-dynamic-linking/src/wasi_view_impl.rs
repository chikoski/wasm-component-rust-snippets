use std::sync::Arc;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};
use crate::app_config::AppConfig;

pub struct WasiViewImpl {
    table: Arc<Table>,
    context: Arc<WasiCtx>
}

impl WasiView for WasiViewImpl {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        Arc::get_mut(&mut self.table).expect("table should become mutable here.")
    }

    fn ctx(&self) -> &WasiCtx {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        Arc::get_mut(&mut self.context).expect("WasiCtX should become mutable here.")
    }
}

impl WasiViewImpl {
    pub fn new(config: &AppConfig) -> WasiViewImpl {
        let context = WasiCtxBuilder::new().inherit_stdio().args(&config.files).build();
        let table = Table::new();
        WasiViewImpl {
            table: Arc::new(table),
            context: Arc::new(context),
        }
    }
}
