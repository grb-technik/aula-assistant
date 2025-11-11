import {
    Menubar,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarSeparator,
    MenubarShortcut,
    MenubarTrigger,
} from "@/components/ui/menubar";
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuTrigger,
    ContextMenuItem,
    ContextMenuShortcut,
} from "@/components/ui/context-menu";
import { TraficLights } from "./trafic-lights";
import { useWindow } from "@/contexts/window";
import { useEffect, useState } from "react";
import * as os_info from "@tauri-apps/plugin-os";
import { toast } from "sonner";
import { useStartupData } from "@/contexts/startup-data";
import { showMessageDialog } from "@/lib/dialog";

export function MenuAppBar() {
    const { isWindowMaximized, minimizeWindow, maximizeWindow, unmaximizeWindow, toggleMaximizeWindow, closeWindow } =
        useWindow();
    const startUpData = useStartupData();
    const [isFullscreen, setIsFullscreen] = useState<boolean>(false);

    const toggleFullscreen = () => {
        if (isFullscreen) {
            unmaximizeWindow();
            setIsFullscreen(false);
        } else {
            maximizeWindow();
            setIsFullscreen(true);
        }
    };

    const showAboutDialog = async () => {
        const dialogResult = await showMessageDialog(
            `Aula Assistant
Version: ${startUpData.build.version}
${startUpData.build.commit ? "Commit: " + startUpData.build.commit.long_id + "\n" : ""}Date: ${startUpData.build.date}
OS: ${os_info.platform()} ${os_info.arch()} ${os_info.version()}`,
            { title: "Aula Assistant", kind: "info" },
        );

        if (!dialogResult) {
            toast.error("Failed to show about dialog.");
        }
    };

    useEffect(() => {
        const handleKeydown = (event: KeyboardEvent) => {
            if (event.key === "F11") {
                event.preventDefault();
                toggleFullscreen();
            }
        };

        document.addEventListener("keydown", handleKeydown);

        return () => {
            document.removeEventListener("keydown", handleKeydown);
        };
    });

    useEffect(() => {
        if (startUpData.open_in_fullscreen) {
            maximizeWindow();
            setIsFullscreen(true);
        }
    }, [startUpData]);

    return (
        <header
            data-tauri-drag-region
            className="bg-background fixed top-0 right-0 left-0 z-50 flex h-10 items-center border-b py-2 pl-2 text-neutral-200">
            <ContextMenu>
                <ContextMenuTrigger>
                    <img src="/favicon.ico" className="aspect-square h-6" />
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem onClick={unmaximizeWindow} disabled={!isWindowMaximized}>
                        Restore
                    </ContextMenuItem>
                    <ContextMenuItem onClick={maximizeWindow} disabled={isWindowMaximized}>
                        Maximize
                    </ContextMenuItem>
                    <ContextMenuItem onClick={minimizeWindow}>Minimize</ContextMenuItem>
                    <ContextMenuItem onClick={closeWindow}>
                        Close
                        <ContextMenuShortcut>Alt + F4</ContextMenuShortcut>
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>

            <Menubar tabIndex={-1} className="h-full border-0 bg-transparent shadow-none">
                <MenubarMenu>
                    <MenubarTrigger tabIndex={-1}>Window</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={toggleFullscreen}>
                            {isFullscreen ? "Exit Full Screen" : "Full Screen"}
                            <MenubarShortcut>F11</MenubarShortcut>
                        </MenubarItem>
                        <MenubarItem onClick={toggleMaximizeWindow} disabled={isWindowMaximized}>
                            Maximize
                        </MenubarItem>
                        <MenubarItem onClick={minimizeWindow}>Minimize</MenubarItem>
                        <MenubarItem onClick={closeWindow}>
                            Close
                            <MenubarShortcut>Alt + F4</MenubarShortcut>
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>

                <MenubarMenu>
                    <MenubarTrigger tabIndex={-1}>Help</MenubarTrigger>
                    <MenubarContent>
                        <a target="_blank" rel="noreferrer" href="https://aula-assistant.teschnik.de/docs">
                            <MenubarItem>Documentation</MenubarItem>
                        </a>
                        <a
                            target="_blank"
                            rel="noreferrer"
                            href="https://github.com/grb-technik/aula-assistant/blob/master/CHANGELOG.md">
                            <MenubarItem>Show Release Notes</MenubarItem>
                        </a>

                        <MenubarSeparator />

                        <a
                            target="_blank"
                            rel="noreferrer"
                            href="https://github.com/grb-technik/aula-assistant/issues/new/choose">
                            <MenubarItem>Report Issue</MenubarItem>
                        </a>

                        <MenubarSeparator />

                        <a
                            target="_blank"
                            rel="noreferrer"
                            href="https://github.com/grb-technik/aula-assistant/blob/master/LICENSE.txt">
                            <MenubarItem>View License</MenubarItem>
                        </a>

                        <MenubarSeparator />

                        <a
                            target="_blank"
                            rel="noreferrer"
                            href="https://github.com/grb-technik/aula-assistant/releases">
                            <MenubarItem>Check for Updates</MenubarItem>
                        </a>

                        <MenubarSeparator />

                        <MenubarItem onClick={showAboutDialog}>About</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>

            {isFullscreen ? null : <TraficLights className="absolute right-0" />}
        </header>
    );
}
