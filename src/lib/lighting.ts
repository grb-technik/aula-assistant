import { invoke } from "@tauri-apps/api/core";
import { tryCatch } from "./try-catch";
import { error } from "@tauri-apps/plugin-log";

export async function loadArtnetScenes(): Promise<string[] | null> {
    const result = await tryCatch(invoke<string[]>("get_all_artnet_scenes"));
    if (result.error) {
        error(`failed to retrieve lighting scenes: failed to invoke get_all_artnet_scenes: ${result.error.message}`);
        return null;
    }
    return result.data || [];
}

export async function runArtnetScene(sceneName: string): Promise<boolean> {
    const result = await tryCatch(invoke<void>("run_artnet_scene", { sceneName }));
    if (result.error) {
        error(
            `failed to run lighting scene '${sceneName}': failed to invoke run_artnet_scene: ${result.error.message}`,
        );
        return false;
    }
    return true;
}
