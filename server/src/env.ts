import { z } from "zod";

// process.env is a Record<string, string | undefined>
const envSchema = z.object({
    LOG_DIR: z.string(),
    PORT: z.string(),
    FRONTEND_URL: z.string(),
    X32_IP: z.string(),
    BEAMER_IP: z.string(),
    SEND_ARTNET_AS_BROADCAST_ANYWAY: z.string().optional(),
});

export default envSchema.parse(process.env);
