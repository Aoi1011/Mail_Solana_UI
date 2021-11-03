import { deserialize, serialize } from "borsh";

class Assignable {
    constructor(properties:  any) {
        Object.keys(properties).forEach((key) => {
            this[key] = properties[key];
        });
    }

    encode() {
        return serialize(SCHEMA, this);
    }
}
