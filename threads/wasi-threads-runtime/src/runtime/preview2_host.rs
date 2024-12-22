use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct Preview2Host {
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

impl Default for Preview2Host {
    fn default() -> Self {
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let resource_table = ResourceTable::default();
        Self { wasi_ctx, resource_table }
    }
}

impl WasiView for Preview2Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}