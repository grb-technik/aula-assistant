import { Router } from "express";
import DMXController from "../../controller/dmx/DMXController.js";
import getLogger from "../../utils/logger.js";

export default function (): Router {
    const app = Router();

    const dmx = new DMXController();

    app.post("/dmx/scene/anlage-an", async (req, res) => {
        try {
            await dmx.sceneAnlageAn();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/anlage-aus", async (req, res) => {
        try {
            await dmx.sceneAnlageAus();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/clear", async (req, res) => {
        try {
            await dmx.clearScenes();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/stage-ww", async (req, res) => {
        try {
            await dmx.stageWW();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    return app;
}
