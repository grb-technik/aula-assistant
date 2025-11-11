import { Router } from "express";
import getLogger from "../../utils/logger.js";
import BeamerController from "../../controller/BeamerController.js";

export default function (): Router {
    const app = Router();

    app.post("/beamer/power/on", async (req, res) => {
        try {
            await BeamerController.powerOn();
            res.status(200).send();
        } catch (err) {
            getLogger("beamer")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/beamer/power/off", async (req, res) => {
        try {
            await BeamerController.powerOff();
            res.status(200).send();
        } catch (err) {
            getLogger("beamer")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    return app;
}
