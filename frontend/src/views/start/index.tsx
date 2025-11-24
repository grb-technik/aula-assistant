import { ViewButton } from "@/components/view-button";
import { MovingHeadIcon, QuestionMarkIcon, ProjectorIcon, MusicNoteIcon } from "@/components/icons";
import type { ViewLocation } from "@/views";

export function StartView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    return (
        <div className="grid min-h-[inherit] grid-cols-2 grid-rows-2 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <ViewButton
                title="Audio"
                icon={<MusicNoteIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("audio")}
            />
            <ViewButton
                title="Beamer"
                icon={<ProjectorIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("beamer")}
            />
            <ViewButton
                title="Lighting"
                icon={<MovingHeadIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("lighting")}
            />
            <ViewButton
                title="Help"
                icon={<QuestionMarkIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("help")}
            />
        </div>
    );
}
