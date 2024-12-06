import { countable } from "./counter/counter.js";

function main() {
	const counter = countable.Counter.new();
	console.log(counter.valueOf());
	counter.up();
	counter.up();
	counter.up();
	console.log(counter.valueOf());
	counter.down();
	console.log(counter.valueOf());
}

main();
