namespace App.Global {
    interface WinCount {
        wins: number,
        losses: number
    }

    export interface Display {
        data: string[][];
        colors: string[][];
    }
    export class GlobalData {
        total_playthroughs_matrix: number[][];
        general_array: WinCount[];
        team_array: WinCount[];
        player_game_matrix: number[][];
        player_player_matrix: WinCount[][];
        games: Array<string>;
        players: Array<string>;

        constructor(games: Array<string>, players: Array<string>) {
            this.total_playthroughs_matrix = Array();
            this.general_array = Array();
            this.team_array = Array();
            this.player_game_matrix = Array();
            this.player_player_matrix = Array();
            for (let i = 0; i < players.length; i++) {
                this.player_game_matrix.push(Array(games.length).fill(-1));
                this.total_playthroughs_matrix.push(Array(games.length).fill(0));
                this.general_array.push({wins: 0, losses: 0});
                this.team_array.push({wins: 0, losses: 0});

                const arr = Array();
                for (let i = 0; i < players.length; i++) {
                    arr.push({wins: 0, losses: 0});
                }
                this.player_player_matrix.push(arr);
            }

            this.games = games; this.players = players;
        }
        
        static build_from_sessions(data: Session.SessionResult): GlobalData {
            const global = new GlobalData(data.games_array, data.players_array);
            for (const s of data.sessions) {
                const g_id = data.game_ids.get(s.game)!;
                const w_id = data.player_ids.get(s.winner)!;
                const p_ids = s.loosers.map((x)=>data.player_ids.get(x)!);

                global.total_playthroughs_matrix[w_id][g_id]++;
                global.general_array[w_id].wins++;
                if (global.player_game_matrix[w_id][g_id] < 0) { global.player_game_matrix[w_id][g_id] = 1; }
                else { global.player_game_matrix[w_id][g_id]++; }

                for (const p_id of p_ids) {
                    if (global.player_game_matrix[p_id][g_id] < 0) { global.player_game_matrix[p_id][g_id] = 0; }
                    global.total_playthroughs_matrix[p_id][g_id]++;
                    global.general_array[p_id].losses++;

                    global.player_player_matrix[w_id][p_id].wins++;
                    global.player_player_matrix[p_id][w_id].losses++;
                }
            }

            for (const t_data of data.team_data) {
                for (const x of t_data.winners) {
                    global.team_array[data.player_ids.get(x)!].wins++;
                } for (const x of t_data.losers) {
                    global.team_array[data.player_ids.get(x)!].losses++;
                }
            }

            console.log(global.team_array);
            return global;
        }

        get_general_display(): string[][] {
            return this.general_array.map(display_win_count);
        }

        get_team_display(): string[][] {
            return this.team_array.map(display_win_count);
        }

        get_game_display(): Display {
            const colors = Color.get_colors();
            const values = this.player_game_matrix.map(
                (row, i)=>row.map(
                    (x, j)=>x >= 0 ? 100 * x / this.total_playthroughs_matrix[i][j] : -1
                )
            );

            return {
                data: values.map((row)=>row.map((x)=>x === -1 ? "x" : `${x} %`)),
                colors: values.map((row)=>row.map(
                    (x)=>x === -1 ? colors.empty : Color.interpolate(colors.defeat, colors.victory, x/100))
                )
            }
        }

        get_player_display(): Display {
            const stored_colors = Color.get_colors();
            const data = Array<string[]>(); const colors = Array<string[]>();
            for (const row of this.player_player_matrix) {
                const d = Array<string>(); const c = Array<string>();
                for (const x  of row) {
                    if (x.wins === 0 && x.losses === 0) {
                        d.push("x"); c.push(stored_colors.empty);
                    } else {
                        const tot = x.wins + x.losses;
                        d.push(`${x.wins} / ${tot}`);
                        c.push(Color.interpolate(stored_colors.defeat, stored_colors.victory, x.wins / tot));
                    }
                }
                data.push(d); colors.push(c);
            }

            return {
                data, colors
            }
        }
    }

    function display_win_count(x: WinCount) {
        const tot = x.wins + x.losses;
        return [tot, x.wins, 100 * x.wins / tot].map((x)=>x.toString())
    }
}