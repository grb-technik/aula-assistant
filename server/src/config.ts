import { z } from "zod";

const configSchema = z.object({
    port: z.number().positive().int(),
    cors_origins: z.array(z.string()),
    x32: z.object({
        bind: z.object({
            host: z.string().nonempty(),
            port: z.number().positive().int(),
        }),
        target: z.object({
            host: z.string().nonempty(),
            port: z.number().positive().int(),
        }),
    }),
});

export { configSchema };
