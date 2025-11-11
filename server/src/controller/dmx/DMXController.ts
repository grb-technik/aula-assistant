import ArtNetService from "../../services/ArtNetService.js";
import PATCH from "./patch.js";

class DMXController {
    private data: Uint8ClampedArray = new Uint8ClampedArray(512).fill(0);

    constructor() {
        ArtNetService.addNode(0, "255.255.255.255");
    }

    clearScenes(): Promise<void> {
        this.data = new Uint8ClampedArray(512).fill(0);
        for (let i = 0; i < 24; i++) {
            // make single channel fixtures on
            this.data[i] = 255;
        }
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    sceneAnlageAn(): Promise<void> {
        this.data = new Uint8ClampedArray(512).fill(0);
        for (let i = 0; i < 24; i++) {
            // make single channel fixtures on
            this.data[i] = 255;
        }
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    sceneAnlageAus(): Promise<void> {
        this.data = new Uint8ClampedArray(512).fill(0);
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageWW(): Promise<void> {
        this.data[PATCH.CWWW_L_DIMMER] = 255;
        this.data[PATCH.CWWW_R_DIMMER] = 255;
        this.data[PATCH.CWWW_L_WW] = 255;
        this.data[PATCH.CWWW_R_WW] = 255;
        this.data[PATCH.CWWW_L_STROBE] = 0;
        this.data[PATCH.CWWW_R_STROBE] = 0;
        this.data[PATCH.CWWW_L_COLOR_TEMP] = 0;
        this.data[PATCH.CWWW_R_COLOR_TEMP] = 0;
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageCW(): Promise<void> {
        this.data[PATCH.CWWW_L_DIMMER] = 255;
        this.data[PATCH.CWWW_R_DIMMER] = 255;
        this.data[PATCH.CWWW_L_CW] = 255;
        this.data[PATCH.CWWW_R_CW] = 255;
        this.data[PATCH.CWWW_L_STROBE] = 0;
        this.data[PATCH.CWWW_R_STROBE] = 0;
        this.data[PATCH.CWWW_L_COLOR_TEMP] = 0;
        this.data[PATCH.CWWW_R_COLOR_TEMP] = 0;
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageOff(): Promise<void> {
        Object.keys(PATCH).forEach((key) => {
            if (["PAR64", "W648", "CWWW"].includes(key)) {
                this.data[PATCH[key as keyof typeof PATCH]] = 0;
            }
        });
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageV1(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageV2(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    stageV3(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    spotLeft(): Promise<void> {
        this.data[PATCH.REVUELED_L_DIMMER] = 255;
        this.data[PATCH.REVUELED_R_DIMMER] = 255;
        this.data[PATCH.REVUELED_L_STROBE] = 0;
        this.data[PATCH.REVUELED_R_STROBE] = 0;
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    spotMid(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    spotOff(): Promise<void> {
        this.data[PATCH.REVUELED_L_DIMMER] = 0;
        this.data[PATCH.REVUELED_R_DIMMER] = 0;
        this.data[PATCH.REVUELED_L_STROBE] = 0;
        this.data[PATCH.REVUELED_R_STROBE] = 0;
        // todo: add MHx200
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    accentV1(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    accentV2(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }

    disco(): Promise<void> {
        // TODO
        return ArtNetService.getInstance().sendDataAsync(0, this.data);
    }
}

export default DMXController;
