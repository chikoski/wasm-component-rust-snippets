use std::ops::{Deref, DerefMut};
use wasmtime_wasi::preview1::WasiP1Ctx;
use wasmtime_wasi::WasiCtxBuilder;

pub struct Preview1Host {
    wasi_ctx: WasiP1Ctx
}

impl Deref for Preview1Host {
    type Target = WasiP1Ctx;

    fn deref(&self) -> &Self::Target {
        &self.wasi_ctx
    }
}

impl Default for Preview1Host {
    fn default() -> Self {
        let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build_p1();
        Self { wasi_ctx }
    }
}

impl DerefMut for Preview1Host {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wasi_ctx
    }
}