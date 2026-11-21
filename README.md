# CHIP-8 en Rust

Un émulateur **CHIP-8** écrit en Rust, pour apprendre comment fonctionne un émulateur de l'intérieur.

Le CHIP-8 est une machine virtuelle des années 70 : 4 Ko de mémoire, 16 registres, un écran noir et blanc de 64 × 32 pixels et 35 instructions. C'est le premier pas classique avant d'émuler une vraie console.

## État actuel

✅ La ROM **IBM Logo** s'affiche.

Instructions déjà gérées :

| Opcode | Rôle |
|---|---|
| `00E0` | Effacer l'écran |
| `1NNN` | Sauter à l'adresse `NNN` |
| `6XNN` | `VX = NN` |
| `7XNN` | `VX += NN` |
| `ANNN` | `I = NNN` |
| `DXYN` | Dessiner un sprite de `N` lignes en (`VX`, `VY`), avec détection de collision dans `VF` |

## Lancer le projet

Il faut [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/u5398446195-cell/CHIP-8.git
cd CHIP-8
cargo run
```

La ROM chargée est `roms/ibm.ch8` (chemin dans `src/main.rs`). `Échap` ferme la fenêtre.

## Comment ça marche

À chaque image (60 par seconde), l'émulateur exécute quelques instructions puis affiche l'écran :

1. **Fetch** : lire 2 octets en mémoire à l'adresse `PC` pour former l'instruction.
2. **Decode** : la découper en morceaux (`X`, `Y`, `N`, `NN`, `NNN`) avec des masques (`&`) et des décalages (`>>`).
3. **Execute** : faire ce qu'elle demande, via un `match` sur le premier chiffre.

```
src/
├── main.rs     → crée la machine, charge la ROM, ouvre la fenêtre
├── chip.rs     → la machine : mémoire, registres, police, fetch / execute
└── window.rs   → la fenêtre (minifb), agrandie × 8
```

## À faire

- [ ] Fonctions : `2NNN`, `00EE` (pile)
- [ ] Sauts conditionnels : `3XNN`, `4XNN`, `5XY0`, `9XY0`
- [ ] Calculs : famille `8XY_`
- [ ] Hasard : `CXNN`
- [ ] Famille `FX__` (timers, police, décimal, sauvegarde des registres)
- [ ] Clavier : `EX9E`, `EXA1`, `FX0A`
- [ ] Timers à 60 Hz et son
- [ ] Passer la [suite de tests de Timendus](https://github.com/Timendus/chip8-test-suite)
- [ ] Faire tourner un vrai jeu (Pong, Brix, Tetris…)

## Technologies

- **Rust** (édition 2024)
- **[minifb](https://crates.io/crates/minifb)** pour la fenêtre et l'affichage

## Ressources

- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) — Tobias V. Langhoff
- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [chip8-test-suite](https://github.com/Timendus/chip8-test-suite) — ROMs de test
