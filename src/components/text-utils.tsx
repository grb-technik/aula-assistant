import { cn } from "@/lib/utils";

export function XSTextMono({ children, className, ...props }: React.ComponentProps<"span">) {
    return (
        <span className={cn("text-muted-foreground font-mono text-xs", className)} {...props}>
            {children}
        </span>
    );
}
