# ArchiBot

`ArchiBot` est un bot discord conçu pour faciliter l'organisation de sessions de versus sur [Archipelago](https://archipelago.gg/), durant lesquelles plusieurs équipes s'affrontent pour finir leurs jeux le plus rapidement possible.

Après ajout sur un serveur Discord, `ArchiBot` propose les fonctionnalités suivantes :
- Organisation de sessions et envoi d'un récapitulatif post-session sur Google Sheet
- Sélection d'un jeu aléatoire pour un utilisateur donné
- Création automatique de sondage de participation

## Documentation utilisateur

### Introduction

Les commandes d'`Archibot` sont disposés en structure arborescente, dont voici le détail :


├── `picker` : famille relative au choix aléatoire de jeux \
│ ├── **`pick_random_game`** : **choisir un jeu aléatoire dans la liste de l'utilisateur** \
│ ├── **`add_game`** : **ajouter un jeu à la liste** \
│ ├── **`remove_game`** : **retirer un jeu de la liste** \
│ └── **`list_games`** : **afficher les jeux enregistrés** \
├── `session` : famille relative à la gestion de session \
│ ├── `builder` : famille relative à la création d'une session \
│ │ ├── **`init`** : **initialisation d'une session avec des équipes de $N$ joueurs** \
│ │ ├── **`add_game`** : **ajouter un jeu à la session** \
│ │ ├── **`remove_last_game`** : **retirer le dernier jeu ajouté** \
│ │ ├── **`add_player`** : **ajouter un joueur à la session** \
│ │ ├── **`remove_last_player`** : **retirer le dernier joueur ajouté** \
│ │ └── **`start`** : **commencer la session en construction** \
│ ├── `timestamps` : famille de commandes relative à la gestion d'étiquettes \
│ │ ├── **`add`** : **ajouter une nouvelle étiquette (temps intermédiaire)** \
│ │ └── **`list`** : **afficher toutes les étiquettes** \
│ ├── **`finish`** : **marquer la fin de partie d'un joueur** \
│ ├── **`close`** : **clôturer la session en cours en envoyant les données sur Google Sheets** \
│ └── **`get_time`** : **afficher le temps de partie d'un joueur** \
└── **`poll`** : **création de sondage de participation** \

Ainsi pour appeler une commande, utilisez le chemin complet séparé d'espaces, par exemple :

```bash
# Appelle la commande 'pick_random_game'
/picker pick_random_game

# Appelle la commande de sondage
/poll

# Appelle la commende de création de session
/session builder start
```

***Remarque** : seules les feuilles de l'arbre sont des commandes dans Discord. Par ailleurs, toutes les commandes sont des commandes slash.*

La documentation qui suit présente le détail de chaque commande : leur fonction et leurs paramètres.

### Commandes `picker`

Cette famille de commandes permet à un utilisateur de fournir à `ArchiBot` une liste de jeux auxquels il souhaiterait jouer, puis d'en choisir un aléatoirement. Les listes sont propres à chaque joueur, et seul l'utilisateur concerné est en droit de modifier cette liste (la lecture est quand à elle publique sur le serveur).

#### Commande `add_game`

Cette commande est utilisée pour ajouter un jeu à la liste. **Elle s'applique à l'utilisateur client.**

**Paramètres**
- *game* : nom du jeu à ajouter

**Exemple**
```bash
/picker add_game Celeste # Liste : [Celeste]
```

**Notes additionnelles**

Pour le moment la commande n'accepte pas des doublons de jeux pour des raisons d'ergonomie, mais cela pourrait être ajouté pour permettre de casser l'uniformité des probabilités.

#### Commande `remove_game`

Cette commande est utilisée pour retirer un jeu ajouté à la liste. **Elle s'applique à l'utilisateur client.**

**Paramètres**
- *game* : nom du jeu à retirer

**Exemple**
```bash
/picker add_game Celeste # liste : [Celeste]
/picker remove_game Tunic # Aucune action : le jeu n'est pas ajouté
/picker remove_game Celeste # Liste : []
```

