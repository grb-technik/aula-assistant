import { SplashScreen } from "@/components/layout/splash";
import { tryCatch } from "@/lib/try-catch";
import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";
import React, { createContext, useContext, useEffect, useState } from "react";
import { toast } from "sonner";

interface StartupDataType {
    build: {
        version: string;
        date: string;
        commit: {
            date: string;
            short_id: string;
            long_id: string;
        } | null;
    };
    show_appbar: boolean;
    open_in_fullscreen: boolean;
}

const StartUpDataContext = createContext<StartupDataType | undefined>(undefined);

export const StartupDataProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [startupData, setStartupData] = useState<StartupDataType | undefined>(undefined);

    useEffect(() => {
        const loadStartupData = async () => {
            const delay = new Promise((resolve) => setTimeout(resolve, 1000)); // 1 second delay to show splash screen
            const fetchStartupData = tryCatch(invoke<StartupDataType>("get_startup_data"));

            const [_, result] = await Promise.all([delay, fetchStartupData]);

            if (result.error) {
                error(`failed to retrieve startup data: failed to invoke get_startup_data: ${result.error.message}`);
                toast.error("Failed to retrieve startup data.");
                return;
            }

            setStartupData(result.data);
        };

        loadStartupData();
    }, [setStartupData]);

    if (!startupData) {
        return <SplashScreen />;
    }

    return <StartUpDataContext.Provider value={startupData}>{children}</StartUpDataContext.Provider>;
};

export const useStartupData = () => {
    const context = useContext(StartUpDataContext);
    if (!context) {
        throw new Error("useStartupData must be used within a StartupDataProvider");
    }
    return context;
};
