namespace App {
    const game_sheet_ids = new Map<string, number>(); let next_g = 0;
    const games_ordered_array = Array<string>();
    const player_sheet_ids = new Map<string, number>(); let next_p = 0;
    const players_ordered_array = Array<string>();

    interface SessionData {
        game: string,
        winner: string,
        loosers: string[]
    } interface SessionTeamData {
        winners: string[],
        losers: string[]
    } export interface SessionResult {
        sessions: SessionData[],
        game_ids: Map<string, number>,
        games_array: Array<string>,
        player_ids: Map<string, number>,
        players_array: Array<string>,
        team_data: SessionTeamData[]
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

    function parse_session_from_raw(raw: string[], teams_record: string[][]): SessionData {
        const game = raw[0];
        if (!game_sheet_ids.has(game)) {
            game_sheet_ids.set(game, next_g);
            games_ordered_array.push(game);
            next_g++;
        } 

        const players = Array<string>();
        const times = Array<string>();
        let team_id = teams_record.length === 0 ? -1 :  0;
        for (let i = 1; i < raw.length; i++) {
            if (raw[i] === "")  { break; }

            if (!player_sheet_ids.has(raw[i])) {
                player_sheet_ids.set(raw[i], next_p);
                players_ordered_array.push(raw[i]);
                next_p++;
            }
            players.push(raw[i]);

            if (team_id < 0) {
                teams_record.push([raw[i]]);
            } else {
                teams_record[team_id].push(raw[i]);
                team_id++;
            }

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
        const sessions = Array<SessionData>();
        const team_data = Array<SessionTeamData>();

        let current_teams = Array(); let session_id = 1;
        for (let i = 0; i < data.length; i++) {
            if (data[i][0] === "Jeux") {
                i++;
                while (data[i] !== undefined && data[i][0] !== "") {
                    sessions.push(parse_session_from_raw(data[i], current_teams));
                    i++
                }

                let winners: string[] | null = null;
                let losers: string[] | null = null;
                let team_id = 0;
                for (let j = 1; j < data[i].length && data[i][j].length > 0; j += 2) {
                    switch (data[i][j].toLowerCase()) {
                        case "victoire":
                            if (winners) {
                                throw new Error("Ce script ne prend pas en charge une session avec plus d'une équipe gagnante");
                            }
                            winners = current_teams[team_id];
                            break;
                        case "defaite":
                            if (losers) {
                                losers = losers.concat(current_teams[team_id]);
                            } else {
                                losers = current_teams[team_id];
                            }
                            break;
                        default: 
                            throw new Error(`Résultat inconnu: ${data[i][j]}. Ceux pris en charge sonr "VICTOIRE" ou "DEFAITE"`);
                    }
                    team_id++;
                }

                if (!winners) {
                    throw new Error(`Erreur: la session ${session_id} n'a pas d'équipe gagnante.`);
                } if (!losers) {
                    throw new Error(`Erreur: la session ${session_id} n'a pas d'équipe perdante`);
                }

                team_data.push({winners, losers});
                current_teams = Array();
                session_id++;
            }
        }

        return {
            sessions,
            team_data,
            game_ids: game_sheet_ids,
            player_ids: player_sheet_ids,
            players_array: players_ordered_array,
            games_array: games_ordered_array
        }
    }
}