#### Commande `list_games`

Cette commande est utilisée pour afficher tous les jeux ajoutés par un utilisateur.

**Paramètres**
- *user* (Utilisateur Discord, optionnel) : utilisateur cible (par défaut, le client)

**Exemple**
```bash
# Utilisez l'autocomplétion de Discord renseigner le paramètre d'utilisateur
/picker list_games @Frisk # Affiche les jeux de Frisk
/picker list_games # Affiche les jeux du client`
```

### Commandes `session` -- Généralités

Ces commandes sont destinées à organiser est faciliter le déroulement de sessions de Versus. Créer une session avec `ArchiBot` offre les fonctionnalités suivantes :

- **Mesure automatique du temps de partie** : `ArchiBot` commence un chronomètre lors du démarrage d'une session, il vous est ensuite possible de  déclarer à tout moment des temps intermédiaires (étiquettes) ou la fin de partie des joueurs de cette session.
- **Envoi des données sur Google Sheets** : `ArchiBot` peut envoyer le récapitulatif de la session sur Google Sheets grâce à une seule commande Discord !
- **Préparation du serveur** : `ArchiBot` peut organiser le serveur pour assurer le bon déroulement d'une session : création de rôles d'équipe et création de canaux dédiés à chaque équipe, permettant l'execution de commandes `ArchiBot` en toute confidentialité. `ArchiBot` peut ensuite nettoyer ces composants temporaires lorsque la session est terminée.

Le paquet `builder` rassemble les commandes dédiées à la création d'une session.

### Commandes `session` -- création de session avec le paquet `builder`

Pour configurer une nouvelle session, il est recommandé de suivre la procédure décrite ci-dessous.

1) Il est nécessaire d'initier une nouvelle session avec `init`. Vous précisez ici un nombre $N$ de joueurs, qui représentent la taille des équipes : le nombre d'équipes n'est pas limité, mais les équipes doivent nécessairement être uniformes. 
2) Ajouter les jeux de cette session avec `add_game`. Il est nécessaire d'ajouter $N$ jeux. A noter que ces jeux n'ont aucune fonction dans Discord, mais seront utiles lors de l'envoi vers Google Sheets.
3) Ajouter les joueurs avec `add_player`, en construisant les équipes une par une. C'est-à-dire que les $N$ premiers ajoutés seront dans l'équipe 1, les $N$ suivants dans l'équipe 2 et ainsi de suite.
4) Tout est prêt ! Démarrez une session avec `start`.

#### Commande `init`

Cette commande permet d'initier une nouvelle configuration, écrasant les données d'une éventuelle précédente (après confirmation utilisateur).

**Paramètres**
- *team_size* (nombre) : taille de chaque équipe de cette session

**Exemple**
```bash
/session builder 2
```

#### Commande `add_game`

Cette commande permet d'ajouter un jeu à la session en construction. Pour une session à équipes de taille $N$, il faut appeler $N$ fois cette commande.

**Paramètres**
- *game* : nom du jeu à ajouter

**Exemple**
```bash
/session builder init 2
/session builder add_game Celeste
/session builder add_game Tunic # Configuration des jeux terminée
/session builder add_game Fez # Commande refusée
```

#### Commande `remove_last_game`

Cette commande permet de retirer le dernier jeu ajouté.

**Exemple**
```bash
/session builder add_game Celeste
/session builder add_game Tunic
/session builder remove_last_game # Removes Tunic
```

#### Commande `add_player`

Cette commande ajoute un joueur à la session en cours de construction, à noter que **les équipes se construisent une par une**. De plus, il est important de constituer une équipe **dans l'ordre dans lequel les jeux ont été ajoutés**, c'est-à-dire que le premier joueur de chaque équipe est associé au premier jeu de la liste et ainsi de suite.

**Paramètres**
- *player* (Utilisateur Discord) : joueur à ajouter

**Exemple**
```bash
# Partant d'une configuration d'équipes de taille 2 avec les jeux [Celeste, Tunic]

