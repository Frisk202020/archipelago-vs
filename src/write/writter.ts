namespace App.Write {
    const players_writter = {
        id: 1081434893,
        fn: (sheet: Sheet, data: Global.GlobalData)=>{
            const n = data.players.length;
            Helper.get_first_row(sheet, n).setValues([data.players]);
            Helper.get_first_column(sheet, n).setValues(Helper.players(data));

            const display = data.get_player_display();
            sheet.getRange(2, 2, n, n)
            .setValues(display.data)
            .setBackgrounds(display.colors);
        }
    };
    const games_writter = {
        id: 1484584255,
        fn: (sheet: Sheet, data: Global.GlobalData)=>{
            Helper.get_first_row(sheet, data.games.length).setValues([data.games]);
            Helper.get_first_column(sheet, data.players.length).setValues(Helper.players(data));

            const display = data.get_game_display();
            sheet.getRange(2, 2, data.players.length, data.games.length)
            .setValues(display.data)
            .setBackgrounds(display.colors);
        }
    };
    
    function summary_fn(sheet: Sheet, data: Global.GlobalData, forGeneral=true) {
        const colors = Color.get_colors();

        Helper.get_first_row(sheet, 3).setValues([["Nombre de VS", "Victoires", "Score (%)"]]);
        Helper.get_first_column(sheet, data.players.length).setValues(Helper.players(data));
        sheet.getRange(2,2,data.players.length,3).setValues(forGeneral ? data.get_general_display() : data.get_team_display());
        sheet.getRange(2,2,data.players.length,2).setBackgrounds(Array(data.players.length).fill(Array(2).fill(colors.empty)));

        const rule = SpreadsheetApp.newConditionalFormatRule()
        .setGradientMaxpointWithValue(
            colors.victory.to_hex(),
            SpreadsheetApp.InterpolationType.NUMBER,
            '100',
        )
        .setGradientMinpointWithValue(
            colors.defeat.to_hex(),
            SpreadsheetApp.InterpolationType.NUMBER,
            '0',
        )
        .setRanges([sheet.getRange(2,4,data.players.length,1)])
        .build();
        sheet.setConditionalFormatRules([rule]);
    }
    const team_writter = {
        id: 1879123301,
        fn: (s: Sheet, d: Global.GlobalData)=>summary_fn(s,d,false) 
    }
    const general_writter = {
        id: 0,
        fn: summary_fn
    }
    const writters = [
        general_writter, team_writter, games_writter, players_writter
    ];

    export function write_sheets(ss: Spread, data: Global.GlobalData) {
        for (const w of writters) {
            Helper.write_sheet(ss, data, w);
        }
    }
}