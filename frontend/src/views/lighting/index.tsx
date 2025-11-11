import { MiniViewButton } from "@/components/mini-view-button";
import { BackArrowIcon, MovingHeadIcon } from "@/components/icons";
import { ViewLocation } from "..";
import { toast } from "sonner";
import { ViewButton } from "@/components/view-button";
import { runArtnetScene } from "@/lib/lighting";

const scenes: {
    label: string;
    value: string;
}[] = [
    {
        label: "Anlage Ein",
        value: "anlage-ein",
    },
    {
        label: "Anlage Aus",
        value: "anlage-aus",
    },
    {
        label: "Bühne Dunkel",
        value: "clear",
    },
    {
        label: "Bühne Weiß",
        value: "stage-ww",
    },
];

export function LightingView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    const onButtonClick = async (sceneName: string) => {
        if (await runArtnetScene(sceneName)) {
            toast.success(`Activated lighting scene '${sceneName}'.`);
        } else {
            toast.error(`Failed to run lighting scene '${sceneName}'.`);
        }
    };

    if (scenes.length === 0) {
        return (
            <div className="grid min-h-[inherit] gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
                <ViewButton
                    onClick={() => onLocationSwitch("start")}
                    title="Back to start"
                    description="No scenes available."
                    icon={<BackArrowIcon className="fill-foreground" height={32} width={32} />}
                />
            </div>
        );
    }

    return (
        <div className="grid min-h-[inherit] grid-cols-4 grid-rows-3 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <MiniViewButton
                onClick={() => onLocationSwitch("start")}
                title="Back to start"
                icon={<BackArrowIcon className="fill-foreground" height={32} width={32} />}
            />

            {scenes.map((scene, index) => (
                <MiniViewButton
                    key={index}
                    title={scene.label}
                    onClick={() => {
                        onButtonClick(scene.value);
                    }}
                    icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
                />
            ))}
        </div>
    );
}
