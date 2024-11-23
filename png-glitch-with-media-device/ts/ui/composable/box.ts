import { type Modifier, DefaultModifier } from "../composable";

export type Content<T extends HTMLElement = HTMLDivElement> = (parent: T) => void;
export function DefaultContent(parent: HTMLElement): null {
    return null;
}
export function Box(
    content: Content = DefaultContent,
    modifier: Modifier = DefaultModifier): HTMLDivElement {
    const div = document.createElement("div");
    content(div);
    return modifier(div);
}
