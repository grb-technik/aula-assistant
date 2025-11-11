import { Button } from "@/components/ui/button";
import { ViewLocation } from "..";

export function AudioView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    return (
        <div className="grid min-h-[inherit] place-items-center p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <Button variant={"secondary"} onClick={() => onLocationSwitch("start")} className="m-4">
                Back to Start
            </Button>
        </div>
    );
}
