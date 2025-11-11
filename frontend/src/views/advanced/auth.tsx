import { ViewButton } from "@/components/view-button";
import { BackArrowIcon } from "@/components/icons";
import { ViewLocation } from "..";
import { ViewCard } from "@/components/view-card";
import { PinInput } from "@/components/pin-input";
import { useState } from "react";
import { toast } from "sonner";
import { checkAdvancedPin } from "@/lib/advanced";

export function AdvancedAuthenticateView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    const [pinInputMessage, setPinInputMessage] = useState<
        "Please enter your pin to proceed." | "Please enter a valid pin." | "Pin incorrect."
    >("Please enter your pin to proceed.");

    const handlePinEnter = async (pin: number[]) => {
        if (pin.length === 0) {
            setPinInputMessage("Please enter a valid pin.");
            return;
        }

        const result = await checkAdvancedPin(pin.join(""));
        if (result === null) {
            toast.error("Failed to authenticate.");
            return;
        } else if (result) {
            toast.success("Authenticated successfully.");
            onLocationSwitch("start");
            return;
        } else {
            setPinInputMessage("Pin incorrect.");
            return;
        }
    };

    return (
        <div className="grid min-h-[inherit] grid-cols-2 grid-rows-2 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <ViewButton
                className="col-start-1 row-span-2 row-start-1"
                title="Back to start"
                description="Return to main menu"
                icon={<BackArrowIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("start")}
            />
            <ViewCard className="col-start-2 row-span-2 row-start-1 justify-start">
                <PinInput
                    className="mt-2"
                    onEnter={handlePinEnter}
                    message={pinInputMessage}
                    isError={pinInputMessage !== "Please enter your pin to proceed."}
                    onInputChange={(pin) =>
                        setPinInputMessage(
                            pin.length === 0 ? "Please enter a valid pin." : "Please enter your pin to proceed.",
                        )
                    }
                />
            </ViewCard>
        </div>
    );
}
