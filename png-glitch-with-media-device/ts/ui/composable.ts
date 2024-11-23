export type Modifier<T extends HTMLElement = HTMLDivElement> = (element: T) => T;

export function DefaultModifier<T extends HTMLElement>(e: T): T {
    return e;
}
