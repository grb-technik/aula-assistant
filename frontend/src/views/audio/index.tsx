import { BackArrowIcon, VolumeLowerIcon, VolumeOffIcon, VolumeUpIcon } from "@/components/icons";
import type { ViewLocation } from "@/views";
import { MiniViewButton } from "@/components/mini-view-button";
import { CHANNELS, faderMinus, faderPlus, muteToggle, ChannelType } from "@/lib/audio";
import { toast } from "sonner";

export function AudioView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    const onButtonClick = async (action: "mute-toggle" | "faderup" | "faderdown", channel: ChannelType) => {
        switch (action) {
            case "mute-toggle":
                if (await muteToggle(channel)) {
                    toast.success(`Toggled mute for channel '${channel}'.`);
                } else {
                    toast.error(`Failed to toggle mute for channel '${channel}'.`);
                }
                break;
            case "faderup":
                if (await faderPlus(channel)) {
                    toast.success(`Increased fader for channel '${channel}'.`);
                } else {
                    toast.error(`Failed to increase fader for channel '${channel}'.`);
                }
                break;
            case "faderdown":
                if (await faderMinus(channel)) {
                    toast.success(`Decreased fader for channel '${channel}'.`);
                } else {
                    toast.error(`Failed to decrease fader for channel '${channel}'.`);
                }
                break;
        }
    };

    return (
        <div className="grid min-h-[inherit] grid-cols-8 grid-rows-1 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <MiniViewButton
                onClick={() => onLocationSwitch("start")}
                title="Back to start"
                icon={<BackArrowIcon className="fill-foreground" height={32} width={32} />}
            />

            {CHANNELS.map((channel, index) => (
                <div key={index} className="grid grid-cols-1 grid-rows-3 justify-items-center gap-2">
                    <MiniViewButton
                        title={channel}
                        onClick={() => {
                            onButtonClick("mute-toggle", channel);
                        }}
                        icon={<VolumeOffIcon className="fill-foreground" height={32} width={32} />}
                    />
                    <MiniViewButton
                        title={channel}
                        onClick={() => {
                            onButtonClick("faderup", channel);
                        }}
                        icon={<VolumeUpIcon className="fill-foreground" height={32} width={32} />}
                    />
                    <MiniViewButton
                        title={channel}
                        onClick={() => {
                            onButtonClick("faderdown", channel);
                        }}
                        icon={<VolumeLowerIcon className="fill-foreground" height={32} width={32} />}
                    />
                </div>
            ))}
        </div>
    );
}
