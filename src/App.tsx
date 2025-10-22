import { Toaster } from "@/components/ui/sonner";
import { WindowProvider } from "@/contexts/window";
import { StartupDataProvider, useStartupData } from "./contexts/startup-data";
import { MenuAppBar } from "./components/layout/app-bar";
import { cn } from "./lib/utils";
import { Views } from "./views";

import "./styles/global.css";

export default function App() {
    return (
        <>
            <StartupDataProvider>
                <WindowProvider>
                    <AppComponent />
                </WindowProvider>
            </StartupDataProvider>

            <Toaster duration={1000} visibleToasts={1} position="bottom-center" richColors />
        </>
    );
}

function AppComponent() {
    const startUpData = useStartupData();

    return (
        <>
            {startUpData.show_appbar ? <MenuAppBar /> : null}

            <main
                className={cn(
                    "w-full",
                    startUpData.show_appbar ? "mt-10 min-h-[calc(100dvh-40px)]" : "mt-0 min-h-dvh",
                )}>
                <Views />
            </main>
        </>
    );
}
