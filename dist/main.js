function get_html_colors() {
  return App.Color.get_hex_colors();
} function save_colors(x) {
  App.Color.getUisave_colors(x);
} function reset_colors() {
  App.Color.reset_colors();
} function init_html() {
  const html = HtmlService.createHtmlOutputFromFile("index");
  SpreadsheetApp.getUi()
    .showModalDialog(html, "Palette de couleur")
}

function main() {
  App.main();
}

function onOpen() {
  const ui = SpreadsheetApp.getUi();
  
  ui.createMenu('Script de Frisk') // The name on the toolbar
    .addItem('Mettre à jour les données', 'main')
    .addItem('Palette de couleurs', 'init_html')
    .addItem('Reinitialiser les couleurs', 'reset_colors')
    .addToUi();
}