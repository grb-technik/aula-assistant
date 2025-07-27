import { cn } from "@/lib/utils";
import { useWindow } from "@/contexts/window";
import { CloseWindowIcon, MaximizeRestoreWindowIcon, MaximizeWindowIcon, MinimizeWindowIcon } from "@/components/icons";

const TraficLightIconBtn = (props: JSX.IntrinsicAttributes & React.ComponentProps<"button">) => (
    <button
        {...props}
        className={cn(
            "inline-flex h-full w-[46px] cursor-default items-center justify-center rounded-none bg-transparent text-black/90 dark:text-white",
            props.className,
        )}>
        {props.children}
    </button>
);

export function TraficLights({ className }: { className?: string }) {
    const { isWindowMaximized, minimizeWindow, toggleMaximizeWindow, closeWindow } = useWindow();

    return (
        <div className={cn("flex h-full items-center justify-end", className)}>
            <TraficLightIconBtn
                onClick={minimizeWindow}
                className="hover:bg-black/[.05] active:bg-black/[.03] dark:hover:bg-white/[.06] dark:active:bg-white/[.04]">
                <MinimizeWindowIcon />
            </TraficLightIconBtn>
            <TraficLightIconBtn
                onClick={toggleMaximizeWindow}
                className="hover:bg-black/[.05] active:bg-black/[.03] dark:hover:bg-white/[.06] dark:active:bg-white/[.04]">
                {!isWindowMaximized ? <MaximizeWindowIcon /> : <MaximizeRestoreWindowIcon />}
            </TraficLightIconBtn>
            <TraficLightIconBtn
                onClick={closeWindow}
                className="hover:bg-[#c42b1c] hover:text-white active:bg-[#c42b1c]/90">
                <CloseWindowIcon />
            </TraficLightIconBtn>
        </div>
    );
}

