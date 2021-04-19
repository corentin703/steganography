# Stéganographie [WIP]

Ce projet vise à proposer un outil permettant de cacher des fichiers dans une image.
Pour ce faire, une image au format bmp est nécessaire.

## Principe
Les octets du fichier à cacher seront décomposé et écraseront les LSB (bits de poids faible) 
des pixels, engendrant une légère modification des couleurs de l'image originale 
invisible à l'œil humain.

## Utilisation du programme
Un programme pour cacher des fichiers dans une image

USAGE:
steganography --input <input-file-path> --output <output-file-path> <SUBCOMMAND>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-i, --input <input-file-path> Fichier d'entrée
-o, --output <output-file-path>    Fichier de sortie

SUBCOMMANDS:
decode    Mode décodage
encode    Mode encodage
help      Prints this message or the help of the given subcommand(s)