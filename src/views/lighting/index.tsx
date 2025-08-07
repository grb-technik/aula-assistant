import { MiniViewButton } from "@/components/mini-view-button";
import { BackArrowIcon, MovingHeadIcon } from "@/components/icons";
import { ViewLocation } from "..";
import { toast } from "sonner";
import { error } from "@tauri-apps/plugin-log";
import { tryCatch } from "@/lib/try-catch";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { ViewButton } from "@/components/view-button";

export function LightingView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    const [scenes, setScenes] = useState<string[] | undefined>(undefined);

    const loadArtnetScenes = async () => {
        const result = await tryCatch(invoke<string[]>("get_all_artnet_scenes"));
        if (result.error) {
            error(`failed to retrieve lighting scenes: failed to invoke get_all_artnet_scenes: ${result.error.message}`);
            toast.error("Failed to retrieve lighting scenes.");
            return;
        }
        return result.data || [];
    };

    const onSceneClick = (_scenes: string) => {
        // TODO: impl invoke handler for scene click
    };

    useEffect(() => {
        loadArtnetScenes().then((scenes: string[] | undefined) => {
            setScenes(scenes);
        });
    }, [setScenes]);

    if (!scenes || scenes.length === 0) {
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
                        onSceneClick(scene);
                    }}
                    icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
                />
            ))}
        </div>
    );
}
