import React, { createContext, useContext, useEffect, useState, useCallback } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

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
        const _isMaximized = await appWindow.isMaximized();
        setIsWindowMaximized(_isMaximized);
    }, [appWindow]);

    const minimizeWindow = async () => {
        if (appWindow) {
            await appWindow.minimize();
        }
    };

    const toggleMaximizeWindow = async () => {
        if (appWindow) {
            await appWindow.toggleMaximize();
        }
    };

    const maximizeWindow = async () => {
        if (appWindow) {
            await appWindow.maximize();
        }
    };

    const unmaximizeWindow = async () => {
        if (appWindow) {
            await appWindow.unmaximize();
        }
    };

    const closeWindow = async () => {
        if (appWindow) {
            await appWindow.close();
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
