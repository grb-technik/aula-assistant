import { useState } from "react";
import { StartView } from "./start";
import { AudioView } from "./audio";
import { BeamerView } from "./beamer";
import { LightingView } from "./lighting";
import { HelpView } from "./help";
import { HelpSupportView } from "./help/support";
import { AdvancedAuthenticateView } from "./advanced/auth";

export type ViewLocation = "start" | "help" | "audio" | "beamer" | "lighting" | "help:support" | "advanced:auth";

export function Views() {
    const [location, setLocation] = useState<ViewLocation>("start");

    switch (location) {
        case "start":
            return <StartView onLocationSwitch={(to) => setLocation(to)} />;
        case "audio":
            return <AudioView onLocationSwitch={(to) => setLocation(to)} />;
        case "beamer":
            return <BeamerView onLocationSwitch={(to) => setLocation(to)} />;
        case "lighting":
            return <LightingView onLocationSwitch={(to) => setLocation(to)} />;
        case "help":
            return <HelpView onLocationSwitch={(to) => setLocation(to)} />;
        case "help:support":
            return <HelpSupportView onLocationSwitch={(to) => setLocation(to)} />;
        case "advanced:auth":
            return <AdvancedAuthenticateView onLocationSwitch={(to) => setLocation(to)} />;
        default:
            throw new Error(`No view component set for location: ${location} (views/index.tsx)`);
    }
}
