import { ViewButton } from "@/components/view-button";
import type { ViewLocation } from "@/views";
import { BackArrowIcon, PowerPlugOffIcon, PowerPlugOnIcon } from "@/components/icons";
import { ViewCard } from "@/components/view-card";
import { setBeamerPower } from "@/lib/beamer";
import { toast } from "sonner";

export function BeamerView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    const onButtonClick = async (powerState: boolean) => {
        if (await setBeamerPower(powerState)) {
            toast.success(`Turned beamer '${powerState ? "on" : "off"}'.`);
        } else {
            toast.error(`Failed to set beamer power state.`);
        }
    };

    return (
        <div className="grid min-h-[inherit] grid-cols-2 grid-rows-2 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <ViewButton
                title="Back to start"
                description="Return to the main menu"
                icon={<BackArrowIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("start")}
            />
            <ViewCard>
                <span className="text-2xl font-semibold">Connecting Your Device</span>
                <span className="text-muted-foreground text-center text-sm text-pretty">
                    There are two HDMI inputs in the auditorium: <br />
                    One is under the stage on the left (facing the stage). <br />
                    The other is in the technician&apos;s booth at the back. <br />
                    For further instructions, please refer to the documentation or help section.
                </span>
            </ViewCard>
            <ViewButton
                title="Beamer On"
                description="Not implemented yet"
                icon={<PowerPlugOnIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onButtonClick(true)}
            />
            <ViewButton
                title="Beamer Off"
                description="Not implemented yet"
                icon={<PowerPlugOffIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onButtonClick(false)}
            />
        </div>
    );
}
