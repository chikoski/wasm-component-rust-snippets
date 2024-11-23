import { type Modifier, DefaultModifier } from "../composable";
import { Video } from "../composable/components";
import { Row } from "../composable/row";


export function WithStream(previewStream: MediaStream, stream: MediaStream) {
    return Row((parent) => {
        /*
        const previewStreamViewer = MediaStreamViewer(
            previewStream,
            true,
            (e) => {
                e.style.maxWidth = "50%";
                return e;
            }
        );
        */
        const mediaStreamViewer = MediaStreamViewer(
            stream,
            true,
            (e) => {
                e.style.maxWidth = "100%";
                return e;
            }
        );
        //parent.appendChild(previewStreamViewer);
        parent.appendChild(mediaStreamViewer);
    });
}
function MediaStreamViewer(
    mediaStream: MediaStream,
    autoPlay = true,
    modifier: Modifier<HTMLVideoElement> = DefaultModifier
): HTMLVideoElement {
    const video = Video((e) => {
        e.classList.add("media-stream-viewer");
        return e;
    });
    video.srcObject = mediaStream;
    if (autoPlay) {
        video.play();
    }
    return modifier(video);
}
