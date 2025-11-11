/**
 * parsed recieved data from FakeObjectNotation (string) to JSON
 * @param data the recieved data
 * @returns the parsed data as an JSON object
 * @author filip326
 */
export function parseFONToJSON(data: string): { [key: string]: unknown } {
    // check if it starts with ( and ends with )
    if (!data.startsWith("(") || !data.endsWith(")")) {
        throw new Error("Invalid FakeON string");
    }
    // remove ( ) at the beginning and end of the string
    data = data.replace(/^\(/, "");
    data = data.replace(/\)$/, "");
    // escape all " in data
    data = data.replace(/"/g, '\\"');
    // replace all ' with ", except if it is preceded by \
    data = data.replace(/(?<!\\)'/g, '"');
    const obj = JSON.parse(data) as { [key: string]: unknown };
    return obj;
}

/**
 * stringifies a JSON object to a FakeObjectNotation string
 * @param obj a JSON object
 * @returns the FakeObjectNotation string
 * @author filip326
 */
export function stringifyJSONToFON(obj: any) {
    return `(${JSON.stringify(obj).replace(/"/g, "'")})`;
}
