# SwarmDrones

Dépôt principal de la **solution A** : un essaim de drones autonomes qui doit
maintenir une décision d'état partagée sans coordinateur central.

Ce dépôt ne contient **pas** le code des algorithmes ni celui des protocoles. Il
les **agrège** : chaque algorithme et chaque protocole vit dans son propre dépôt
GitHub, rattaché ici en tant que
[Git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules).

---

## Swarm Command Center

La [spécification détaillée premium V3.0](spec/Swarm_Command_Center_Specification_Premium_V3_0.md) définit l'interface de compréhension opérationnelle de l'essaim : supervision, mission, architecture LikeC4, décisions algorithmiques, historique, simulation et preuves scientifiques. La [version Word](spec/Swarm_Command_Center_Specification_Premium_V3_0.docx) est disponible pour revue et diffusion.

Le document distingue les exigences produit des éléments déjà validés par le modèle LikeC4 et signale explicitement les décisions encore ouvertes.

## Pourquoi cette organisation

Un algorithme d'essaim est un objet autonome : il a son propre cycle de vie, ses
propres tests, sa propre parité de référence, ses propres versions. Le mélanger
au code d'orchestration produit un dépôt qui ne peut plus évoluer sans casser
autre chose.

L'architecture retenue sépare les deux responsabilités :

- **`SwarmDrones`** — l'orchestrateur. Il déclare *quels* algorithmes et
  protocoles composent la solution, à *quelle version* ils sont épinglés, et
  comment ils s'articulent.
- **`alg-<nom>`** — un dépôt par algorithme. Il contient le code, les tests, la
  documentation de référence et l'historique de ses propres versions.

Le submodule est le lien : `SwarmDrones` enregistre un **commit précis** de
chaque algorithme, pas une branche flottante. La solution est donc
reproductible — cloner `SwarmDrones` à une date donnée redonne exactement les
mêmes versions d'algorithmes.

### Algorithmes et protocoles

Le dépôt distingue deux natures d'objets :

- **`algorithms/`** — les algorithmes de décision (consensus, allocation de
  tâches). Ce sont des **calculs** : ils transforment un état en une décision.
- **`protocols/`** — les protocoles de communication. Ce sont des **contrats
  d'échange** : ils fixent le format des messages, les fréquences, les règles
  de rejet et les garanties de convergence. Un protocole n'est pas un
  algorithme — il est spécifié de façon normative, puis implémenté.

---

## Algorithmes

