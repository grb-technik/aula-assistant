import { cn } from "@/lib/utils";

export function MiniViewButton({
    icon,
    title,
    className,
    onClick,
}: {
    className?: string;
    icon: JSX.Element;
    title: string;
    onClick?: () => void;
}) {
    return (
        <button
            className={cn(
                "bg-secondary active:bg-primary/40 hover:bg-secondary/80 focus-visible:border-ring focus-visible:ring-ring/50 flex h-full w-full flex-col items-center justify-center gap-1 rounded p-4 font-medium transition-all duration-100 ease-in outline-none focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50",
                className,
            )}
            onClick={onClick}>
            {icon}
            <span className="mt-2 text-xl font-semibold">{title}</span>
        </button>
    );
}
