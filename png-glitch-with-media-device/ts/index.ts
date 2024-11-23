async function main() {
    console.log("main");
    const lib = await import("./ui/media-screen");
    const screen = lib.default();
    document.body.appendChild(screen);
}

document.addEventListener("DOMContentLoaded", main)