import { Router } from "express";
import getLogger from "../../utils/logger.js";
import X32 from "../../controller/x32/X32.js";
import X32Channel from "../../controller/x32/X32Channel.js";
import X32Sub from "../../controller/x32/MC.js";

export default function (): Router {
    const app = Router();

    const m1 = new X32Channel("01");
    const m2 = new X32Channel("02");
    const m3 = new X32Channel("03");
    const h1 = new X32Channel("04");
    const bluetooth = new X32Channel("09");
    const beamer = new X32Channel("15");
    const mc = new X32Sub();

    app.post("/x32/load-scene/default", (req, res) => {
        try {
            X32.loadScene(0);
            res.status(200).send();
        } catch (err) {
            getLogger("x32")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/x32/channel/:channel/mute-toggle", (req, res) => {
        try {
            switch (req.params.channel) {
                case "m1":
                    m1.toggleMute();
                    break;
                case "m2":
                    m2.toggleMute();
                    break;
                case "m3":
                    m3.toggleMute();
                    break;
                case "h1":
                    h1.toggleMute();
                    break;
                case "bluetooth":
                    bluetooth.toggleMute();
                    break;
                case "beamer":
                    beamer.toggleMute();
                    break;
                case "sub":
                    mc.toggleMute();
                    break;
            }
            res.status(200).send();
        } catch (err) {
            getLogger("x32")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/x32/channel/:channel/fader/add", (req, res) => {
        try {
            switch (req.params.channel) {
                case "m1":
                    m1.faderPlus();
                    break;
                case "m2":
                    m2.faderPlus();
                    break;
                case "m3":
                    m3.faderPlus();
                    break;
                case "h1":
                    h1.faderPlus();
                    break;
                case "bluetooth":
                    bluetooth.faderPlus();
                    break;
                case "beamer":
                    beamer.faderPlus();
                    break;
                case "sub":
                    mc.faderPlus();
                    break;
            }
            res.status(200).send();
        } catch (err) {
            getLogger("x32")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    app.post("/x32/channel/:channel/fader/min", (req, res) => {
        try {
            switch (req.params.channel) {
                case "m1":
                    m1.faderMinus();
                    break;
                case "m2":
                    m2.faderMinus();
                    break;
                case "m3":
                    m3.faderMinus();
                    break;
                case "h1":
                    h1.faderMinus();
                    break;
                case "bluetooth":
                    bluetooth.faderMinus();
                    break;
                case "beamer":
                    beamer.faderMinus();
                    break;
                case "sub":
                    mc.faderMinus();
                    break;
            }
            res.status(200).send();
        } catch (err) {
            getLogger("x32")("ERROR", JSON.stringify(err));
            res.status(500).send();
        }
    });

    return app;
}
