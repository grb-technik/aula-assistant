import { AppBar } from "./components/layout/app-bar";
import { Toaster } from "@/components/ui/sonner";
import { WindowProvider } from "@/contexts/window";
import { cn } from "./lib/utils";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { tryCatch } from "./lib/try-catch";
import { toast } from "sonner";

import "./styles/global.css";

function App() {
    const [showAppBar, setShowAppBar] = useState<boolean>(false);

    useEffect(() => {
        tryCatch(invoke<boolean>("get_show_appbar")).then((result) => {
            if (result.error) {
                toast.error("Failed to retrieve app bar visibility setting.");
                return;
            }

            setShowAppBar(Boolean(result.data) ?? false);
        });
    }, [invoke, setShowAppBar]);

    return (
        <>
            <WindowProvider>
                {showAppBar ? <AppBar /> : null}

                <main
                    className={cn("w-full", showAppBar ? "mt-10 min-h-[calc(100dvh-40px)]" : "mt-0 min-h-dvh")}></main>
            </WindowProvider>

            <Toaster richColors />
        </>
    );
}

export default App;

