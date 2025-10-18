import { error } from "@tauri-apps/plugin-log";
import { tryCatch } from "./try-catch";
import { invoke } from "@tauri-apps/api/core";

export async function checkAdvancedPin(pin: string): Promise<boolean | null> {
    const result = await tryCatch(
        invoke<boolean>("check_advanced_pin", {
            pin: pin,
        }),
    )

    if (result.error) {
        error(`Failed to authenticate: failed to invoke check_advanced_pin: ${result.error.message}`);
        return null;
    }

    return result.data;
}