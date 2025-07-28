import { cn } from "@/lib/utils";

export function ViewButton({
    icon,
    title,
    description,
    className,
    onClick,
}: {
    className?: string;
    icon: JSX.Element;
    title: string;
    description?: string;
    onClick?: () => void;
}) {
    return (
        <button
            className={cn(
                "bg-secondary active:bg-primary/40 hover:bg-secondary/80 focus-visible:border-ring focus-visible:ring-ring/50 flex h-full min-h-20 w-full min-w-40 flex-col items-center justify-center gap-1 rounded p-4 font-medium transition-all duration-100 ease-in outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50",
                className,
            )}
            onClick={onClick}>
            {icon}
            <span className="mt-2 text-2xl font-semibold">{title}</span>
            <span className="text-muted-foreground w-4/5 text-center text-sm text-pretty">{description}</span>
        </button>
    );
}
