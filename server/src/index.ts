import express from "express";
import cors from "cors";
import http from "node:http";
import { readFileSync } from "node:fs";
import { configSchema } from "./config.js";

const config = configSchema.parse(
    JSON.parse(readFileSync(new URL("../config.json", import.meta.url), "utf8"))
);

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
