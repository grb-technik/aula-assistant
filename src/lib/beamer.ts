import { invoke } from "@tauri-apps/api/core";
import { tryCatch } from "./try-catch";
import { error } from "@tauri-apps/plugin-log";

export async function setBeamerPower(on: boolean): Promise<boolean> {
    const result = await tryCatch(invoke<void>("set_beamer_power_state", { power: on }));
    if (result.error) {
        error(`failed to set beamer power state: failed to invoke set_beamer_power_state: ${result.error.message}`);
        return false;
    }
    return true;
}
