import { type Modifier, DefaultModifier } from "../composable";
import { type Content, DefaultContent, Box } from "./box";

export function Column(
    content: Content = DefaultContent,
    modifier: Modifier = DefaultModifier): HTMLDivElement {
    return Box(content, (div) => {
        div.classList.add("column");
        return modifier(div);
    });
}
function Card(
    content: Content = DefaultContent,
    modifier: Modifier = DefaultModifier
): HTMLDivElement {
    return Box(content, (div) => {
        div.classList.add("card");
        return modifier(div);
    });
}
function DisplayLarge(
    text: string,
    modifier: Modifier<HTMLHeadElement> = DefaultModifier
): HTMLHeadElement {
    const head = document.createElement("h1");
    head.innerText = text;
    return modifier(head);
}
export function Video(
    modifier: Modifier<HTMLVideoElement> = DefaultModifier): HTMLVideoElement {
    const video = document.createElement("video");
    return modifier(video);
}
export function Button(
    label: string,
    onClick: () => void,
    modifier: Modifier<HTMLButtonElement> = DefaultModifier): HTMLButtonElement {
    const button = document.createElement("button");
    button.type = "button";
    button.addEventListener("click", onClick);
    button.innerText = label;
    return modifier(button);
}

