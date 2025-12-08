import { z } from "zod";

const configSchema = z.object({
    port: z.number().positive().int(),
    cors_origins: z.array(z.string()),
});

export { configSchema };
