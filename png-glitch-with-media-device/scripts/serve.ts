import { serveDir } from "jsr:@std/http/file-server";
import { resolve } from "./location.ts";

const locations = {
  rootDir: "./build"
}

class Context {
  fsRoot: string

  constructor(root: string) {
    this.fsRoot = root
  }
}

function start() {
  const projectRoot = resolve("..");
  const config = new Context(resolve(locations.rootDir, projectRoot));
  console.log(config);
  main(config);
}

function main(config: Context) {
  Deno.serve((req: Request) => {
    const url = new URL(req.url);
    let request = req;

    if (url.pathname  === "/")  {
      url.pathname = "/index.html"
      request = new Request(url);
    }
    if (url.pathname.startsWith("/ui/")) {
      url.pathname += ".js";
      request = new Request(url)
    }
    if (url.pathname === "/png-glitchable/png-glitch") {
      url.pathname += ".js";
      request = new Request(url)
    }
    console.log(request.url);
    return serveDir(request, {
      fsRoot: config.fsRoot,
    });
  });    
}

start();