/session builder add_player @Madeline # Madeline dans l'équipe 1 sur Celeste
/session builder add_player @Ruin_Seeker # Ruin_Seeker dans l'équipe 1 sur Tunic
/session builder add_player @Théo # Théo dans l'équipe 2 sur Celeste
...
```

#### Commande `remove_last_player`

Cette commande retire le dernier joueur ajouté à la configuration en cours.

**Exemple**
```bash
/session builder add_player @Madeline 
/session builder add_player @Ruin_Seeker 
/session builder remove_last_player # removes @Ruin_Seeker
```

#### Commande `start`

Commencer une nouvelle session, c'est-à-dire que le chrono démarre au moment de son execution.

- Si une configuration est en cours de construction, `ArchiBot` procède à la construction de l'environnement de jeu sur le serveur (canaux et rôles d'équipe) avant de commencer la session.
- Si aucune configuration n'est en cours, `ArchiBot` autorise tout de même l'execution de la commande. Il sera possible d'utiliser toutes les commandes de session, mais `ArchiBot` n'enverra pas de données sur Google Sheets à la fin de la partie. A noter que cette session écrasera tout de même les données d'une précédente session.

**Exemple**
```bash
/session builder start
```

### Commandes `session` -- gestion d'étiquettes avec le paquet `timestamps`

Les *étiquettes*, ou *temps intermédiaires*, permettent de mettre en lumière des moments clés d'une partie. Cela offre une comparaison plus riche entre les parties de plusieurs adversaires sur un même jeu, par exemple.

#### Commande `add`

Cette commande ajoute un temps intermédiaire, en attachant une étiquette au temps de session actuel.

**Paramètres**
- *label* : description (courte) à attacher à cette étiquette.
- *player* (Utilisateur Discord, optionnel) : joueur affecté par la commande, par défaut le client.  

**Exemple**
```bash
/session timestamps add 'Forsaken City A -- Golden Strawberry' # Appliquée au client
/session timestamps add 'Farewell -- Golden Strawberry' @Frisk # Appliquée à @Frisk 
```

#### Commande `list`

Cette commande liste les temps ajoutés.

**Paramètres**
- *player* (Utilisateur Discord, optionnel) : joueur affecté par la commande, par défaut le client

**Exemple**
```bash
/session timestamps list # Appliquée au client
/session timestamps list @Frisk # Appliquée à @Frisk
```

### Commandes `session` -- divers

Il reste une partie des commandes de sessions à explorer, toutes à utiliser en cours de session. 

#### Commande `get_time`

Cette commande affiche le temps de jeu actuel (depuis le début de la session).

**Exemple**
```bash
/session get_time
```

#### Commande `finish`

Cette commande marque la fin de partie d'un joueur.

**Paramètres**
- *player* (Utilisateur Discord, optionnel) : joueur affecté par la commande.

**Exemple**
/session finish # Appliquée au client
/session finish @Frisk # Appliquée à @Frisk

#### Commande `close`

Cette commande cloture la session en cours. Envoi ensuite les données de la session sur Google Sheets. Elle n'a aucun effet notable si la session n'a pas d'équipes.    

**Exemple**
```bash
/session close
```
## Commande `poll`

Cette commande permet de lancer un sondage de participation sur le week-end prochain.

**Example**
```bash
/poll
```

## Permissions requises

Afin de profiter de tous les fonctionnalités disponibles, `ArchiBot` doit recevoir les autorisations suivantes :
- **Send Messages** pour les commandes `picker`
- **Send Messages, Create Polls** pour la commande `poll`
- **Send Messages**, **Manage Roles**, **Manage Channels** pour les commandes `session`

A noter que le bot doit recevoir, quelque soit sa fonction, l'autorisation **Bot**.

Actuellement il n'est pas possible d'ajouter `ArchiBot` sans toutes ses fonctionnalités, donc toutes les permissions ci-dessus devront être accordées.