| Algorithme | Dépôt | Chemin | Spécification |
|---|---|---|---|
| `ALG_COLLISION_AVOIDANCE` | [dagornc/alg-collision-avoidance](https://github.com/dagornc/alg-collision-avoidance) | `algorithms/collision-avoidance` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_COLLISION_AVOIDANCE_v1.pdf) |
| `ALG_CONSENSUS` | [dagornc/alg-consensus](https://github.com/dagornc/alg-consensus) | `algorithms/consensus` | [PDF](https://likec4.breizh.ai/Spec_ALG_CONSENSUS_v7.pdf) |
| `ALG_COOPERATIVE_LOCALIZATION` | [dagornc/alg-cooperative-localization](https://github.com/dagornc/alg-cooperative-localization) | `algorithms/cooperative-localization` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_COOPERATIVE_LOCALIZATION_v1.pdf) |
| `ALG_ENERGY_AWARE` | [dagornc/alg-energy-aware](https://github.com/dagornc/alg-energy-aware) | `algorithms/energy-aware` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_ENERGY_AWARE_v1.pdf) |
| `ALG_EVENT_TRIGGERED_COMM` | [dagornc/alg-event-triggered-comm](https://github.com/dagornc/alg-event-triggered-comm) | `algorithms/event-triggered-comm` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_EVENT_TRIGGERED_COMM_v1.pdf) |
| `ALG_FAULT_TOLERANT_CONTROL_ALLOC` | [dagornc/alg-fault-tolerant-control-alloc](https://github.com/dagornc/alg-fault-tolerant-control-alloc) | `algorithms/fault-tolerant-control-alloc` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_FAULT_TOLERANT_CONTROL_ALLOC_v1.pdf) |
| `ALG_FORMATION_CONTROL` | [dagornc/alg-formation-control](https://github.com/dagornc/alg-formation-control) | `algorithms/formation-control` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_FORMATION_CONTROL_v1.pdf) |
| `ALG_HEALTH_MONITORING` | [dagornc/alg-health-monitoring](https://github.com/dagornc/alg-health-monitoring) | `algorithms/health-monitoring` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_HEALTH_MONITORING_v1.pdf) |
| `ALG_JAMMING_RESILIENT_MODE` | [dagornc/alg-jamming-resilient-mode](https://github.com/dagornc/alg-jamming-resilient-mode) | `algorithms/jamming-resilient-mode` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_JAMMING_RESILIENT_MODE_v1.pdf) |
| `ALG_LEADER_ELECTION` | [dagornc/alg-leader-election](https://github.com/dagornc/alg-leader-election) | `algorithms/leader-election` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_LEADER_ELECTION_v2.pdf) |
| `ALG_NAV_GNSS_DEGRADE` | [dagornc/alg-nav-gnss-degrade](https://github.com/dagornc/alg-nav-gnss-degrade) | `algorithms/nav-gnss-degrade` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_NAV_GNSS_DEGRADE_v2.pdf) |
| `ALG_PATH_PLANNING` | [dagornc/alg-path-planning](https://github.com/dagornc/alg-path-planning) | `algorithms/path-planning` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_PATH_PLANNING_v1.pdf) |
| `ALG_PERCEPTION_FUSION` | [dagornc/alg-perception-fusion](https://github.com/dagornc/alg-perception-fusion) | `algorithms/perception-fusion` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_PERCEPTION_FUSION_v1.pdf) |
| `ALG_SAFETY_RULES` | [dagornc/alg-safety-rules](https://github.com/dagornc/alg-safety-rules) | `algorithms/safety-rules` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_SAFETY_RULES_v1.pdf) |
| `ALG_TASK_ALLOCATION` | [dagornc/alg-task-allocation](https://github.com/dagornc/alg-task-allocation) | `algorithms/task-allocation` | [PDF](https://likec4.breizh.ai/Spec_ALG_TASK_ALLOCATION_V3.pdf) |

> **Note sur les versions épinglées.** Chaque submodule épingle un **commit
> précis**, pas une branche flottante. Pour `ALG_CONSENSUS` et
> `ALG_TASK_ALLOCATION`, l'épingle pointe la branche **`v4`** (couche
> opérationnelle, loop engineering) et non `master` : dans ces deux dépôts,
> `master` porte la **v1 historique** et les versions ultérieures vivent sur
> des branches dédiées (`v2`, `v3`, `v4`, …). Épingler `master` reviendrait à
> rétrograder la solution. Vérifier l'épingle avec `git submodule status`.

### ALG_CONSENSUS

Consensus d'état d'essaim par **CRDT semi-treillis** et **gossip** sur une
topologie de grille 6×5. Chaque agent détient un état partiel ; la fusion est
commutative, associative et idempotente, ce qui garantit la convergence sans
coordinateur.

- Implémentation **Rust**, à **parité bit-à-bit** avec une référence Python
  embarquée (MT19937 compatible CPython).
- 9 tests de harnais (T1…T9), 62 tests automatisés, 0 warning.
- Documentation de référence complète : 22 chapitres couvrant le
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
- **v4** (branche `v4`) — couche **opérationnelle** (loop engineering) : les 8
  building blocks rendus exécutables — état de consensus complet, persistance,
  reconfiguration dynamique, vérification indépendante (maker/checker),
  supervision avec escalade, budgets durs. Parité v1 préservée bit-à-bit.

**Notation /20** (grille : correction 6, coût 5, latence 3, robustesse 3,
ingénierie 3) : **v1 15,7** · **v2 16,3** · **v3 19,5** · **v4 17,5**.

→ [Lire la documentation](https://github.com/dagornc/alg-consensus/blob/master/README.md)
→ [Section 20 — la v2](https://github.com/dagornc/alg-consensus/blob/v2/README.md#20-version-2--reconnexion-des-agents-isolés)
→ [Section 21 — la v3](https://github.com/dagornc/alg-consensus/blob/v3/README.md#21-version-3--quiescence-loop-engineering)
→ [Section 22 — la v4](https://github.com/dagornc/alg-consensus/blob/v4/README.md#22-version-4--couche-opérationnelle-loop-engineering)

### ALG_TASK_ALLOCATION

Allocation de tâches par **essaim de drones** : CBBA événementiel (ED-CBBA)
avec résilience aux partitions. Chaque agent construit un bundle de tâches,
le diffuse, et résout les conflits par consensus ; la partition est détectée
par quorum et le système se dégrade proprement.

- Implémentation **Rust**, à **parité bit-à-bit** avec une référence Python
  embarquée (MT19937 compatible CPython).
- 30 agents, grille 6×5, 8 building blocks opérationnels.
- Réduction du trafic de **95,2 %** sur T8 (1,96 M → 93 k messages).

**Versions disponibles :**

- **v1** (branche `master`) — CBBA standard, comportement historique.
- **v2** (branche `v2`) — auto-amélioration de l'allocation par rejeu
  (méthode Dream-RSI).
- **v3** (branche `v3`) — boucle d'ingénierie (loop engineering) : résilience
  aux partitions, condition d'arrêt vérifiable.
- **v4** (branche `v4`) — couche **opérationnelle** (loop engineering) : les 8
  building blocks rendus exécutables — état complet d'affectation,
  persistance, reconfiguration dynamique, vérification indépendante
  (maker/checker), supervision avec escalade, budgets durs.

**Notation /20** : **v1 15,0** · **v2 16,5** · **v3 19,5** · **v4 17,5**.

→ [Lire la documentation](https://github.com/dagornc/alg-task-allocation/blob/master/README.md)
→ [Section 12 — la v4](https://github.com/dagornc/alg-task-allocation/blob/v4/README.md)

---

## Protocoles

| Protocole | Dépôt | Chemin | Version épinglée |
|---|---|---|---|
| `H-Zip` | [dagornc/alg-protocole](https://github.com/dagornc/alg-protocole) | `protocols/hzip` | voir `git submodule status` |

### H-Zip v2.2 « Dream-Zip »

Protocole de communication **event-driven** pour essaim hétérogène haute densité
(UAV / USV / UUV, 200 nœuds). L'état par défaut est le **silence radio** : on ne
transmet que pour corriger une déviation par rapport à un modèle prédictif
partagé.

Trois principes :

- **Event-driven** — le silence est nominal, la transmission est l'exception.
- **Hiérarchie par hubs** — les USV servent de routeurs de secteur ; les
  terminaux ne s'adressent qu'à leur hub, ce qui élimine le broadcast storm à
  200 nœuds.
- **Séparation stricte des couches** — prédiction déterministe, réactive (APF),
  stratégique (PSO), cognitive (LLM). **Le LLM est interdit dans les couches
  temps réel.**

Quatre trames : **R** (4 o, APF, 10–50 Hz) · **S** (8 o, PSO, 0,1–2 Hz) ·
**D** (4 o, delta, événementielle) · **H** (8 o, heartbeat, 1/0,5/0,2 Hz).

**Statut : spécification normative figée, implémentation non commencée.** Les
**19 paramètres** du protocole, les **4 formats de trame** et la **borne de
divergence** `e_max ≤ ½·a_max·T_hb²` sont figés. Les bornes physiques
(`a_max`, `d_min`, capacité modem — décisions DE-05/06/07) restent **à
mesurer**.

→ [Lire la documentation](https://github.com/dagornc/alg-protocole/blob/main/README.md)
→ [Spécification complète](https://github.com/dagornc/alg-protocole/blob/main/spec/H-Zip_v2.2_Specification_premium.docx)

---

## Spécifications

Chaque algorithme dispose d'une **spécification normative** (PDF) et d'un
**modèle SysML v2** (`.sysml`). Ces artefacts sont hébergés publiquement sur
[likec4.breizh.ai](https://likec4.breizh.ai) et rattachés aux cartes du modèle
d'architecture LikeC4.

Le modèle d'architecture complet est maintenu dans le dépôt
`dagornc/swarmdrones-likec4` et publié sur
[likec4.breizh.ai](https://likec4.breizh.ai).

### Spécifications par algorithme

| Algorithme | Spécification PDF | Modèle SysML v2 |
|---|---|---|
| `ALG_CONSENSUS` | [PDF](https://likec4.breizh.ai/Spec_ALG_CONSENSUS_v7.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_CONSENSUS.sysml) |
| `ALG_TASK_ALLOCATION` | [PDF](https://likec4.breizh.ai/Spec_ALG_TASK_ALLOCATION_V3.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_TASK_ALLOCATION.sysml) |
| `ALG_COLLISION_AVOIDANCE` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_COLLISION_AVOIDANCE_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_COLLISION_AVOIDANCE.sysml) |
| `ALG_COOPERATIVE_LOCALIZATION` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_COOPERATIVE_LOCALIZATION_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_COOPERATIVE_LOCALIZATION.sysml) |
| `ALG_ENERGY_AWARE` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_ENERGY_AWARE_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_ENERGY_AWARE.sysml) |
| `ALG_EVENT_TRIGGERED_COMM` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_EVENT_TRIGGERED_COMM_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_EVENT_TRIGGERED_COMM.sysml) |
| `ALG_FAULT_TOLERANT_CONTROL_ALLOC` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_FAULT_TOLERANT_CONTROL_ALLOC_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_FAULT_TOLERANT_CONTROL_ALLOC.sysml) |
| `ALG_FORMATION_CONTROL` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_FORMATION_CONTROL_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_FORMATION_CONTROL.sysml) |
| `ALG_HEALTH_MONITORING` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_HEALTH_MONITORING_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_HEALTH_MONITORING.sysml) |
| `ALG_JAMMING_RESILIENT_MODE` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_JAMMING_RESILIENT_MODE_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_JAMMING_RESILIENT_MODE.sysml) |
| `ALG_LEADER_ELECTION` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_LEADER_ELECTION_v2.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_LEADER_ELECTION.sysml) |
| `ALG_NAV_GNSS_DEGRADE` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_NAV_GNSS_DEGRADE_v2.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_NAV_GNSS_DEGRADE.sysml) |
| `ALG_PATH_PLANNING` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_PATH_PLANNING_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_PATH_PLANNING.sysml) |
| `ALG_PERCEPTION_FUSION` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_PERCEPTION_FUSION_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_PERCEPTION_FUSION.sysml) |
| `ALG_SAFETY_RULES` | [PDF](https://likec4.breizh.ai/specification/Spec_ALG_SAFETY_RULES_v1.pdf) | [SysML](https://likec4.breizh.ai/sysml/ALG_SAFETY_RULES.sysml) |

### Spécification système

| Document | Lien |
|---|---|
| Spécification système essaim de drones (v2) | [PDF](https://likec4.breizh.ai/specification/Specification_Systeme_Essaim_Drones_v2.pdf) |
| H-Zip v2.2 « Dream-Zip » — spécification premium | [DOCX](https://likec4.breizh.ai/H-Zip_v2.2_Specification_premium.docx) |

### Traçabilité

Le modèle LikeC4 relie chaque algorithme à son code source, sa spécification,
son modèle SysML et ses **fondations scientifiques** (articles de référence
vérifiés par DOI). Les cartes du modèle sont la source de vérité pour les
relations `implements`, `documentedBy` et `evidences`.


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

## Ajouter un protocole

Un protocole suit la même mécanique, mais son contenu est **normatif** : il
décrit un contrat d'échange (formats de trame, fréquences, règles de rejet,
garanties), pas un calcul.

1. Créer le dépôt dédié sur GitHub, nommé `alg-<nom>`.
2. Y publier la **spécification normative** (document de référence) et un
   `README.md` de synthèse.
3. Rattacher le dépôt ici :

```bash
git submodule add https://github.com/dagornc/alg-<nom>.git protocols/<nom>
git commit -m "protocols/<nom> : rattachement du submodule"
```

4. Mettre à jour le tableau « Protocoles » de ce README.

**Règle de cohérence.** Le modèle d'architecture LikeC4 **NE DOIT PAS**
contredire la spécification du protocole. En cas d'écart, **la spécification
fait foi**.

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

La même procédure s'applique aux protocoles, sous `protocols/<nom>`.

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

### Standard de documentation d'un protocole

Un protocole n'a ni installation ni paramètres d'entrée : son README est une
**synthèse normative**, pas un manuel d'utilisation. Il doit couvrir :

1. En bref — tableau de synthèse
2. Le problème résolu
3. Les principes du protocole
4. Les formats de trame (tableau par trame, champ par champ)
5. Les couches et leurs règles d'isolation
6. Les paramètres figés (avec leur source de vérité)
7. Les garanties formelles (bornes, preuves informelles)
8. Les contraintes de portabilité
9. Les critères de conformité
10. Les métriques de vérification mesurables
11. Les points ouverts
12. Structure du dépôt
13. La spécification normative (lien vers le document de référence)
14. Bibliographie vérifiée
15. Licence

Le modèle de référence est le
[README de `alg-protocole`](https://github.com/dagornc/alg-protocole/blob/main/README.md).

---

## Licence

MIT — Copyright (c) 2026 Christophe Dagorn.
