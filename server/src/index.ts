import express from "express";
import cors from "cors";
import http from "http";

import env from "./env.js";
import getLogger from "./utils/logger.js";

import beamerHandler from "./api/routes/beamer.js";
import x32Handler from "./api/routes/x32.js";
import dmxHandler from "./api/routes/dmx.js";

const app = express();
const server = http.createServer(app);
const mainLogger = getLogger("main");

(async () => {
    app.use(
        cors({
            origin: env.FRONTEND_URL,
            methods: ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"],
            optionsSuccessStatus: 204,
            allowedHeaders: ["Content-Type", "Authorization", "X-Requested-With", "Accept", "Origin"],
        }),
    );
    app.use(express.json());

    app.use(beamerHandler());
    app.use(x32Handler());
    app.use(dmxHandler());

    server.listen(env.PORT, () => {
        mainLogger("INFO", `Server is running on port ${env.PORT}`);
    });
})();
