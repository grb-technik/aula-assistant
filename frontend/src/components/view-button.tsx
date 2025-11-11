import { cn } from "@/lib/utils";

export function ViewButton({
    icon,
    title,
    description,
    className,
    disabled,
    onClick,
}: {
    className?: string;
    icon: JSX.Element;
    title: string;
    description?: string;
    disabled?: boolean;
    onClick?: () => void;
}) {
    return (
        <button
            disabled={disabled}
            className={cn(
                "bg-secondary active:bg-primary/40 hover:bg-secondary/80 focus-visible:border-ring focus-visible:ring-ring/50 flex h-full w-full flex-col items-center justify-center gap-1 rounded p-4 font-medium transition-all duration-100 ease-in outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50",
                className,
            )}
            onMouseDown={(e) => {
                if (e.button === 0) return;
                e.preventDefault();
                e.currentTarget.click();
            }}
            onClick={onClick}>
            {icon}
            <span className="mt-2 text-2xl font-semibold">{title}</span>
            <span className="text-muted-foreground w-4/5 text-center text-sm text-pretty">{description}</span>
        </button>
    );
}
