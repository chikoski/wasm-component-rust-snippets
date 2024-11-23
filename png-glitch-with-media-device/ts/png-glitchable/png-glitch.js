const base64Compile = str => WebAssembly.compile(typeof Buffer !== 'undefined' ? Buffer.from(str, 'base64') : Uint8Array.from(atob(str), b => b.charCodeAt(0)));

function clampGuest(i, min, max) {
  if (i < min || i > max) throw new TypeError(`must be between ${min} and ${max}`);
  return i;
}

class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const emptyFunc = () => {};

const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

function finalizationRegistryCreate (unregister) {
  if (typeof FinalizationRegistry === 'undefined') {
    return { unregister () {} };
  }
  return new FinalizationRegistry(unregister);
}

const handleTables = [];

const instantiateCore = WebAssembly.instantiate;

const T_FLAG = 1 << 30;

function rscTableCreateBorrow (table, rep) {
  const free = table[0] & ~T_FLAG;
  if (free === 0) {
    table.push(scopeId);
    table.push(rep);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free];
  table[free << 1] = scopeId;
  table[(free << 1) + 1] = rep;
  return free;
}

function rscTableCreateOwn (table, rep) {
  const free = table[0] & ~T_FLAG;
  if (free === 0) {
    table.push(0);
    table.push(rep | T_FLAG);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free << 1];
  table[free << 1] = 0;
  table[(free << 1) + 1] = rep | T_FLAG;
  return free;
}

function rscTableRemove (table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = (val & T_FLAG) !== 0;
  const rep = val & ~T_FLAG;
  if (val === 0 || (scope & T_FLAG) !== 0) throw new TypeError('Invalid handle');
  table[handle << 1] = table[0] | T_FLAG;
  table[0] = handle | T_FLAG;
  return { rep, scope, own };
}
let resourceCallBorrows = [];
function resourceTransferBorrow(handle, fromTid, toTid) {
  const fromTable = handleTables[fromTid];
  const isOwn = (fromTable[(handle << 1) + 1] & T_FLAG) !== 0;
  const rep = isOwn ? fromTable[(handle << 1) + 1] & ~T_FLAG : rscTableRemove(fromTable, handle).rep;
  if (definedResourceTables[toTid]) return rep;
  const toTable = handleTables[toTid] || (handleTables[toTid] = [T_FLAG, 0]);
  const newHandle = rscTableCreateBorrow(toTable, rep);
  resourceCallBorrows.push({ rid: toTid, handle: newHandle });
  return newHandle;
}

function resourceTransferOwn(handle, fromTid, toTid) {
  const { rep } = rscTableRemove(handleTables[fromTid], handle);
  const toTable = handleTables[toTid] || (handleTables[toTid] = [T_FLAG, 0]);
  return rscTableCreateOwn(toTable, rep);
}

let scopeId = 0;

const symbolRscHandle = Symbol('handle');

const symbolDispose = Symbol.dispose || Symbol.for('dispose');

function toUint32(val) {
  return val >>> 0;
}

function toUint8(val) {
  val >>>= 0;
  val %= 2 ** 8;
  return val;
}

const definedResourceTables = [true,true,true,true,];

let exports0;
let exports1;
let exports2;
let exports3;
let exports4;
let exports5;
let exports6;
let exports7;
let memory0;
let postReturn0;
let realloc0;
let postReturn1;
const handleTable0 = [T_FLAG, 0];
const finalizationRegistry0 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable0, handle);
  exports0['0'](rep);
});

handleTables[0] = handleTable0;
const trampoline0 = rscTableCreateOwn.bind(null, handleTable0);
function trampoline1(handle) {
  const handleEntry = rscTableRemove(handleTable0, handle);
  if (handleEntry.own) {
    
    exports0['0'](handleEntry.rep);
  }
}
const handleTable1 = [T_FLAG, 0];
const finalizationRegistry1 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable1, handle);
  exports0['1'](rep);
});

handleTables[1] = handleTable1;
const trampoline2 = rscTableCreateOwn.bind(null, handleTable1);
function trampoline3(handle) {
  const handleEntry = rscTableRemove(handleTable1, handle);
  if (handleEntry.own) {
    
    exports0['1'](handleEntry.rep);
  }
}
const handleTable9 = [T_FLAG, 0];
const finalizationRegistry9 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable9, handle);
  exports3['6'](rep);
});

