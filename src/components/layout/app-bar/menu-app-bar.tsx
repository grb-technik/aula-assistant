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

export function MenuAppBar() {
    const { isWindowMaximized, minimizeWindow, maximizeWindow, unmaximizeWindow, toggleMaximizeWindow, closeWindow } =
        useWindow();

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

            <Menubar className="h-full border-0 bg-transparent shadow-none">
                <MenubarMenu>
                    <MenubarTrigger>Window</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onClick={toggleMaximizeWindow}>Maximize</MenubarItem>
                        <MenubarItem onClick={minimizeWindow}>Minimize</MenubarItem>
                        <MenubarItem onClick={closeWindow}>
                            Close
                            <MenubarShortcut>Alt + F4</MenubarShortcut>
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>Help</MenubarTrigger>
                    <MenubarContent>
                        <a
                            target="_blank"
                            href="https://github.com/grb-technik/aula_assistant/blob/master/LICENSE.txt">
                            <MenubarItem>View License</MenubarItem>
                        </a>
                        <MenubarSeparator />
                        <a
                            target="_blank"
                            href="https://github.com/grb-technik/aula_assistant/blob/master/README.md">
                            <MenubarItem>About</MenubarItem>
                        </a>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>

            <TraficLights className="absolute right-0" />
        </header>
    );
}
