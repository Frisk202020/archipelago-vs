namespace App {
    function interpolate_color(from: RGB, to: RGB, pRatio: number): string {
        const out = new RGB(
            from.r + pRatio * (to.r - from.r),
            from.g + pRatio * (to.g - from.g),
            from.b + pRatio * (to.b - from.b)
        );

        return out.to_hex();
    };

    interface WinCount {
        wins: number,
        losses: number
    }

    export interface Display {
        data: string[][];
        colors: string[][];
    }
    export interface General {
        total: number, wins: number, rate: number
    }
    export class GlobalData {
        total_playthroughs_matrix: number[][];
        player_game_matrix: number[][];
        player_player_matrix: WinCount[][];
        games: Array<string>;
        players: Array<string>;

        constructor(games: Array<string>, players: Array<string>) {
            this.total_playthroughs_matrix = Array();
            this.player_game_matrix = Array();
            this.player_player_matrix = Array();
            for (let i = 0; i < players.length; i++) {
                this.player_game_matrix.push(Array(games.length).fill(-1));
                this.total_playthroughs_matrix.push(Array(games.length).fill(0));

                const arr = Array();
                for (let i = 0; i < players.length; i++) {
                    arr.push({wins: 0, losses: 0});
                }
                this.player_player_matrix.push(arr);
            }

            this.games = games; this.players = players;
        }

        static build_from_sessions(data: SessionResult): GlobalData {
            const global = new GlobalData(data.games_array, data.players_array);
            for (const s of data.sessions) {
                const g_id = data.game_ids.get(s.game)!;
                const w_id = data.player_ids.get(s.winner)!;
                const p_ids = s.loosers.map((x)=>data.player_ids.get(x)!);

                global.total_playthroughs_matrix[w_id][g_id]++;
                if (global.player_game_matrix[w_id][g_id] < 0) { global.player_game_matrix[w_id][g_id] = 1; }
                else { global.player_game_matrix[w_id][g_id]++; }

                for (const p_id of p_ids) {
                    if (global.player_game_matrix[p_id][g_id] < 0) { global.player_game_matrix[p_id][g_id] = 0; }
                    global.total_playthroughs_matrix[p_id][g_id]++;

                    global.player_player_matrix[w_id][p_id].wins++;
                    global.player_player_matrix[p_id][w_id].losses++;
                }
            }

            return global;
        }

        to_general_recap(): General[] {
            return this.player_player_matrix.map((row)=>{
                let wins = 0; let total = 0;
                for (const x of row) {
                    wins += x.wins;
                    total += x.wins + x.losses;
                }
                return {wins, total, rate: 100 * wins / total};
            });
        }

        get_game_display(): Display {
            const values = this.player_game_matrix.map(
                (row, i)=>row.map(
                    (x, j)=>x >= 0 ? 100 * x / this.total_playthroughs_matrix[i][j] : -1
                )
            );

            return {
                data: values.map((row)=>row.map((x)=>x === -1 ? "x" : `${x} %`)),
                colors: values.map((row)=>row.map(
                    (x)=>x === -1 ? COLORS.empty : interpolate_color(COLORS.lose, COLORS.win, x/100))
                )
            }
        }

        get_player_display(): Display {
            const data = Array<string[]>(); const colors = Array<string[]>();
            for (const row of this.player_player_matrix) {
                const d = Array<string>(); const c = Array<string>();
                for (const x  of row) {
                    if (x.wins === 0 && x.losses === 0) {
                        d.push("x"); c.push(COLORS.empty);
                    } else {
                        const tot = x.wins + x.losses;
                        d.push(`${x.wins} / ${tot}`);
                        c.push(interpolate_color(COLORS.lose, COLORS.win, x.wins / tot));
                    }
                }
                data.push(d); colors.push(c);
            }

            return {
                data, colors
            }
        }
    }
}