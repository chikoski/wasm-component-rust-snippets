import { Column, Video, Button } from "../composable/components";


export function DefaultMediaScreen(
    requestMediaSource: () => void = () => undefined): HTMLElement {
    return Column((parent) => {
        const video = Video((e) => {
            e.classList.add("preview");
            return e;
        });
        parent.appendChild(video);

        const button = Button("Click to capture", requestMediaSource);
        parent.appendChild(button);
    });
}
