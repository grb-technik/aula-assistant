import express from "express";
import cors from "cors";
import http from "node:http";
import { readFileSync } from "node:fs";
import { configSchema } from "./config.js";
import { createOSCClient } from "./lib/core/osc.js"
import { x32StartHeartbeat } from "./lib/x32.js"

async function main() {
    const config = (() => {
        try {
            return configSchema.parse(
                JSON.parse(readFileSync(new URL("../config.json", import.meta.url), "utf8"))
            );
        } catch (err) {
            console.error("ERROR Invalid configuration:", err);
            process.exit(1);
        }
    })();

    const { data: x32, error } = createOSCClient({
        bind: {
            host: config.x32.bind.host,
            port: config.x32.bind.port,
        },
        target: {
            host: config.x32.target.host,
            port: config.x32.target.port,
        },
    });

    if (error) {
        console.error("ERROR Failed to create OSC client:", error);
        process.exit(1);
    }

    x32StartHeartbeat(x32, (err) => {
        if (err) {
            console.error("ERROR X32 Heartbeat error:", err);
            process.exit(1);
        }
    });

    const app = express();
    const server = http.createServer(app);

    app.use(
        cors({
            origin: config.cors_origins,
            methods: ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"],
            optionsSuccessStatus: 204,
            allowedHeaders: ["Content-Type", "Authorization", "X-Requested-With", "Accept", "Origin"],
        }),
    );
    app.use(express.json());

    server.listen(config.port, () => {
        console.log(`INFO Server is running on port ${config.port}`);
    });
}

void main();