handleTables[9] = handleTable9;
const trampoline4 = rscTableCreateOwn.bind(null, handleTable9);
function trampoline5() {
  scopeId++;
}
const trampoline6 = resourceTransferBorrow;
function trampoline7() {
  scopeId--;
  for (const { rid, handle } of resourceCallBorrows) {
    if (handleTables[rid][handle << 1] === scopeId)
    throw new TypeError('borrows not dropped for resource call');
  }
  resourceCallBorrows= [];
}
const handleTable8 = [T_FLAG, 0];
const finalizationRegistry8 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable8, handle);
  exports3['5'](rep);
});

handleTables[8] = handleTable8;
const trampoline8 = rscTableCreateOwn.bind(null, handleTable8);
function trampoline9(handle) {
  const handleEntry = rscTableRemove(handleTable8, handle);
  if (handleEntry.own) {
    
    exports3['5'](handleEntry.rep);
  }
}
function trampoline10(handle) {
  const handleEntry = rscTableRemove(handleTable9, handle);
  if (handleEntry.own) {
    
    exports3['6'](handleEntry.rep);
  }
}
const handleTable6 = [T_FLAG, 0];
const finalizationRegistry6 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable6, handle);
  exports0['0'](rep);
});

handleTables[6] = handleTable6;
function trampoline11(handle) {
  const handleEntry = rscTableRemove(handleTable6, handle);
  if (handleEntry.own) {
    
    exports0['0'](handleEntry.rep);
  }
}
const handleTable7 = [T_FLAG, 0];
const finalizationRegistry7 = finalizationRegistryCreate((handle) => {
  const { rep } = rscTableRemove(handleTable7, handle);
  exports0['1'](rep);
});

handleTables[7] = handleTable7;
function trampoline12(handle) {
  const handleEntry = rscTableRemove(handleTable7, handle);
  if (handleEntry.own) {
    
    exports0['1'](handleEntry.rep);
  }
}
const trampoline13 = resourceTransferOwn;

class ScanLine{
  constructor () {
    throw new Error('"ScanLine" resource does not define a constructor');
  }
}

ScanLine.prototype.getFilterType = function getFilterType() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.get-filter-type'](handle0);
  let enum2;
  switch (ret) {
    case 0: {
      enum2 = 'none';
      break;
    }
    case 1: {
      enum2 = 'sub';
      break;
    }
    case 2: {
      enum2 = 'up';
      break;
    }
    case 3: {
      enum2 = 'average';
      break;
    }
    case 4: {
      enum2 = 'paeth';
      break;
    }
    default: {
      throw new TypeError('invalid discriminant specified for FilterType');
    }
  }
  return enum2;
};

ScanLine.prototype.setFilterType = function setFilterType(arg1) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  var val2 = arg1;
  let enum2;
  switch (val2) {
    case 'none': {
      enum2 = 0;
      break;
    }
    case 'sub': {
      enum2 = 1;
      break;
    }
    case 'up': {
      enum2 = 2;
      break;
    }
    case 'average': {
      enum2 = 3;
      break;
    }
    case 'paeth': {
      enum2 = 4;
      break;
    }
    default: {
      if ((arg1) instanceof Error) {
        console.error(arg1);
      }
      
      throw new TypeError(`"${val2}" is not one of the cases of filter-type`);
    }
  }
  exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.set-filter-type'](handle0, enum2);
};

ScanLine.prototype.size = function size() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.size'](handle0);
  return ret >>> 0;
};

ScanLine.prototype.getPixelAt = function getPixelAt(arg1) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.get-pixel-at'](handle0, toUint32(arg1));
  return clampGuest(ret, 0, 255);
};

ScanLine.prototype.setPixelAt = function setPixelAt(arg1, arg2) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.set-pixel-at'](handle0, toUint32(arg1), toUint8(arg2));
};

ScanLine.prototype.read = function read() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.read'](handle0);
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var ptr2 = dataView(memory0).getInt32(ret + 4, true);
      var len2 = dataView(memory0).getInt32(ret + 8, true);
      var result2 = new Uint8Array(memory0.buffer.slice(ptr2, ptr2 + len2 * 1));
      variant3= {
        tag: 'ok',
        val: result2
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn0(ret);
  if (variant3.tag === 'err') {
    throw new ComponentError(variant3.val);
  }
  return variant3.val;
};

