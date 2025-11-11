import { MessageDialogOptions, message } from "@tauri-apps/plugin-dialog";
import { tryCatch } from "./utils";
import { error } from "@tauri-apps/plugin-log";

export async function showMessageDialog(msg: string, options?: string | MessageDialogOptions): Promise<boolean> {
    const result = await tryCatch(message(msg, options));

    if (result.error) {
        error(`failed to show about dialog: ${result.error}`);
        return false;
    }

    return true;
}
