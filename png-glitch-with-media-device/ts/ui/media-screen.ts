import { NavigationHost, Navigation } from "./composable/navigation";
import { DefaultMediaScreen } from "./media-screen/default-screen";
import { StreamOpened } from "./media-screen/ui-state";
import { WithStream } from "./media-screen/with-stream";

export default function MediaScreen(): HTMLElement {
	const root = document.createElement("div");
	const navHost = new NavigationHost(root);

	Navigation(navHost, (navHost) => {

		navHost.compsable("/", async () => {
			return DefaultMediaScreen(() => {
				navHost.navigate("/open")
			});
		});

		navHost.compsable("/open", async () => {
			const mediaStream = await navigator.mediaDevices.getDisplayMedia();
			const uiState = new StreamOpened(
				mediaStream,
				() => { navHost.navigate("/") }
			);
			return WithStream(uiState.preview, uiState.glitched)
		});

	});
	navHost.navigate("/");

	return root;
}

