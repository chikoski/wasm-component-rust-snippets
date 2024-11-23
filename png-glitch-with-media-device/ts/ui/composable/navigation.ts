
type Content = () => Promise<HTMLElement>;
export class NavigationHost {
	routes: Map<string, Content> = new Map();
	root: HTMLElement;

	constructor(root: HTMLElement) {
		this.root = root;
	}

	compsable(route: string, content: Content) {
		this.routes.set(route, content);
	}

	async navigate(route: string) {
		const content = this.routes.get(route);
		if (content != null) {
			const updated = await content();
			this.unmount();
			this.root.appendChild(updated);
		}
	}

	unmount() {
		for (const child of this.root.children) {
			this.root.removeChild(child);
		}
	}
}
type RouteGraph = (navHost: NavigationHost) => void;
export function Navigation(
	navHost: NavigationHost,
	graph: RouteGraph) {
	graph(navHost);
}
