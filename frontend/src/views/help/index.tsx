import { ViewButton } from "@/components/view-button";
import { BackArrowIcon, SupportIcon } from "@/components/icons";
import { ViewLocation } from "..";
import { ViewCard } from "@/components/view-card";

export function HelpView({ onLocationSwitch }: { onLocationSwitch: (to: ViewLocation) => void }) {
    return (
        <div className="grid min-h-[inherit] grid-cols-2 grid-rows-2 gap-4 p-4 max-sm:flex max-sm:flex-col max-sm:items-center max-sm:justify-start">
            <ViewButton
                title="Back to start"
                description="Return to the main menu"
                icon={<BackArrowIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("start")}
            />
            <ViewCard>
                <div
                    className="aspect-square h-full bg-contain bg-center bg-no-repeat max-sm:h-40"
                    style={{
                        backgroundImage: "url('/qrcode_docs.svg')",
                    }}
                />
                <span className="-mb-2 text-center">View Documentation</span>
            </ViewCard>
            <ViewButton
                className="col-span-2"
                title="Ask for help"
                description="(We won't always be available)"
                icon={<SupportIcon className="fill-foreground" height={48} width={48} />}
                onClick={() => onLocationSwitch("help:support")}
            />
        </div>
    );
}
