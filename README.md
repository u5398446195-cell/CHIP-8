# CHIP-8 en Rust
<img width="476" height="258" alt="SCR-20261121-tzod" src="https://github.com/user-attachments/assets/1f669e10-f4f9-41b7-9258-cb21413f734e" />
<img width="499" height="242" alt="SCR-20261121-tzsi" src="https://github.com/user-attachments/assets/bba4f6f3-c178-4b8a-895d-774d410e825c" />


Un émulateur **CHIP-8** écrit en Rust, pour apprendre comment fonctionne un émulateur de l'intérieur.

Le CHIP-8 est une machine virtuelle des années 70 : 4 Ko de mémoire, 16 registres, un écran noir et blanc de 64 × 32 pixels et 35 instructions. C'est le premier pas classique avant d'émuler une vraie console.

## État actuel

✅ Les **35 instructions** du CHIP-8 sont gérées, avec le clavier et les timers (60 Hz).
✅ Le jeu **Dino Run** tourne.

| Famille | Instructions |
|---|---|
| Écran | `00E0` effacer, `DXYN` dessiner un sprite (XOR + collision dans `VF`) |
| Sauts et fonctions | `1NNN`, `BNNN`, `2NNN` / `00EE` (pile) |
| Sauts conditionnels | `3XNN`, `4XNN`, `5XY0`, `9XY0` |
| Registres | `6XNN`, `7XNN`, `ANNN`, `CXNN` (hasard) |
| Calculs | `8XY0` à `8XY7`, `8XYE` |
| Clavier | `EX9E`, `EXA1`, `FX0A` |
| Timers et mémoire | `FX07`, `FX15`, `FX18`, `FX1E`, `FX29`, `FX33`, `FX55`, `FX65` |

## Lancer le projet

Il faut [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/u5398446195-cell/CHIP-8.git
cd CHIP-8
cargo run
```

Le jeu lancé est `roms/dinorun.ch8` (chemin dans `src/main.rs`). `Échap` ferme la fenêtre.

## ROMs

| Fichier | Contenu |
|---|---|
| `roms/dinorun.ch8` | **Dino Run** — Andrezinrc · jeu de course sans fin : saute par-dessus les cactus |
| `roms/ibm.ch8` | IBM Logo, la ROM de test classique |

Dino Run vient du [CHIP-8 Archive](https://github.com/JohnEarnest/chip8Archive), sous licence CC0 (domaine public).

### Contrôles

| Touche | Action |
|---|---|
| `Espace` | Sauter (Dino Run) |
| `Échap` | Quitter |

Le pavé 16 touches du CHIP-8 est aussi placé à gauche du clavier, pour les autres jeux :

```
CHIP-8          Clavier (AZERTY)
1 2 3 C         1 2 3 4
4 5 6 D   →     A Z E R
7 8 9 E         Q S D F
A 0 B F         W X C V
```

## Comment ça marche

À chaque image (60 par seconde), l'émulateur exécute quelques instructions puis affiche l'écran :

1. **Fetch** : lire 2 octets en mémoire à l'adresse `PC` pour former l'instruction.
2. **Decode** : la découper en morceaux (`X`, `Y`, `N`, `NN`, `NNN`) avec des masques (`&`) et des décalages (`>>`).
3. **Execute** : faire ce qu'elle demande, via un `match` sur le premier chiffre.

```
src/
├── main.rs     → crée la machine, charge la ROM, ouvre la fenêtre
├── chip.rs     → la machine : mémoire, registres, police, fetch / execute
└── window.rs   → la fenêtre (minifb) agrandie × 8, le clavier et les timers
```

## À faire

- [ ] Son : un bip tant que `sound_timer` > 0
- [ ] Passer la [suite de tests de Timendus](https://github.com/Timendus/chip8-test-suite)
- [ ] Choisir la ROM au lancement (`cargo run -- roms/jeu.ch8`)

## Technologies

- **Rust** (édition 2024)
- **[minifb](https://crates.io/crates/minifb)** pour la fenêtre et l'affichage

## Ressources

- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) — Tobias V. Langhoff
- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [chip8-test-suite](https://github.com/Timendus/chip8-test-suite) — ROMs de test
