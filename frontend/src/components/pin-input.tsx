import { cn } from "@/lib/utils";
import { useState } from "react";

export function PinInput({
    className,
    onEnter,
    onInputChange,
    message,
    isError = false,
}: {
    className?: string;
    onEnter?: (pin: number[]) => void;
    onInputChange?: (pin: number[]) => void;
    message: string;
    isError: boolean;
}) {
    const [pin, setPin] = useState<number[]>([]);

    const addDigit = (digit: number) => {
        setPin((prev) => {
            const newPin = [...prev, digit];
            onInputChange?.(newPin);
            return newPin;
        });
    };

    const removeLastDigit = () => {
        setPin((prev) => {
            const newPin = prev.slice(0, -1);
            onInputChange?.(newPin);
            return newPin;
        });
    };

    return (
        <>
            <span className="inline-flex w-full items-center justify-center rounded-md border-2 p-2 text-xl font-semibold">
                {pin.length > 0 ? (
                    <span className="text-foreground" role="textbox" aria-hidden>
                        {Array.from({ length: pin.length }, (_, i) => (
                            <span
                                key={i}
                                className="bg-primary mx-1 mb-0.5 inline-block aspect-square h-3 rounded-full"></span>
                        ))}
                    </span>
                ) : (
                    <span className="text-foreground/80">Enter Pin</span>
                )}
            </span>
            <span
                className={cn(
                    "text-center text-sm text-pretty",
                    isError ? "text-destructive" : "text-muted-foreground",
                )}>
                {message}
            </span>
            <div className={cn("grid h-full w-full grid-cols-3 grid-rows-4 gap-2", className)}>
                {[7, 8, 9, 4, 5, 6, 1, 2, 3].map((digit) => (
                    <PinInputButton key={digit} onClick={() => addDigit(digit)}>
                        {digit}
                    </PinInputButton>
                ))}

                <PinInputButton className="text-destructive" onClick={removeLastDigit}>
                    Del
                </PinInputButton>
                <PinInputButton onClick={() => addDigit(0)}>0</PinInputButton>
                <PinInputButton
                    disabled={pin.length === 0}
                    className="bg-primary text-primary-foreground hover:bg-primary/80 active:bg-white"
                    onClick={() => {
                        if (onEnter) {
                            onEnter(pin);
                        }
                        setPin([]);
                    }}>
                    Enter
                </PinInputButton>
            </div>
        </>
    );
}

function PinInputButton({
    children,
    className,
    onClick,
    disabled,
}: {
    disabled?: boolean;
    children: React.ReactNode;
    className?: string;
    onClick?: () => void;
}) {
    return (
        <div className="bg-background h-full w-full rounded-md">
            <button
                disabled={disabled}
                onMouseDown={(e) => {
                    if (e.button === 0) return;
                    e.preventDefault();
                    e.currentTarget.click();
                }}
                onClick={onClick}
                className={cn(
                    "bg-secondary active:bg-primary/40 hover:bg-secondary/80 focus-visible:border-ring focus-visible:ring-ring/50 h-full w-full rounded-md border-2 font-medium transition-all duration-100 ease-in outline-none focus-visible:border-0 focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50",
                    className,
                )}>
                {children}
            </button>
        </div>
    );
}