ScanLine.prototype.write = function write(arg1) {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable8[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "ScanLine" resource.');
  }
  var handle0 = handleTable8[(handle1 << 1) + 1] & ~T_FLAG;
  var val2 = arg1;
  var len2 = val2.byteLength;
  var ptr2 = realloc0(0, 0, 1, len2 * 1);
  var src2 = new Uint8Array(val2.buffer || val2, val2.byteOffset, len2 * 1);
  (new Uint8Array(memory0.buffer, ptr2, len2 * 1)).set(src2);
  exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.write'](handle0, ptr2, len2);
};

class Png{
  constructor () {
    throw new Error('"Png" resource does not define a constructor');
  }
}

Png.prototype.getScanLines = function getScanLines() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable9[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "Png" resource.');
  }
  var handle0 = handleTable9[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]png.get-scan-lines'](handle0);
  var len4 = dataView(memory0).getInt32(ret + 4, true);
  var base4 = dataView(memory0).getInt32(ret + 0, true);
  var result4 = [];
  for (let i = 0; i < len4; i++) {
    const base = base4 + i * 4;
    var handle3 = dataView(memory0).getInt32(base + 0, true);
    var rsc2 = new.target === ScanLine ? this : Object.create(ScanLine.prototype);
    Object.defineProperty(rsc2, symbolRscHandle, { writable: true, value: handle3});
    finalizationRegistry8.register(rsc2, handle3, rsc2);
    Object.defineProperty(rsc2, symbolDispose, { writable: true, value: function () {
      finalizationRegistry8.unregister(rsc2);
      rscTableRemove(handleTable8, handle3);
      rsc2[symbolDispose] = emptyFunc;
      rsc2[symbolRscHandle] = null;
      exports3['5'](handleTable8[(handle3 << 1) + 1] & ~T_FLAG);
    }});
    result4.push(rsc2);
  }
  postReturn1(ret);
  return result4;
};

