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
    artnet: z.object({
        bind: z.object({
            host: z.string().nonempty(),
            port: z.number().positive().int(),
        }),
        target: z.object({
            host: z.string().nonempty(),
            port: z.number().positive().int(),
        }),
        broadcast: z.boolean(),
        universe: z.number().min(0).max(32767),
    }),
    ptmahdbt42: z.object({
        host: z.string().nonempty(),
        port: z.number().positive().int(),
    }),
});

type Config = z.infer<typeof configSchema>;

export { configSchema, Config };

