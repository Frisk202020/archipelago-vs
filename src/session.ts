namespace App {
    const game_sheet_ids = new Map<string, number>(); let next_g = 0;
    const games_ordered_array = Array<string>();
    const player_sheet_ids = new Map<string, number>(); let next_p = 0;
    const players_ordered_array = Array<string>();

    interface SessionData {
        game: string,
        winner: string,
        loosers: string[]
    } export interface SessionResult {
        sessions: SessionData[],
        game_ids: Map<string, number>,
        games_array: Array<string>,
        player_ids: Map<string, number>,
        players_array: Array<string>
    }

    function with_fallback(n: number | undefined) {
        return n === undefined ? Number.MAX_VALUE : n;
    } 

    enum Compare {
        Greater, Equal, Lesser
    }
    function compare_number(x: number, y: number): Compare {
        if (x === y) { return Compare.Equal; }
        else if (x > y) { return Compare.Greater; }
        return Compare.Lesser;
    }
    class Time {
        hour: number; min: number; sec: number;
        constructor(str: string) {
            const split = str.split(":").map((x)=>{
                const n = Number.parseInt(x);
                if (Number.isNaN(n)) { return Number.MAX_VALUE; }
                return n;
            });
            this.hour = with_fallback(split[0]);
            this.min = with_fallback(split[1]);
            this.sec = with_fallback(split[2]);
        }

        static get_best_time(x: string[]): number {
            const times = x.map((x)=>new Time(x));
            let best = 0;
            for (let i = 1; i < times.length; i++) {
                if (times[best]._compare(times[i]) === Compare.Greater) {
                    best = i;
                }
            }

            return best;
        }

        _compare(x: Time): Compare {
            if (this.hour != x.hour) { return compare_number(this.hour, x.hour); }
            else if (this.min != x.min) { return compare_number(this.min, x.min); }
            return compare_number(this.sec, x.sec);
        }
    }

    function parse_session_from_raw(raw: string[]): SessionData {
        const game = raw[0];
        if (!game_sheet_ids.has(game)) {
            game_sheet_ids.set(game, next_g);
            games_ordered_array.push(game);
            next_g++;
        } 

        const players = Array<string>();
        const times = Array<string>();
        for (let i = 1; i < raw.length; i++) {
            if (raw[i] === "")  { break; }

            if (!player_sheet_ids.has(raw[i])) {
                player_sheet_ids.set(raw[i], next_p);
                players_ordered_array.push(raw[i]);
                next_p++;
            }
            players.push(raw[i]);
            i++;
            times.push(raw[i]);
        }

        const win_id = Time.get_best_time(times);
        const winner = players[win_id];
        players.splice(win_id, 1);
        return {
            game, winner, loosers: players
        }
    }

    export function parse_sheet(sheet: GoogleAppsScript.Spreadsheet.Sheet): SessionResult {
        const data = sheet.getDataRange().getDisplayValues();
        const raw = Array<string[]>();
        for (let i = 0; i < data.length; i++) {
            if (data[i][0] === "Jeux") {
                i++;
                while (data[i] !== undefined && data[i][0] !== "") {
                    raw.push(data[i]);
                    i++
                }
            }
        }

        return {
            sessions: raw.map((x)=>parse_session_from_raw(x)),
            game_ids: game_sheet_ids,
            player_ids: player_sheet_ids,
            players_array: players_ordered_array,
            games_array: games_ordered_array
        }
    }
}