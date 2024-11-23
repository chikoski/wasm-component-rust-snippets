import type { Png } from "../../png-glitchable/interfaces/chikoski-glitch-art-bridge-to-png-glitchable";
import { bridgeToPngGlitchable } from "../../png-glitchable/png-glitch";

export type MediaScreenUiState = DefaultState | StreamOpened;
export class DefaultState { }

export class StreamOpened {
    public preview: MediaStream;
    public glitched: MediaStream;

    constructor(
        mediaStream: MediaStream,
        onEnded: () => void
    ) {
        this.preview = mediaStream;
        this.glitched = modifyVideoTrack(
            this.preview.clone(),
            onEnded,
        );
    }

    stop() {
        for (const track of this.preview.getTracks()) {
            track.stop();
        }
    }

}

function modifyVideoTrack(
	mediaStream: MediaStream,
	onEnded: () => void = () => { }
): MediaStream {
	const videoTrack = mediaStream.getVideoTracks()[0];
	videoTrack.addEventListener("ended", onEnded);
	
	const newVideoTrack = glitchEffect(videoTrack);

	const newMediaStream = new MediaStream([
		newVideoTrack,
		...mediaStream.getAudioTracks(),
	]);
	return newMediaStream;
}

function glitchEffect(
    videoTrack: MediaStreamVideoTrack
): MediaStreamVideoTrack {
    const canvas = createOffScreenCavasFor(videoTrack);
    
    const processor = new MediaStreamTrackProcessor({ track: videoTrack });
	const generator = new MediaStreamTrackGenerator({ kind: "video" });
    const transformer = createGlitchTransformStream(
        canvas,
        (glitcher) => {
            glitcher.getScanLines().forEach((scanline, index) => {
                if(index % 5 === 0) {
                    scanline.setFilterType("paeth");
                }
                const i = index % scanline.size();
                scanline.setPixelAt(i, 0);
            });
        }        
    );

    processor.readable.pipeThrough(transformer).pipeTo(generator.writable);
    return generator
}

function createGlitchTransformStream(
    canvas: OffscreenCanvas,
    glitch: (glitcher: Png) => void,
): TransformStream {
    const context = canvas.getContext("2d");

    return new TransformStream({
        transform(frame, controller) {
            (async () => {
                if(context != null) {
                    performance.mark("start");
                    context.drawImage(frame, 0, 0);
                    performance.mark("png:start");
                    const bitmap = context.getImageData(
                        0, 0, 
                        canvas.width, canvas.height, 
                    );
                    
                    performance.mark("glitch:start");
                    const glitched = await glitchPng(
                        bitmap.data.buffer, 
                        canvas.width, 
                        canvas.height, 
                        glitch,
                    );
                    performance.mark("glitch:end");
                    context.drawImage(glitched, 0, 0);
                    performance.mark("end");
                    
                    const all = performance.measure("all", "start", "end");
                    const glitchTime = performance.measure("glitch", "glitch:start", "glitch:end");
                    console.log(`${all.duration},${glitchTime.duration}`);
                }
                const modifiedFrame = canvas.transferToImageBitmap();
                const videoFrame = new VideoFrame(
                    modifiedFrame, {
                        timestamp: frame.timestamp
                    }
                );
                controller.enqueue(videoFrame);
            })()            
        }
    });
}

async function glitchPng(
    bitmap: ArrayBuffer,
    width: number,
    height: number,
    glitch: (glitcher: Png) => void,
): Promise<HTMLImageElement> {
    const data = new Uint8Array(bitmap);
    performance.mark("wasm:start");
    performance.mark("wasm:create:start");
    const glitcher = await bridgeToPngGlitchable.create(data, width, height);
    performance.mark("wasm:create:end");
    glitch(glitcher);
    performance.mark("wasm:read:start");
    const glitched = await glitcher.read();
    performance.mark("wasm:read:end");
    performance.mark("wasm:end");

    const wasm = performance.measure("wasm", "wasm:start", "wasm:end");
    const creation = performance.measure("creation", "wasm:create:start", "wasm:create:end");
    const read     = performance.measure("read", "wasm:read:start", "wasm:read:end");
    console.log(`${wasm.name}: ${wasm.duration}, ${creation.duration}, ${read.duration}`);


    performance.mark("blob:start");
    const b = new Blob([glitched], {type: "image/png"});
    const image = new Image(width, height)
    image.src = URL.createObjectURL(b);
    return new Promise((resolve, _) => {
        image.onload = () => {
            performance.mark("blob:end");
            resolve(image);
        };
    });
}

class Size {
    width = 480;
    height = 360;
}

function createOffScreenCavasFor(videoTrack: MediaStreamVideoTrack): OffscreenCanvas {
    return createOffScreenCanvas(getSizeFrom(videoTrack));
}

function createOffScreenCanvas(size: Size): OffscreenCanvas {
    return new OffscreenCanvas(size.width, size.height); 
}

function getSizeFrom(videoTrack: MediaStreamVideoTrack): Size {
    const size = new Size();
    const settings = videoTrack.getSettings();
    if(settings.width != null) {
        size.width = settings.width
    }
    if(settings.height != null) {
        size.height = settings.height
    }
    return size;
}