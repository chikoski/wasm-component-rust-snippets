use std::sync::Arc;
use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

pub struct WasiViewImpl {
    context: Arc<WasiCtx>,
    table: Arc<Table>,
}

impl WasiViewImpl{
    pub fn new() -> WasiViewImpl {
        let context = WasiCtxBuilder::new().inherit_stdout().build();
        let table = Table::new();
        WasiViewImpl {
            context: Arc::new(context),
            table: Arc::new(table),
        }
    }

}

impl WasiView for WasiViewImpl {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        Arc::get_mut(&mut self.table).expect("Mutable table should be acquired.")
    }

    fn ctx(&self) -> &WasiCtx {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        Arc::get_mut(&mut self.context).expect("A mutable context should be acquired.")
    }
}