Png.prototype.read = function read() {
  var handle1 = this[symbolRscHandle];
  if (!handle1 || (handleTable9[(handle1 << 1) + 1] & T_FLAG) === 0) {
    throw new TypeError('Resource error: Not a valid "Png" resource.');
  }
  var handle0 = handleTable9[(handle1 << 1) + 1] & ~T_FLAG;
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[method]png.read'](handle0);
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var ptr2 = dataView(memory0).getInt32(ret + 4, true);
      var len2 = dataView(memory0).getInt32(ret + 8, true);
      var result2 = new Uint8Array(memory0.buffer.slice(ptr2, ptr2 + len2 * 1));
      variant3= {
        tag: 'ok',
        val: result2
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn0(ret);
  if (variant3.tag === 'err') {
    throw new ComponentError(variant3.val);
  }
  return variant3.val;
};

Png.create = function create(arg0) {
  var val0 = arg0;
  var len0 = val0.byteLength;
  var ptr0 = realloc0(0, 0, 1, len0 * 1);
  var src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
  (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
  const ret = exports5['chikoski:glitch-art/png-glitchable@0.3.4#[static]png.create'](ptr0, len0);
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var handle2 = dataView(memory0).getInt32(ret + 4, true);
      var rsc1 = new.target === Png ? this : Object.create(Png.prototype);
      Object.defineProperty(rsc1, symbolRscHandle, { writable: true, value: handle2});
      finalizationRegistry9.register(rsc1, handle2, rsc1);
      Object.defineProperty(rsc1, symbolDispose, { writable: true, value: function () {
        finalizationRegistry9.unregister(rsc1);
        rscTableRemove(handleTable9, handle2);
        rsc1[symbolDispose] = emptyFunc;
        rsc1[symbolRscHandle] = null;
        exports3['6'](handleTable9[(handle2 << 1) + 1] & ~T_FLAG);
      }});
      variant3= {
        tag: 'ok',
        val: rsc1
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  if (variant3.tag === 'err') {
    throw new ComponentError(variant3.val);
  }
  return variant3.val;
};

function create(arg0, arg1, arg2) {
  var val0 = arg0;
  var len0 = val0.byteLength;
  var ptr0 = realloc0(0, 0, 1, len0 * 1);
  var src0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, len0 * 1);
  (new Uint8Array(memory0.buffer, ptr0, len0 * 1)).set(src0);
  const ret = exports5['chikoski:glitch-art/bridge-to-png-glitchable@0.3.4#create'](ptr0, len0, toUint32(arg1), toUint32(arg2));
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      var handle2 = dataView(memory0).getInt32(ret + 4, true);
      var rsc1 = new.target === Png ? this : Object.create(Png.prototype);
      Object.defineProperty(rsc1, symbolRscHandle, { writable: true, value: handle2});
      finalizationRegistry9.register(rsc1, handle2, rsc1);
      Object.defineProperty(rsc1, symbolDispose, { writable: true, value: function () {
        finalizationRegistry9.unregister(rsc1);
        rscTableRemove(handleTable9, handle2);
        rsc1[symbolDispose] = emptyFunc;
        rsc1[symbolRscHandle] = null;
        exports3['6'](handleTable9[(handle2 << 1) + 1] & ~T_FLAG);
      }});
      variant3= {
        tag: 'ok',
        val: rsc1
      };
      break;
    }
    case 1: {
      variant3= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  if (variant3.tag === 'err') {
    throw new ComponentError(variant3.val);
  }
  return variant3.val;
}

const $init = (() => {
  let gen = (function* init () {
    const module0 = fetchCompile(new URL('./png-glitch.core.wasm', import.meta.url));
    const module1 = base64Compile('AGFzbQEAAAABBQFgAX8AAwMCAAAEBQFwAQICBxQDATAAAAExAAEIJGltcG9ydHMBAAoVAgkAIABBABEAAAsJACAAQQERAAALAC8JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQHMC4yMTYuMACZAQRuYW1lABMSd2l0LWNvbXBvbmVudDpzaGltAX0CAD9kdG9yLVtleHBvcnRdY2hpa29za2k6Z2xpdGNoLWFydC9wbmctZ2xpdGNoYWJsZUAwLjMuNC1zY2FuLWxpbmUBOWR0b3ItW2V4cG9ydF1jaGlrb3NraTpnbGl0Y2gtYXJ0L3BuZy1nbGl0Y2hhYmxlQDAuMy40LXBuZw');
    const module2 = base64Compile('AGFzbQEAAAABBQFgAX8AAhoDAAEwAAAAATEAAAAIJGltcG9ydHMBcAECAgkIAQBBAAsCAAEALwlwcm9kdWNlcnMBDHByb2Nlc3NlZC1ieQENd2l0LWNvbXBvbmVudAcwLjIxNi4wABwEbmFtZQAVFHdpdC1jb21wb25lbnQ6Zml4dXBz');
    const module3 = fetchCompile(new URL('./png-glitch.core2.wasm', import.meta.url));
    const module4 = base64Compile('AGFzbQEAAAABFgRgAn9/AGADf39/AGADf39/AGABfwADCAcAAQAAAgMDBAUBcAEHBwcoCAEwAAABMQABATIAAgEzAAMBNAAEATUABQE2AAYIJGltcG9ydHMBAApVBwsAIAAgAUEAEQAACw0AIAAgASACQQERAQALCwAgACABQQIRAAALCwAgACABQQMRAAALDQAgACABIAJBBBECAAsJACAAQQURAwALCQAgAEEGEQMACwAvCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BzAuMjE2LjAAhwQEbmFtZQATEndpdC1jb21wb25lbnQ6c2hpbQHqAwcASGluZGlyZWN0LWNoaWtvc2tpOmdsaXRjaC1hcnQvcG5nLWdsaXRjaGFibGVAMC4zLjQtW21ldGhvZF1zY2FuLWxpbmUucmVhZAFJaW5kaXJlY3QtY2hpa29za2k6Z2xpdGNoLWFydC9wbmctZ2xpdGNoYWJsZUAwLjMuNC1bbWV0aG9kXXNjYW4tbGluZS53cml0ZQJMaW5kaXJlY3QtY2hpa29za2k6Z2xpdGNoLWFydC9wbmctZ2xpdGNoYWJsZUAwLjMuNC1bbWV0aG9kXXBuZy5nZXQtc2Nhbi1saW5lcwNCaW5kaXJlY3QtY2hpa29za2k6Z2xpdGNoLWFydC9wbmctZ2xpdGNoYWJsZUAwLjMuNC1bbWV0aG9kXXBuZy5yZWFkBERpbmRpcmVjdC1jaGlrb3NraTpnbGl0Y2gtYXJ0L3BuZy1nbGl0Y2hhYmxlQDAuMy40LVtzdGF0aWNdcG5nLmNyZWF0ZQU/ZHRvci1bZXhwb3J0XWNoaWtvc2tpOmdsaXRjaC1hcnQvcG5nLWdsaXRjaGFibGVAMC4zLjQtc2Nhbi1saW5lBjlkdG9yLVtleHBvcnRdY2hpa29za2k6Z2xpdGNoLWFydC9wbmctZ2xpdGNoYWJsZUAwLjMuNC1wbmc');
    const module5 = base64Compile('AGFzbQEAAAABFgRgAn9/AGADf39/AGADf39/AGABfwACMwgAATAAAAABMQABAAEyAAAAATMAAAABNAACAAE1AAMAATYAAwAIJGltcG9ydHMBcAEHBwkNAQBBAAsHAAECAwQFBgAvCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BzAuMjE2LjAAHARuYW1lABUUd2l0LWNvbXBvbmVudDpmaXh1cHM');
    const module6 = base64Compile('AGFzbQEAAAABIQZgAX8Bf2AAAGADf39/AX9gAn9/AGACf38Bf2ADf39/AALHAQoFZmxhZ3MJaW5zdGFuY2UxA38BBWZsYWdzCWluc3RhbmNlMwN/AQZjYWxsZWUIYWRhcHRlcjAAAAhyZXNvdXJjZQplbnRlci1jYWxsAAEIcmVzb3VyY2UPdHJhbnNmZXItYm9ycm93AAIIcmVzb3VyY2UJZXhpdC1jYWxsAAEGY2FsbGVlCGFkYXB0ZXIxAAMGY2FsbGVlCGFkYXB0ZXIyAAAGY2FsbGVlCGFkYXB0ZXIzAAQGY2FsbGVlCGFkYXB0ZXI0AAUDBgUAAwAEBQc4BQhhZGFwdGVyMAAICGFkYXB0ZXIxAAkIYWRhcHRlcjIACghhZGFwdGVyMwALCGFkYXB0ZXI0AAwKkAQFhAEBAX8jAUEBcUUEQAALIwBBAnFFBEAACyMAQX1xJAAQASMAQX5xJAAgAEEGQQAQAiMAQQFyJAAQACEBIwFBfnEkAQJ/AkACQAJAAkACQAJAIAEOBQECAwQFAAsAC0EADAQLQQEMAwtBAgwCC0EDDAELQQQLIwFBAXIkASMAQQJyJAAQAwuAAQAjAUEBcUUEQAALIwBBAnFFBEAACyMAQX1xJAAQASMAQX5xJAAgAEEGQQAQAgJ/AkACQAJAAkACQAJAIAEOBQECAwQFAAsAC0EADAQLQQEMAwtBAgwCC0EDDAELQQQLIwBBAXIkABAEIwFBfnEkASMBQQFyJAEjAEECciQAEAMLVAEBfyMBQQFxRQRAAAsjAEECcUUEQAALIwBBfXEkABABIwBBfnEkACAAQQZBABACIwBBAXIkABAFIQEjAUF+cSQBIAEjAUEBciQBIwBBAnIkABADC1oBAX8jAUEBcUUEQAALIwBBAnFFBEAACyMAQX1xJAAQASMAQX5xJAAgAEEGQQAQAiABIwBBAXIkABAGIQIjAUF+cSQBIAJB/wFxIwFBAXIkASMAQQJyJAAQAwtWACMBQQFxRQRAAAsjAEECcUUEQAALIwBBfXEkABABIwBBfnEkACAAQQZBABACIAEgAkH/AXEjAEEBciQAEAcjAUF+cSQBIwFBAXIkASMAQQJyJAAQAws');
    const module7 = base64Compile('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');
    const instanceFlags1 = new WebAssembly.Global({ value: "i32", mutable: true }, 3);
    const instanceFlags3 = new WebAssembly.Global({ value: "i32", mutable: true }, 3);
    ({ exports: exports0 } = yield instantiateCore(yield module1));
    ({ exports: exports1 } = yield instantiateCore(yield module0, {
      '[export]chikoski:glitch-art/png-glitchable@0.3.4': {
        '[resource-drop]png': trampoline3,
        '[resource-drop]scan-line': trampoline1,
        '[resource-new]png': trampoline2,
        '[resource-new]scan-line': trampoline0,
      },
    }));
    ({ exports: exports2 } = yield instantiateCore(yield module2, {
      '': {
        $imports: exports0.$imports,
        '0': exports1['chikoski:glitch-art/png-glitchable@0.3.4#[dtor]scan-line'],
        '1': exports1['chikoski:glitch-art/png-glitchable@0.3.4#[dtor]png'],
      },
    }));
    ({ exports: exports3 } = yield instantiateCore(yield module4));
    ({ exports: exports4 } = yield instantiateCore(yield module6, {
      callee: {
        adapter0: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.get-filter-type'],
        adapter1: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.set-filter-type'],
        adapter2: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.size'],
        adapter3: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.get-pixel-at'],
        adapter4: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.set-pixel-at'],
      },
      flags: {
        instance1: instanceFlags1,
        instance3: instanceFlags3,
      },
      resource: {
        'enter-call': trampoline5,
        'exit-call': trampoline7,
        'transfer-borrow': trampoline6,
      },
    }));
    ({ exports: exports5 } = yield instantiateCore(yield module3, {
      '[export]chikoski:glitch-art/png-glitchable@0.3.4': {
        '[resource-drop]png': trampoline10,
        '[resource-drop]scan-line': trampoline9,
        '[resource-new]png': trampoline4,
        '[resource-new]scan-line': trampoline8,
      },
      'chikoski:glitch-art/png-glitchable@0.3.4': {
        '[method]png.get-scan-lines': exports3['2'],
        '[method]png.read': exports3['3'],
        '[method]scan-line.get-filter-type': exports4.adapter0,
        '[method]scan-line.get-pixel-at': exports4.adapter3,
        '[method]scan-line.read': exports3['0'],
        '[method]scan-line.set-filter-type': exports4.adapter1,
        '[method]scan-line.set-pixel-at': exports4.adapter4,
        '[method]scan-line.size': exports4.adapter2,
        '[method]scan-line.write': exports3['1'],
        '[resource-drop]png': trampoline12,
        '[resource-drop]scan-line': trampoline11,
        '[static]png.create': exports3['4'],
      },
    }));
    ({ exports: exports6 } = yield instantiateCore(yield module7, {
      augments: {
        'mem1 I32Load8U': (ptr, off) => new DataView(exports5.memory.buffer).getUint8(ptr + off, true),
        'mem1 I32Store': (ptr, val, offset) => {
          new DataView(exports5.memory.buffer).setInt32(ptr + offset, val, true);
        },
        'mem1 I32Store8': (ptr, val, offset) => {
          new DataView(exports5.memory.buffer).setInt8(ptr + offset, val, true);
        },
        'mem1 MemorySize': ptr => exports5.memory.buffer.byteLength / 65536,
      },
      callee: {
        adapter5: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.read'],
        adapter6: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]scan-line.write'],
        adapter7: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]png.get-scan-lines'],
        adapter8: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[method]png.read'],
        adapter9: exports1['chikoski:glitch-art/png-glitchable@0.3.4#[static]png.create'],
      },
      flags: {
        instance1: instanceFlags1,
        instance3: instanceFlags3,
      },
      memory: {
        m0: exports1.memory,
      },
      post_return: {
        adapter5: exports1['cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]png.read'],
        adapter7: exports1['cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]png.get-scan-lines'],
      },
      realloc: {
        f0: exports5.cabi_realloc,
        f6: exports1.cabi_realloc,
      },
      resource: {
        'enter-call': trampoline5,
        'exit-call': trampoline7,
        'transfer-borrow': trampoline6,
        'transfer-own': trampoline13,
      },
    }));
    ({ exports: exports7 } = yield instantiateCore(yield module5, {
      '': {
        $imports: exports3.$imports,
        '0': exports6.adapter5,
        '1': exports6.adapter6,
        '2': exports6.adapter7,
        '3': exports6.adapter8,
        '4': exports6.adapter9,
        '5': exports5['chikoski:glitch-art/png-glitchable@0.3.4#[dtor]scan-line'],
        '6': exports5['chikoski:glitch-art/png-glitchable@0.3.4#[dtor]png'],
      },
    }));
    memory0 = exports5.memory;
    postReturn0 = exports5['cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]png.read'];
    realloc0 = exports5.cabi_realloc;
    postReturn1 = exports5['cabi_post_chikoski:glitch-art/png-glitchable@0.3.4#[method]png.get-scan-lines'];
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
const bridgeToPngGlitchable034 = {
  create: create,
  
};
const pngGlitchable034 = {
  Png: Png,
  ScanLine: ScanLine,
  
};

export { bridgeToPngGlitchable034 as bridgeToPngGlitchable, pngGlitchable034 as pngGlitchable, bridgeToPngGlitchable034 as 'chikoski:glitch-art/bridge-to-png-glitchable@0.3.4', pngGlitchable034 as 'chikoski:glitch-art/png-glitchable@0.3.4',  }