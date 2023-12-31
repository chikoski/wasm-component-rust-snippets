use wasmtime_wasi::preview2::{Table, WasiCtx, WasiCtxBuilder, WasiView};

pub struct WasiViewImpl {
    context: WasiCtx,
    table: Table,
}

impl WasiViewImpl {
    pub fn new() -> WasiViewImpl {
        let context = WasiCtxBuilder::new().inherit_stdout().build();
        let table = Table::new();
        WasiViewImpl {
            context,
            table,
        }
    }
}

impl WasiView for WasiViewImpl {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.context
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.context
    }
}