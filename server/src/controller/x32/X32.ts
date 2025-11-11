import OSCx32Service from "../../services/OSCx32Service.js";

class X32 {
    public static loadScene(scene: number) {
        OSCx32Service.getInstance().sendOSC("/-action/goscene", scene);
    }
}

export default X32;
