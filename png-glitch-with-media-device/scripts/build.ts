import * as path from "https://deno.land/std@0.224.0/path/mod.ts";

import { getScriptLocation } from "./location.ts"

const config = {
	build: "./build",
	assets: "./assets",
	wasm: "./ts/png-glitchable"
};

class Context {

	private buildCommand: Deno.Command
	private syncAssets: Deno.Command
	private syncWasm: Deno.Command
	private cleanCommand: Deno.Command
	
	constructor(
		wasmDir: string,
		buidDir: string, 
		assetDir: string,
	) {
		const scriptLocation = getScriptLocation();
		const rootLocation = path.resolve(scriptLocation, "../");
		const buildLocation = path.resolve(rootLocation, buidDir);
		const assetLocation = path.resolve(rootLocation, assetDir);
		const wasmLocation = path.resolve(rootLocation, wasmDir);

		this.buildCommand = createBuildCommand(rootLocation);
		this.cleanCommand = createCleanCommand(rootLocation, buildLocation);

		this.syncAssets = createSyncCommand(rootLocation, assetLocation, buildLocation, true);
		this.syncWasm = createSyncCommand(rootLocation, wasmLocation, buildLocation);
	}

	build() {
		Promise.all([
			this.clean(),
			this.buildScripts(), 
			this.copyAssets(),
			this.copyWasm(),
		]);
	}

	async copyAssets() {
		const _ = await this.syncAssets.spawn();
	}

	async copyWasm() {
		const _ = await this.syncWasm.spawn();
	}

	async buildScripts() {
		const _ = await this.buildCommand.spawn();
	}

	async clean() {
		const _ = await this.cleanCommand.spawn();
	}
}

function createBuildCommand(workingDir: string): Deno.Command {
	return new Deno.Command("tsc", {
		cwd: workingDir,
	});
}

function createCleanCommand(cwd: string, path: string): Deno.Command {
	return new Deno.Command("rm", {
		args: ["-rf", path],
		cwd: cwd
	});
}

function createSyncCommand(
	cwd: string, 
	src: string, 
	dest: string, 
	contentOnly: boolean = false
): Deno.Command {
	console.log(`${src} -> ${dest}`)
	let from = src;
	if(contentOnly) {
		from = `${from}/`;
	}

	return new Deno.Command("rsync", {
		args: ["-auv", from, dest],
        cwd: cwd
	});
}

function start() {
	const context = new Context(config.wasm, config.build, config.assets);
	context.build();
}

start();
