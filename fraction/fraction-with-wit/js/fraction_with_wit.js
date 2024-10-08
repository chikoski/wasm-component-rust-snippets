let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

const instantiateCore = WebAssembly.instantiate;

function toInt32(val) {
  return val >> 0;
}


let exports0;
let memory0;

function _new(arg0, arg1) {
  const ret = exports0['component:fraction-with-wit/fraction#new'](toInt32(arg0), toInt32(arg1));
  return {
    numerator: dataView(memory0).getInt32(ret + 0, true),
    denominator: dataView(memory0).getInt32(ret + 4, true),
  };
}

const $init = (() => {
  let gen = (function* init () {
    const module0 = fetchCompile(new URL('./fraction_with_wit.core.wasm', import.meta.url));
    ({ exports: exports0 } = yield instantiateCore(yield module0));
    memory0 = exports0.memory;
  })();
  let promise, resolve, reject;
  function runNext (value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject = _reject));
      value.then(runNext, reject);
    }
    catch (e) {
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
})();

await $init;
const fraction = {
  'new': _new,
  
};

export { fraction, fraction as 'component:fraction-with-wit/fraction',  }