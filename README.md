# SwarmDrones

Dépôt principal de la **solution A** : un essaim de drones autonomes qui doit
maintenir une décision d'état partagée sans coordinateur central.

Ce dépôt ne contient **pas** le code des algorithmes. Il les **agrège** : chaque
algorithme vit dans son propre dépôt GitHub, rattaché ici en tant que
[Git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules).

---

## Pourquoi cette organisation

Un algorithme d'essaim est un objet autonome : il a son propre cycle de vie, ses
propres tests, sa propre parité de référence, ses propres versions. Le mélanger
au code d'orchestration produit un dépôt qui ne peut plus évoluer sans casser
autre chose.

L'architecture retenue sépare les deux responsabilités :

- **`SwarmDrones`** — l'orchestrateur. Il déclare *quels* algorithmes composent
  la solution, à *quelle version* ils sont épinglés, et comment ils s'articulent.
- **`alg-<nom>`** — un dépôt par algorithme. Il contient le code, les tests, la
  documentation de référence et l'historique de ses propres versions.

Le submodule est le lien : `SwarmDrones` enregistre un **commit précis** de
chaque algorithme, pas une branche flottante. La solution est donc
reproductible — cloner `SwarmDrones` à une date donnée redonne exactement les
mêmes versions d'algorithmes.

---

## Algorithmes

| Algorithme | Dépôt | Chemin | Version épinglée |
|---|---|---|---|
| `ALG_CONSENSUS` | [dagornc/alg-consensus](https://github.com/dagornc/alg-consensus) | `algorithms/consensus` | voir `git submodule status` |

### ALG_CONSENSUS

Consensus d'état d'essaim par **CRDT semi-treillis** et **gossip** sur une
topologie de grille 6×5. Chaque agent détient un état partiel ; la fusion est
commutative, associative et idempotente, ce qui garantit la convergence sans
coordinateur.

- Implémentation **Rust**, à **parité bit-à-bit** avec une référence Python
  embarquée (MT19937 compatible CPython).
- 9 tests de harnais (T1…T9), 26 tests automatisés, 0 warning.
- Documentation de référence complète : 20 chapitres couvrant le
  fonctionnement, les prérequis, l'installation, la configuration, les
  paramètres d'entrée et de sortie, les mesures et les limites connues.

**Versions disponibles :**

- **v1** (branche `master`) — comportement historique. Sur T8, 990/1000 graines
  convergent ; les 10 échecs sont des agents isolés par retrait (propriété de
  la topologie, documentée en §15.1).
- **v2** (branche `v2`) — mode additionnel activé par `--v2`. Reconnexion des
  agents isolés : T8 passe à **1000/1000** (+10 graines, 0 régression) et le
  trafic baisse de **32 %** sur ce test. Parité v1 préservée bit-à-bit.
- **v3** (branche `v3`) — mode additionnel activé par `--v3`. Quiescence : un
  agent cesse d'émettre quand son état est stable, avec réveil périodique.
  **−66 % de messages** sur l'ensemble des tests, accord **1000/1000** partout.
  Parité v1 préservée bit-à-bit.

**Notation /20** (grille : correction 6, coût 5, latence 3, robustesse 3,
ingénierie 3) : **v1 15,7** · **v2 16,3** · **v3 19,5**.

→ [Lire la documentation](https://github.com/dagornc/alg-consensus/blob/master/README.md)
→ [Section 20 — la v2](https://github.com/dagornc/alg-consensus/blob/v2/README.md#20-version-2--reconnexion-des-agents-isolés)
→ [Section 21 — la v3](https://github.com/dagornc/alg-consensus/blob/v3/README.md#21-version-3--quiescence-loop-engineering)

---

## Cloner la solution complète

Le code des algorithmes n'est **pas** inclus dans un `git clone` ordinaire. Il
faut demander explicitement les submodules :

```bash
git clone --recurse-submodules https://github.com/dagornc/SwarmDrones.git
```

Si le dépôt est déjà cloné sans les submodules :

```bash
git submodule update --init --recursive
```

Pour récupérer les évolutions des algorithmes après un `git pull` :

```bash
git submodule update --remote --merge
```

---

## Ajouter un algorithme

1. Créer le dépôt dédié sur GitHub, nommé `alg-<nom>`.
2. Y publier le code, les tests et un `README.md` au standard de documentation
   (voir la section « Standard de documentation » ci-dessous).
3. Rattacher le dépôt ici :

```bash
git submodule add https://github.com/dagornc/alg-<nom>.git algorithms/<nom>
git commit -m "algorithms/<nom> : rattachement du submodule"
```

4. Mettre à jour le tableau « Algorithmes » de ce README.

---

## Mettre à jour un algorithme

Le submodule épingle un commit. Pour avancer l'épingle après une évolution de
l'algorithme :

```bash
cd algorithms/<nom>
git pull origin master
cd ../..
git add algorithms/<nom>
git commit -m "algorithms/<nom> : mise a jour de l'epingle"
```

Le dépôt principal enregistre alors le nouveau commit. C'est cette étape qui
rend l'évolution visible pour la solution.

---

## Standard de documentation

Tout algorithme rattaché à ce dépôt doit fournir un `README.md` couvrant au
minimum :

1. En bref — tableau de synthèse
2. Le problème résolu
3. Fonctionnement de l'algorithme
4. Prérequis
5. Installation
6. Configuration
7. Paramètres d'entrée
8. Paramètres de sortie
9. Utilisation en ligne de commande
10. Utilisation comme bibliothèque
11. Le harnais de tests
12. Vérification et parité
13. Performances mesurées
14. Résultats de référence
15. Limites connues
16. Structure du dépôt
17. Dépannage
18. Glossaire
19. Licence et références
20. Versions et évolutions (v2, v3…)

Le modèle de référence est le
[README de `alg-consensus`](https://github.com/dagornc/alg-consensus/blob/master/README.md).

---

## Licence

MIT — Copyright (c) 2026 Christophe Dagorn.
