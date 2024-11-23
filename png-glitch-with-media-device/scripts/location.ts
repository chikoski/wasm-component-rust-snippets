import * as path from "https://deno.land/std@0.224.0/path/mod.ts";

export function getScriptLocation(): string {
	return path.resolve(path.dirname(path.fromFileUrl(import.meta.url)));
}

export function resolve(
    relativePath: string, 
    baseDir: string = getScriptLocation()
): string {
    return path.resolve(baseDir, relativePath);
}