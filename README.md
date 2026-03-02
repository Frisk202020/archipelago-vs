# Archipelago VS code

This repo contains the source code. All the TS code is compiled and bundled in one JS file, `code.js`, then all the JS code is sent to Google using `clasp`. The workflow is as follows :
- Modify the TS source 
- Push the modification to Google using `npm run push`

Notice that to compile the code using `tsc`, you need to have App Script types installed :

```bash
npm install --save-dev typescript @types/google-apps-script
```