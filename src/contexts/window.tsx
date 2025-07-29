import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { tryCatch } from "@/lib/try-catch";
import { toast } from "sonner";
import { error } from "@tauri-apps/plugin-log";

interface WindowContextType {
    isWindowMaximized: boolean;
    minimizeWindow: () => Promise<void>;
    toggleMaximizeWindow: () => Promise<void>;
    closeWindow: () => Promise<void>;
    updateIsWindowMaximized: () => Promise<void>;
    maximizeWindow: () => Promise<void>;
    unmaximizeWindow: () => Promise<void>;
}

const WindowContext = createContext<WindowContextType | undefined>(undefined);

export const WindowProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [isWindowMaximized, setIsWindowMaximized] = useState(false);
    const appWindow = getCurrentWindow();

    const updateIsWindowMaximized = useCallback(async () => {
        const result = await tryCatch(appWindow.isMaximized());
        if (result.error) {
            error(`failed to check window state: ${result.error}`);
            toast.error("Failed to check window state.");
            return;
        }
        setIsWindowMaximized(result.data);
    }, [appWindow]);

    const minimizeWindow = async () => {
        if (appWindow) {
            const result = await tryCatch(appWindow.minimize());
            if (result.error) {
                error(`failed to minimize window: ${result.error}`);
                toast.error("Failed to minimize window.");
                return;
            }
        }
    };

    const toggleMaximizeWindow = async () => {
        if (appWindow) {
            const result = await tryCatch(appWindow.toggleMaximize());
            if (result.error) {
                error(`failed to toggle maximize window: ${result.error}`);
                toast.error("Failed to toggle maximize window.");
                return;
            }
        }
    };

    const maximizeWindow = async () => {
        if (appWindow) {
            const result = await tryCatch(appWindow.maximize());
            if (result.error) {
                error(`failed to maximize window: ${result.error}`);
                toast.error("Failed to maximize window.");
                return;
            }
        }
    };

    const unmaximizeWindow = async () => {
        if (appWindow) {
            const result = await tryCatch(appWindow.unmaximize());
            if (result.error) {
                error(`failed to unmaximize window: ${result.error}`);
                toast.error("Failed to unmaximize window.");
                return;
            }
        }
    };

    const closeWindow = async () => {
        if (appWindow) {
            const result = await tryCatch(appWindow.close());
            if (result.error) {
                error(`failed to close window: ${result.error}`);
                toast.error("Failed to close window.");
                return;
            }
        }
    };

    useEffect(() => {
        updateIsWindowMaximized();

        let unlisten: () => void = () => {};
        const listen = async () => {
            unlisten = await appWindow.onResized(() => {
                updateIsWindowMaximized();
            });
        };

        listen();
        return () => unlisten?.();
    }, [appWindow, updateIsWindowMaximized]);

    return (
        <WindowContext.Provider
            value={{
                isWindowMaximized,
                minimizeWindow,
                toggleMaximizeWindow,
                closeWindow,
                updateIsWindowMaximized,
                maximizeWindow,
                unmaximizeWindow,
            }}>
            {children}
        </WindowContext.Provider>
    );
};

export const useWindow = () => {
    const context = useContext(WindowContext);
    if (!context) {
        throw new Error("useWindow must be used within a WindowProvider");
    }
    return context;
};
