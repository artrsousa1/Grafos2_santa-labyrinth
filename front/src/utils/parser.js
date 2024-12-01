const santa = { "u": true, "l": true, "d": true, "r": true, "can_rotate": false };
const socks = { "u": true, "l": true, "d": true, "r": true, "can_rotate": false };
const corner_dl = { "u": false, "l": true, "d": true, "r": false, "can_rotate": true };
const corner_ul = { "u": true, "l": true, "d": false, "r": false, "can_rotate": true };
const corner_ur = { "u": true, "l": false, "d": false, "r": true, "can_rotate": true };
const corner_dr = { "u": false, "l": false, "d": true, "r": true, "can_rotate": true };
const line_hor = { "u": false, "l": true, "d": false, "r": true, "can_rotate": true };
const line_vert = { "u": true, "l": false, "d": true, "r": false, "can_rotate": true };

const dict = {
    "80": santa,
    "90": socks,
    "10": corner_dl,
    "11": corner_ul,
    "12": corner_ur,
    "13": corner_dr,
    "00": line_hor,
    "01": line_vert,
    "02": line_hor,
    "03": line_vert
}

export default function parseMap(currentState, map) {
    let parsedArray = Array(7).fill().map(() => Array(7).fill(null));
    for (let i = 0; i < 7; i++) {
        for (let j = 0; j < 7; j++) {
            const key = String(map[i][j]) + String(currentState[i][j]);
            parsedArray[i][j] = dict[key];
        }
    }

    return parsedArray;
}



