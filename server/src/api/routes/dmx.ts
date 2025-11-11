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

    app.post("/dmx/scene/stage-cw", async (req, res) => {
        try {
            await dmx.stageCW();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/stage-off", async (req, res) => {
        try {
            await dmx.stageOff();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/stage-v1", async (req, res) => {
        try {
            await dmx.stageV1();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/stage-v2", async (req, res) => {
        try {
            await dmx.stageV2();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/stage-v3", async (req, res) => {
        try {
            await dmx.stageV3();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/spot-left", async (req, res) => {
        try {
            await dmx.spotLeft();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/spot-mid", async (req, res) => {
        try {
            await dmx.spotMid();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/spot-off", async (req, res) => {
        try {
            await dmx.spotOff();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/accent-v1", async (req, res) => {
        try {
            await dmx.accentV1();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/accent-v2", async (req, res) => {
        try {
            await dmx.accentV2();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/dmx/scene/disco", async (req, res) => {
        try {
            await dmx.disco();
            res.status(200).send();
        } catch (err) {
            getLogger("artnet")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    return app;
}
