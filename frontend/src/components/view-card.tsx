import { cn } from "@/lib/utils";

export function ViewCard({ children, className }: { children?: React.ReactNode; className?: string }) {
    return (
        <div
            className={cn(
                "bg-secondary flex h-full min-h-20 w-full min-w-40 shrink-0 flex-col items-center justify-center gap-1 rounded p-4 font-medium transition-all duration-100 ease-in outline-none",
                className,
            )}>
            {children}
        </div>
    );
}
