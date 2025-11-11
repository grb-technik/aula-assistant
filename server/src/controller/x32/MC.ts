import { fromBuffer, OscArgOutput, OscMessageOutput } from "osc-min";
import OSCx32Service from "../../services/OSCx32Service.js";

export default class X32Sub {
    private mix_on: "ON" | "OFF";
    private mix_fader: number;

    constructor() {
        this.mix_on = "OFF";
        this.mix_fader = 0.75;
        OSCx32Service.getInstance().socket.on("message", (buffer, _) => {
            const data = fromBuffer(buffer);
            if (data.oscType === "message") {
                const msg = data;
                if (msg.address.startsWith("/main/m")) {
                    switch (msg.address) {
                        case `/main/m/mix/on`:
                            this.mix_on = this.getValue(msg) as "ON" | "OFF";
                            return;
                        case `/main/m/mix/fader`:
                            this.mix_fader = this.getValue(msg) as number;
                            return;
                    }
                }
            }
        });
    }

    toggleMute() {
        this.mix_on = this.mix_on === "ON" ? "OFF" : "ON";
        OSCx32Service.getInstance().sendOSC(`/main/m/mix/on`, this.mix_on);
    }

    faderPlus() {
        this.mix_fader = this.mix_fader + 0.015;
        OSCx32Service.getInstance().sendOSC(`/main/m/mix/fader`, this.mix_fader);
    }

    faderMinus() {
        this.mix_fader = this.mix_fader - 0.015;
        OSCx32Service.getInstance().sendOSC(`/main/m/mix/fader`, this.mix_fader);
    }

    private getValue(msg: OscMessageOutput): any {
        if (Array.isArray(msg.args)) {
            return msg.args[0].value;
        } else {
            return (msg.args as OscArgOutput).value;
        }
    }
}
