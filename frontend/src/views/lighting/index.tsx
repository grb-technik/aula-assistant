import { MiniViewButton } from "@/components/mini-view-button";
import { BackArrowIcon, MovingHeadIcon } from "@/components/icons";
import { ViewLocation } from "..";
import { toast } from "sonner";
import { useEffect, useState } from "react";
import { ViewButton } from "@/components/view-button";
import { loadArtnetScenes, runArtnetScene } from "@/lib/lighting";

export function LightingView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    const [scenes, setScenes] = useState<string[]>([]);

    const onButtonClick = async (sceneName: string) => {
        if (await runArtnetScene(sceneName)) {
            toast.success(`Activated lighting scene '${sceneName}'.`);
        } else {
            toast.error(`Failed to run lighting scene '${sceneName}'.`);
        }
    };

    useEffect(() => {
        loadArtnetScenes().then((scenes: string[] | null) => {
            if (scenes == null) {
                toast.error("Failed to retrieve lighting scenes.");
                setScenes([]);
                return;
            }
            setScenes(scenes);
        });
    }, [setScenes]);

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
                    title={scene}
                    onClick={() => {
                        onButtonClick(scene);
                    }}
                    icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
                />
            ))}
        </div>
    );
}
