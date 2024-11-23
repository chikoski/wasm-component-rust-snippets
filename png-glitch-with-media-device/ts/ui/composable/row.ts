import { type Modifier, DefaultModifier } from "../composable";
import { Box } from "./box";

export class RowContext {
    children: Array<HTMLElement> = [];

    appendChild(child: HTMLElement) {
        child.style.flexShrink = "1";
        child.style.flexGrow = "1";
        this.children.push(child);
    }
}

type RowContextContent = (context: RowContext) => void;

export function Row(
    content: RowContextContent,
    modifier: Modifier = DefaultModifier): HTMLDivElement {
    const context = new RowContext();
    content(context);
    return Box(
        (parent) => {
            for (const c of context.children) {
                parent.appendChild(c);
            }
        },
        (div) => {
            div.classList.add("row");
            const element = flex(div);
            element.style.flexDirection = "row";
            return modifier(div);
        }
    );
}

function flex(element: HTMLElement): HTMLElement {
    element.style.display = "flex";
    return element;
}
