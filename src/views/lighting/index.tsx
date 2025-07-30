import { MiniViewButton } from "@/components/mini-view-button";
import { BackArrowIcon, MovingHeadIcon } from "@/components/icons";
import { ViewLocation } from "..";

export function LightingView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    return (
        <div className="grid min-h-[inherit] grid-cols-4 grid-rows-3 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <MiniViewButton
                onClick={() => onLocationSwitch("start")}
                title="Back to start"
                className="fill-foreground"
                icon={<BackArrowIcon className="fill-foreground" height={32} width={32} />}
            />

            <MiniViewButton
                title="Szene 1"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 2"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 3"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />

            <MiniViewButton
                title="Szene 4"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 5"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 6"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 7"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />

            <MiniViewButton
                title="Szene 8"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 9"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 10"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
            <MiniViewButton
                title="Szene 11"
                icon={<MovingHeadIcon className="fill-foreground" height={32} width={32} />}
            />
        </div>
    );
}
