async function main(wasmFilePath: string) {
	const bytes = await Deno.readFile(wasmFilePath);
	const { instance } = await WebAssembly.instantiate(bytes, {});
	console.log(instance.exports);

	const add = instance.exports.add;

	console.log(`1 + 2 = ${add(BigInt(1), BigInt(2))}`);

	let id = 0;

	const fraction_new = instance.exports.fraction_new;
	const buffer = new Uint32Array(instance.exports.memory.buffer);

	class Fraction {
		index: number;

		constructor(numerator: number, denominator: number) {
			this.index = id;
			fraction_new(this.index, numerator, denominator);
			id += 2;
		}

		public get denominator(): number {
			return buffer[this.index + 1];
		}

		public get numerator(): number {
			return buffer[this.index];
		}
	}

	const a = new Fraction(3, 2);
	console.log(`a = ${a.numerator} / ${a.denominator}`);
}

async function start(wasmFilePath: string | undefined) {
	if (wasmFilePath == null) {
		console.log("Specify wasmfile to run");
	} else {
		await main(wasmFilePath);
	}
}

await start(Deno.args.at(0));
