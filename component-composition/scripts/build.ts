#!/opt/homebrew/bin/deno run -A

enum BuildType {
	Release = "release",
	Debug = "debug",
}

enum Architecture {
	Wasi = "wasm32-wasi",
	Unknown = "wasm32-unknown-unknown",
}

class Wasm {
	architecture: Architecture;
	buildType: BuildType;
	name: string;

	constructor(
		name: string,
		architecture: Architecture = Architecture.Wasi,
		buildType: BuildType = BuildType.Release,
	) {
		this.name = name;
		this.architecture = architecture;
		this.buildType = buildType;
	}

	get path(): string {
		return `${this.architecture}/${this.buildType}/${this.name}`;
	}

	expandPath(base: URL): URL {
		return new URL(this.path, base);
	}

	normalize(): Wasm {
		const name = this.name.replaceAll("_", "-");
		return new Wasm(name, this.architecture, this.buildType);
	}
}

class Context {
	projectRoot: URL;
	outputDir: URL;
	targetDir: URL;
	wasm: Wasm;
	dependencies: Array<Wasm>;

	constructor(
		wasm: Wasm,
		dependencies: Array<Wasm> = [],
		projectRoot: URL = new URL("../..", import.meta.url),
		outputDir: URL = new URL("./composed/", projectRoot),
	) {
		this.wasm = wasm;
		this.dependencies = dependencies;
		this.projectRoot = projectRoot;
		this.outputDir = outputDir;
		this.targetDir = new URL("./target/", this.projectRoot);
	}

	async start() {
		console.log(`Start composition: ${this.wasm.name}`);
		this.showConfig();
		await this.preProcess();
		await this.compose();
		console.log(`Done. Stored at ${this.outputFilePath}`);
	}

	private showConfig() {
		console.log(`Project root = ${this.projectRoot.pathname}`);
		console.log(`Build artifacts in ${this.targetDir.pathname}`);
		console.log(`Composed Wasm file is stored as ${this.outputFilePath}`);
	}

	private async compose() {
		const toDependencyOption: (wasm: Wasm) => Array<string> = (wasm: Wasm) => [
			"-d",
			wasm.expandPath(this.targetDir).pathname,
		];
		const dependencies = this.dependencies.flatMap(toDependencyOption);
		console.log(this.dependencies);
		const options = [
			"compose",
			"-o",
			this.outputFilePath,
			dependencies,
			this.wasm.expandPath(this.targetDir).pathname,
		].flat();
		const command = new Deno.Command("wasm-tools", { args: options });
		await command.spawn();
	}

	private get outputFilePath(): string {
		const url = new URL(this.wasm.name, this.outputDir);
		return url.pathname;
	}

	private async preProcess() {
		await this.createOutputDir();
		await this.normalizeDependences();
	}

	private async normalizeDependences() {
		const normalizdDependencies = [];
		for (const dependency of this.dependencies) {
			const normalized = dependency.normalize();
			const oririnal = dependency.expandPath(this.targetDir);
			const updated = normalized.expandPath(this.targetDir);
			try {
				await Deno.rename(oririnal.pathname, updated.pathname);
			} catch (_) {
				console.log(`${normalized.name} is already available. Skip.`);
			}
			normalizdDependencies.push(normalized);
		}
		this.dependencies = normalizdDependencies;
	}

	private async createOutputDir() {
		const path = this.outputDir.pathname;
		try {
			await Deno.stat(path);
		} catch (_) {
			await Deno.mkdir(this.outputDir.pathname, { recursive: true });
		}
	}
}

function main() {
	const host = new Wasm("calculation-cli.wasm");
	const dependencies = [
		new Wasm("calculation_add.wasm"),
		new Wasm("formatter_ferris.wasm"),
	];
	const context = new Context(host, dependencies);
	context.start();
}

main();
