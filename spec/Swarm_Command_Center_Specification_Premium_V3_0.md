# Swarm Command Center

## Spécification détaillée premium

**Version** 3.0  
**Date de référence** 22 septembre 2026  
**Statut** Baseline de conception et de consultation  
**Système** SwarmDrones  
**Configuration nominale** 30 plateformes hétérogènes  
**Cible d'architecture** 100 plateformes et plus  
**Cible de stress** 200 plateformes

## Résumé exécutif

Le Swarm Command Center est le poste de compréhension, de supervision et d'autorité humaine du système SwarmDrones. Il unifie l'état opérationnel, la mission, la topologie de l'essaim, les décisions algorithmiques, l'historique, la simulation, l'architecture LikeC4 et les preuves scientifiques dans une interface cohérente. Il ne pilote pas les boucles de vol temps réel et ne devient jamais une dépendance de sûreté pour les plateformes.

La conception repose sur un principe déterminant : le contrat de mission est embarqué et le lien est opportuniste. Une plateforme doit poursuivre ou terminer sa mission nominale sans C2 ni Cloud, selon les règles préapprouvées. L'Edge maintient la coordination de proximité lorsqu'il est isolé du C2. Le C2 assure la supervision, la planification, l'autorisation humaine et l'audit quand le lien est disponible. Le Cloud traite le rejeu, l'analyse et les modèles en différé, avec un budget de l'ordre de la minute.

LikeC4 est la source de vérité de l'architecture sémantique. Le jumeau numérique décrit ce qui se passe réellement. Le World Model décrit ce qui est perçu ou estimé dans l'environnement. La simulation réalise une dynamique physique et temporelle sans modifier la définition architecturale. L'interface traduit ces sources en une compréhension opérateur, avec une indication explicite de la fraîcheur, de l'incertitude et du niveau de preuve.

Cette version consolide la spécification V2.0 avec le modèle publié sur `likec4.breizh.ai`. Elle reprend les quatre zones Onboard, Edge, C2 et Cloud, les décisions ADR-1 à ADR-5, les hypothèses H1 à H5, les invariants, les risques, les angles morts, le protocole H-Zip v2.2 et les contrats de consommation du modèle. Les valeurs non mesurées restent marquées EXTRAP ou TBD. La spécification ne transforme aucun objectif, critère ou hypothèse en résultat acquis.

## 1 Objet et portée

### 1.1 Finalité

Le document définit les exigences produit, fonctionnelles, UX, logicielles, data, cybersécurité, simulation, validation et exploitation du Swarm Command Center. Il sert de contrat commun aux responsables produit, opérateurs, architectes, équipes frontend et backend, roboticiens, spécialistes algorithmiques, QA, cybersécurité et facteurs humains.

### 1.2 Résultat attendu

Le système doit permettre à un opérateur autorisé de répondre, sans changer d'outil, aux questions suivantes :

- Quelle mission est active et quel est son avancement
- Quel est l'état global de l'essaim et de ses groupes
- Quelles plateformes, tâches ou liaisons sont dégradées
- Quelle information est confirmée, estimée, prédite, périmée ou inconnue
- Quelle décision algorithmique a été prise et sur quelles entrées
- Quel composant et quel algorithme sont responsables du comportement observé
- Quelle architecture était attendue et quel runtime est réellement observé
- Quelle source technique ou scientifique justifie une affirmation
- Quelle action humaine est autorisée, avec quel impact et quelle traçabilité
- Comment reconstruire exactement un événement passé ou comparer un scénario simulé

### 1.3 Périmètre inclus

Le périmètre couvre la supervision d'un essaim hétérogène, la préparation et le suivi de mission, l'agrégation d'alertes, la visualisation 2D et 3D, le graphe réseau, l'historisation, le rejeu, le mode simulation, l'exploration de l'architecture, les traces de décision, la gestion des commandes C2, l'autorisation humaine, l'audit, l'observabilité et la détection d'écarts entre l'architecture et le runtime.

### 1.4 Hors périmètre

Le Swarm Command Center ne réalise pas la stabilisation, le contrôle d'attitude, l'évitement local à très faible latence, les réflexes de sûreté embarqués, le calcul direct des actionneurs, la fusion capteur primaire nécessaire au vol, ni une décision létale autonome. Il ne garantit pas une connectivité permanente et n'invente pas les propriétés physiques absentes du modèle.

## 2 Base de référence et statut des connaissances

### 2.1 Sources de conception

La baseline s'appuie sur trois sources :

1. La spécification consolidée V2.0 fournie avec la demande.
2. Le modèle SwarmDrones publié sur `https://likec4.breizh.ai`, consulté le 22 septembre 2026.
3. La documentation officielle LikeC4 pour le modèle logique et le modèle de déploiement.

Le modèle LikeC4 public prévaut pour les identifiants, les zones, les relations et les statuts épistémiques. La présente spécification prévaut pour le comportement attendu du produit Swarm Command Center. Un conflit entre les deux doit produire une décision ouverte et non une correction silencieuse.

### 2.2 Niveaux d'autorité

| Niveau | Source | Autorité |
|---|---|---|
| A | Invariant ou décision approuvée dans LikeC4 | Contrainte de conception obligatoire |
| B | Élément, relation, message ou scénario LikeC4 | Contrat sémantique à conserver |
| C | Exigence normative de cette spécification | Contrat produit à implémenter |
| D | Valeur EXTRAP ou hypothèse | Cible à mesurer ou à valider |
| E | Proposition UX ou choix technologique | Réversible sous ADR |

### 2.3 Statuts épistémiques

Toute donnée ou affirmation affichée doit porter un statut compatible avec la source : `VALIDÉ`, `EXTRAP`, `À VALIDER`, `TBD`, `NON VÉRIFIÉ`, `CONTREDIT` ou `NON APPLICABLE`. Une valeur simulée doit être distinguée d'une valeur mesurée. Un critère d'acceptation non exécuté ne doit jamais être présenté comme un résultat.

### 2.4 Incohérence de taxonomie à résoudre

Le modèle public fixe la population à 18 UAV-R, 8 UAV-F et 4 USV, soit 30 plateformes. Deux descriptions publiques associent toutefois différemment UAV-R et UAV-F aux catégories multirotor et voilure fixe. La présente spécification conserve les identifiants et les effectifs, mais interdit d'afficher le type aérodynamique comme vérité tant que la décision `DE-SCC-01` n'est pas tranchée.

## 3 Vocabulaire normatif

`DOIT` exprime une exigence obligatoire. `DEVRAIT` exprime une recommandation dont l'écart doit être justifié. `PEUT` exprime une option. `INTERDIT` exprime une contrainte négative. Les identifiants `SCC-FR`, `SCC-NFR`, `SCC-UX`, `SCC-DATA`, `SCC-SEC` et `SCC-VAL` sont stables et doivent être repris dans les tickets, tests et revues.

## 4 Principes directeurs

| ID | Principe | Conséquence vérifiable |
|---|---|---|
| P-01 | LikeC4 est la source de vérité sémantique | Aucun consommateur ne crée d'identifiant architectural concurrent |
| P-02 | Architecture et runtime sont distincts | L'attendu et l'observé sont stockés et affichés séparément |
| P-03 | L'essaim précède la plateforme | L'écran s'ouvre au niveau Swarm puis permet le zoom Group et Platform |
| P-04 | La sûreté est locale | Aucune action C2 ou Cloud n'est requise pour un réflexe de sûreté |
| P-05 | Le lien est intermittent | Toute perte de lien entraîne un statut de fraîcheur, pas une panne fictive |
| P-06 | Le temps est une dimension de premier rang | Le système distingue passé, présent, prédiction et simulation |
| P-07 | La 3D est une représentation | Aucune logique mission ou commande critique ne réside dans le moteur 3D |
| P-08 | L'incertitude est visible | Une donnée inconnue n'est jamais remplacée par zéro ou une valeur plausible |
| P-09 | Une décision doit être explicable | Toute décision significative produit une trace versionnée et corrélée |
| P-10 | Une action humaine est attribuable | Toute action critique possède un utilisateur, une autorisation et un résultat |

## 5 Contexte opérationnel aligné sur LikeC4

### 5.1 Population nominale

Le scénario nominal comprend 30 plateformes hétérogènes : 18 UAV-R, 8 UAV-F et 4 USV. Le modèle logique représente une plateforme type et le modèle de déploiement matérialise les 30 instances. L'architecture doit accepter une ou plusieurs stations Edge, un C2 et un Cloud, tout en permettant leur absence opérationnelle dans les scénarios autorisés.

### 5.2 Hypothèses structurantes

| ID | Hypothèse | Impact produit |
|---|---|---|
| H1 | Chaque plateforme dispose d'un autopilote et d'un mode mission embarqué | Le Command Center envoie un contrat de mission, pas une consigne temps réel continue |
| H2 | Les liens sont intermittents par défaut | Le produit gère les trous de données, le store-and-forward et la reconnexion |
| H3 | Le GNSS peut être dégradé ou brouillé | L'interface expose source, qualité et dérive de position sans masquer l'incertitude |
| H4 | L'Edge peut relayer la coordination hors portée directe | Le produit distingue topologie directe, relayée et inconnue |
| H5 | L'humain garde l'autorité sur les actions critiques | Le parcours de commande impose autorisation, confirmation et audit |

### 5.3 Décisions d'architecture acquises

| ID | Décision | Traduction dans le Command Center |
|---|---|---|
| ADR-1 | Contrat de mission embarqué et replanification opportuniste | La perte du C2 n'interrompt pas la mission nominale |
| ADR-2 | Fraîcheur explicite sans cohérence forte globale | Chaque état porte temps source, temps de réception et durée de validité |
| ADR-3 | Canaux de sûreté et de sécurité séparés | Le C2 ne bloque jamais le chemin local de sûreté |
| ADR-4 | Traitement lourd hors boucle de contrôle | Analytics et entraînement restent Edge non critique ou Cloud différé |
| ADR-5 | Robustesse privilégiée à l'optimalité sous partition | L'interface accepte le consensus éventuel et rend les partitions visibles |

### 5.4 Invariants

- `INV-1` La mission nominale doit pouvoir continuer sans C2 ni Cloud.
- `INV-2` La voie de sûreté doit rester locale et préemptive.
- `INV-3` Le comportement à 30 plateformes en vol réel n'est pas considéré prouvé.
- `INV-4` LikeC4 ne contient ni coordonnées 3D, ni caméra, ni animation, ni matériaux.
- `INV-5` Le statut épistémique d'une donnée ou d'une affirmation doit être préservé de la source jusqu'à l'interface.

## 6 Architecture fonctionnelle du produit

### 6.1 Capacités

Le Command Center couvre huit capacités transverses : voler, percevoir, décider, coopérer, communiquer, survivre, observer et commander. Il observe toutes les capacités mais n'exécute directement que les fonctions de supervision, de planification, d'autorisation, de commande et d'analyse.

### 6.2 Blocs fonctionnels

| Bloc | Responsabilité | Criticité produit |
|---|---|---|
| Situation opérationnelle | Vue agrégée de l'essaim, des groupes, de la mission et du réseau | Mission |
| Mission | Préparation, versionnement, distribution et suivi du contrat de mission | Mission |
| Alertes | Détection, corrélation, priorisation et acquittement opérateur | Mission |
| Commandes | Autorisation, émission, accusé, exécution et résultat | Safety ou mission selon le type |
| Jumeau numérique | État courant, événements, historique et prédictions | Observation |
| Rejeu | Reconstruction déterministe et navigation temporelle | Analyse |
| Simulation | Exécution de scénarios et comparaison à une baseline | Validation |
| Architecture | Exploration LikeC4, mapping runtime et détection de dérive | Gouvernance |
| Science | Traçabilité des algorithmes vers spécifications et publications | Connaissance |
| Audit | Preuve des actions, configurations et décisions | Gouvernance |

## 7 Architecture de zones

### 7.1 Zone Onboard

La zone Onboard est instanciée sur chaque plateforme. Elle doit poursuivre la mission nominale sans C2 ni Cloud. Les identifiants LikeC4 suivants constituent la baseline :

| Identifiant | Rôle | Fréquence ou contrainte publiée |
|---|---|---|
| `onboard.sensorsHAL` | Interfaces capteurs et abstraction matérielle | Selon capteur, best effort |
| `onboard.perception` | Fusion, localisation et détection d'évitement | 10 à 60 Hz, safety critical, réaction visée inférieure ou égale à 100 ms EXTRAP |
| `onboard.health` | Diagnostic batterie, moteurs, capteurs et température | 1 à 10 Hz |
| `onboard.energy` | Marge énergétique et budget de retour | 0,2 à 2 Hz |
| `onboard.mission` | Machine à états et exécution du contrat embarqué | 1 à 10 Hz, mission critical |
| `onboard.taskAuction` | Allocation locale par enchères et consensus éventuel | 0,2 à 1 Hz |
| `onboard.telemetry` | Priorisation, compactage delta et fraîcheur | 1 à 10 Hz selon mode |
| `onboard.linkRadio` | Voisinage, mesh, relais, chiffrement et store-and-forward | Continu, lien intermittent |
| `onboard.safety` | Geofence, perte de contrôle, RTL et atterrissage d'urgence | Local, safety critical |
| `onboard.autopilot` | Attitude, position, actionneurs et exécution bas niveau | 50 à 400 Hz, budget 10 ms publié |

La machine de mission publiée est `INIT → READY → LAUNCH → EXECUTE ↔ DEGRADED → RTL ou RECOVER → LAND → DONE`, avec `ABORT` possible depuis tout état compatible. L'interface doit afficher l'état observé et sa source, sans simuler une transition absente des événements.

### 7.2 Zone Edge

L'Edge fonctionne en mode C2 déconnecté. Il consolide les flux radio, maintient une image tactique locale et assure la continuité :

| Identifiant | Responsabilité |
|---|---|
| `edge.relay` | Relais de proximité, mesh, lien longue portée et store-and-forward |
| `edge.fusion` | Pistes consolidées, carte locale, déconfliction et perte d'agent |
| `edge.coordinator` | Allocation et résolution de conflits locale quand le C2 est indisponible |
| `edge.cache` | Persistance locale de mission, logs et artefacts |
| `edge.gateway` | Normalisation, contrôle d'accès et frontière de confiance vers C2 et Cloud |

Le budget publié pour la coordination locale, inférieur ou égal à une seconde, reste EXTRAP. Le Command Center doit l'afficher comme objectif de conception tant qu'une campagne de mesure ne l'a pas validé.

### 7.3 Zone C2

Le C2 est intermittent et ne porte aucune responsabilité de sûreté embarquée :

| Identifiant | Responsabilité |
|---|---|
| `c2.gcs` | Console opérateur, supervision et unique chemin d'autorisation humaine |
| `c2.planner` | Plans embarquables versionnés et signés, replanification opportuniste |
| `c2.missionStore` | Source de vérité des versions, plans, statuts et traces |
| `c2.auth` | Authentification, autorisation, signature et `authorization_ref` |
| `c2.alerting` | Agrégation et notification des alarmes |

Le planificateur vise un budget de 1 à 30 secondes, marqué EXTRAP. Une indisponibilité de `c2.planner` ou `c2.gcs` ne doit pas modifier l'état local de sûreté d'une plateforme.

### 7.4 Zone Cloud

Le Cloud est hors boucle de contrôle et travaille en différé :

| Identifiant | Responsabilité |
|---|---|
| `cloud.replayStore` | Journaux rejouables et non-régression |
| `cloud.twin` | Rejeu, simulation, prédiction de dérive et calibration |
| `cloud.analytics` | Analyse post-mission et détection d'anomalies a posteriori |
| `cloud.modelRegistry` | Versionnement des modèles et configurations |

Le retour `cloud.modelRegistry → c2.planner` est non bloquant. Aucun composant Onboard ou Edge ne doit avoir de dépendance synchrone requise vers le Cloud.

## 8 Modèle d'information unifié

### 8.1 Séparation des modèles

| Modèle | Question | Contenu | Ne contient pas |
|---|---|---|---|
| Architecture LikeC4 | Que doit être le système | Éléments, relations, responsabilités, preuves et déploiements | État opérationnel instantané |
| Runtime observé | Qu'est-ce qui tourne réellement | Instances, versions, topics, services et santé | Intention architecturale |
| Digital Twin | Que fait le système dans le temps | États, missions, tâches, événements, décisions et prédictions | Vérité architecturale statique |
| World Model | Que perçoit ou estime l'essaim | Objets, obstacles, cibles, terrain, zones et incertitude | État complet de l'organisation |
| Simulation | Que se passerait-il sous un scénario | Horloge, dynamique, capteurs, réseau et physique | Autorité sur le réel |
| Vue opérateur | Que doit comprendre l'humain | Synthèse, alertes, contexte, actions et preuves | Logique de vol critique |

### 8.2 Architecture Graph canonique

Le pipeline doit être `LikeC4 → export → parser → normalizer → Canonical Architecture Graph → manifest → delta engine → consommateurs`. Le format canonique doit être indépendant du format interne LikeC4 et contenir version, date de génération, éléments, relations, messages, documents, algorithmes, décisions, hypothèses, risques, angles morts et mappings runtime.

Chaque élément doit au minimum porter `id`, `likec4Id`, `name`, `kind`, `parentId`, `description`, `tags`, `technologies`, `metadata`, `documents` et `runtimeMappings`. Les identifiants LikeC4 doivent être immuables après publication. Un renommage métier doit préserver un alias ou une migration explicite.

### 8.3 Digital Twin

Le Digital Twin agrège `SwarmState`, `GroupState`, `PlatformState`, `MissionState`, `TaskState`, `NetworkState`, `WorldModelState`, `AlertState`, `CommandState`, `DecisionTrace`, `Event` et `PredictionState`. Chaque objet doit porter sa provenance et sa temporalité.

### 8.4 Enveloppe temporelle commune

Tout état diffusé doit contenir :

```json
{
  "schemaVersion": "1.0",
  "entityId": "uav-17",
  "sourceId": "edge.gateway-01",
  "sourceTimestamp": "2026-09-22T10:15:12.120Z",
  "receivedAt": "2026-09-22T10:15:12.340Z",
  "validUntil": "2026-09-22T10:15:14.120Z",
  "sequence": 18452,
  "confidence": 0.91,
  "epistemicStatus": "VALIDATED",
  "payload": {}
}
```

Le client calcule `dataAge` à partir du temps source lorsque l'horloge est fiable, sinon à partir du temps de réception et en indique la limite. Il ne doit pas corriger silencieusement l'horloge d'une source.

### 8.5 États de fraîcheur

| État | Règle |
|---|---|
| FRESH | Âge inférieur au seuil nominal du flux |
| AGING | Âge entre le seuil nominal et `validUntil` |
| STALE | Validité dépassée mais dernière valeur encore affichée |
| LOST | Source attendue sans mise à jour au-delà du seuil de perte |
| UNKNOWN | Aucune valeur fiable n'a été reçue |

Les seuils sont configurables par type de flux et versionnés avec la mission.

## 9 Contrats de messages

### 9.1 Catalogue de référence

Le modèle public annonce 13 messages sans orphelin. La baseline produit doit couvrir au minimum : Heartbeat, DroneState, NeighborState, PositionUpdate, TaskBid, TaskAssignment, TaskStatus, TrajectoryProposal, TrajectoryCommand, FormationState, CollisionWarning, MissionCommand et FailureNotification. Le catalogue exact exporté de LikeC4 reste l'autorité sur les identifiants.

### 9.2 Métadonnées obligatoires

Chaque définition de message doit préciser producteur, consommateurs, canal, protocole, schéma, QoS, fréquence, criticité, comportement en cas de perte, compatibilité de versions, politique de rétention et règles de confidentialité.

### 9.3 Priorités

Les files suivent l'ordre `sûreté → état de mission → télémétrie → debug`. Une saturation du lien ne doit pas permettre à la vidéo, aux traces ou au debug de retarder un message de sûreté. Cette exigence doit être vérifiée avec un budget radio mesuré.

### 9.4 H-Zip v2.2

Le Command Center doit visualiser le protocole H-Zip sans l'exécuter. Les quatre couches sont indépendantes et une couche basse ne dépend jamais d'une couche haute. La couche 3 fondée sur un LLM est en lecture seule et ne peut écrire dans les couches réactives. Les trames R, S, D et H doivent être observables avec producteur, consommateurs, canal, fréquence, taille déclarée et état de fraîcheur. Le silence prédictif et la coordination par LLM doivent rester présentés comme propositions originales tant qu'ils ne sont pas validés.

## 10 Exigences fonctionnelles

### 10.1 Situation opérationnelle

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-001 | Le système DOIT afficher la mission, le nombre de plateformes actives, les groupes, la santé, le réseau et les alertes | Une anomalie critique est localisable depuis la vue initiale sans ouvrir plus de deux niveaux |
| SCC-FR-002 | Le système DOIT agréger les plateformes par groupe et rôle | La somme des membres correspond aux plateformes connues, avec un groupe `non attribué` explicite |
| SCC-FR-003 | Le système DOIT distinguer plateforme absente, déconnectée, silencieuse et défaillante | Aucun de ces états n'est inféré uniquement d'une absence de télémétrie |
| SCC-FR-004 | Le système DOIT permettre un zoom sémantique Swarm, Group, Platform | Le changement de niveau conserve le contexte mission et temporel |
| SCC-FR-005 | Le système DOIT afficher la source et l'âge d'une valeur critique | La provenance est accessible en un geste depuis toute métrique critique |

### 10.2 Mission et tâches

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-010 | Toute mission DOIT être versionnée et signée avant distribution | La version et l'empreinte sont visibles dans le détail mission |
| SCC-FR-011 | Le système DOIT comparer plan attendu, plan distribué et plan accusé | Toute divergence produit un événement corrélé |
| SCC-FR-012 | Une tâche DOIT porter priorité, statut, contraintes, assignation et progression | Les champs inconnus sont affichés `inconnus` |
| SCC-FR-013 | Une réallocation DOIT conserver l'assignation précédente et sa cause | Le replay montre avant, déclencheur, décision et après |
| SCC-FR-014 | La reconnexion NE DOIT PAS réémettre une commande ou un plan sans décision explicite | Un test rejoue une coupure après `SENT` sans duplicata |

### 10.3 Alertes

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-020 | Les alertes DOIVENT utiliser INFO, ADVISORY, WARNING et CRITICAL | Chaque niveau possède un traitement visuel et sonore configurable |
| SCC-FR-021 | Une alerte DOIT contenir symptôme, impact, cause probable, entités, temps et actions recommandées | L'absence de cause est affichée comme inconnue |
| SCC-FR-022 | Les alertes corrélées DEVRAIENT former un incident unique | Une tempête de 100 alertes issues d'une partition produit un incident principal et des événements enfants |
| SCC-FR-023 | L'acquittement NE DOIT PAS modifier l'état technique source | L'interface distingue `vu`, `acquitté`, `résolu` et `clos` |
| SCC-FR-024 | Toute alerte critique DOIT rester visible hors du panneau d'alertes | La fermeture du panneau ne masque pas l'état critique global |

### 10.4 Commandes et autorité humaine

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-030 | Une commande DOIT suivre CREATED, AUTHORIZED, SENT, ACKNOWLEDGED, EXECUTING et COMPLETED, ou un état terminal d'échec | Chaque transition produit un événement immuable |
| SCC-FR-031 | Une action critique DOIT inclure `authorization_ref` et signature | Une commande critique sans référence est rejetée côté serveur |
| SCC-FR-032 | L'interface DOIT présenter cible, portée, paramètres, impact attendu et caractère réversible avant confirmation | Le récapitulatif est conservé dans l'audit |
| SCC-FR-033 | Une commande expirée ou ambiguë NE DOIT PAS être relancée automatiquement | La reprise nécessite une nouvelle intention opérateur |
| SCC-FR-034 | La perte du C2 NE DOIT PAS empêcher `onboard.safety` d'agir localement | Le scénario de perte de lien valide un RTL local sans commande C2 |

### 10.5 Historique et rejeu

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-040 | Le rejeu DOIT reconstruire positions, groupes, mission, tâches, alertes, commandes et décisions | Deux relectures du même journal produisent le même état logique |
| SCC-FR-041 | Le lecteur DOIT proposer pause, seek et vitesses 0,25×, 0,5×, 1×, 2× et 10× | La timeline reste synchronisée avec la scène et les panneaux |
| SCC-FR-042 | Le mode REPLAY DOIT être impossible à confondre avec LIVE | Un bandeau persistant affiche mode et horodatage |
| SCC-FR-043 | Le système DOIT permettre des signets et annotations non destructives | Les annotations ne modifient pas le journal source |

### 10.6 Simulation et what if

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-050 | La simulation DOIT réutiliser les interfaces du runtime réel autant que possible | Un même adaptateur de message alimente LIVE et SIMULATION |
| SCC-FR-051 | Chaque scénario DOIT déclarer seed, versions, paramètres et hypothèses | Un run peut être reproduit à paramètres identiques |
| SCC-FR-052 | Les résultats simulés DOIVENT être séparés des mesures réelles | Toute métrique simulée porte `sourceType=simulation` |
| SCC-FR-053 | Le simulateur NE DOIT PAS inventer une capacité batterie ou une portée radio absente du contrat | Le run est bloqué ou marqué non fidèle avec la décision manquante |
| SCC-FR-054 | Le mode what if DOIT montrer baseline, variante et deltas | Les effets prédits ne sont jamais présentés comme conséquences garanties |

### 10.7 Architecture et science

| ID | Exigence | Critère d'acceptation |
|---|---|---|
| SCC-FR-060 | Le mode Architecture DOIT afficher systèmes, composants, runtimes, algorithmes, messages, documents et relations | La navigation conserve les identifiants LikeC4 |
| SCC-FR-061 | Un élément runtime DOIT pouvoir ouvrir son élément LikeC4 attendu | Un mapping absent est affiché comme dérive ou non cartographié |
| SCC-FR-062 | Le système DOIT détecter composant absent, inattendu, mauvaise version, topic manquant et configuration divergente | Chaque écart crée un `ArchitectureDriftEvent` |
| SCC-FR-063 | La chaîne Platform → Component → Algorithm → Specification → Paper DOIT être navigable | Les liens manquants sont visibles et non comblés automatiquement |
| SCC-FR-064 | Le produit DOIT indiquer que 2 algorithmes sur 15 disposent actuellement d'une spécification détaillée dans le modèle public | Les 13 autres portent un statut TBD tant que LikeC4 n'évolue pas |

## 11 UX et architecture de l'information

### 11.1 Modes primaires

Quatre modes sont obligatoires : `LIVE`, `REPLAY`, `SIMULATION` et `ARCHITECTURE`. Le mode est présent dans la barre globale, dans le titre de fenêtre et dans les exports. La couleur seule ne suffit pas. Le changement de mode exige un contexte explicite et ne réutilise pas un état de sélection incompatible.

### 11.2 Cadre d'écran

L'écran comprend une barre de situation globale, un rail de mission et de hiérarchie à gauche, une vue centrale 2D ou 3D, un panneau contextuel à droite, une timeline en bas et une zone événements ou alertes. Les panneaux sont redimensionnables, mémorisés par utilisateur et accessibles au clavier.

### 11.3 Parcours OODA

- **Observe** montre la santé globale, le nombre de plateformes, l'avancement mission, le réseau, les alertes et la fraîcheur.
- **Orient** permet de descendre d'un incident vers groupe, plateforme, liaison, tâche, composant et événement causal.
- **Decide** compare les actions disponibles, les contraintes, l'impact estimé, le niveau de confiance et l'autorité requise.
- **Act** crée une commande, obtient l'autorisation, suit les accusés et enregistre le résultat.

### 11.4 Zoom sémantique

Au niveau Swarm, l'interface affiche synthèse, groupes, progression, santé réseau et incidents. Au niveau Group, elle affiche membres, rôle, formation, tâche, cohésion et partitions. Au niveau Platform, elle affiche énergie, tâche, position, voisins, santé, algorithme actif et âge des données. Les détails capteur ou middleware sont accessibles sans être présents par défaut.

### 11.5 Couches de représentation

Les couches disponibles sont plateformes, groupes, traces, trajectoires planifiées, trajectoires prédites, tâches, graphe de communication, voisinage, couverture, champs potentiels, obstacles, geofences, risques de collision, incertitude, zones explorées et volumes capteurs. Les presets Mission, Réseau, Sûreté, Navigation, Algorithmes et Debug limitent le nombre de couches simultanées.

### 11.6 Vues 2D et 3D

La 2D est la vue par défaut pour planification, zones, tâches, densité et couverture. La 3D est utilisée pour altitude, relief, volumes, trajectoires et relations verticales. Le passage 2D ou 3D conserve sélection, temps, filtres et couche métier. Les coordonnées, caméras, matériaux et animations restent hors de LikeC4.

### 11.7 Incertitude et provenance

Les états `CONFIRMED`, `ESTIMATED`, `PREDICTED`, `UNCERTAIN`, `STALE` et `UNKNOWN` utilisent forme, texte et motif en plus de la couleur. Un panneau de provenance affiche source, timestamp, validité, confiance, méthode de calcul et éventuelle hypothèse. Les prédictions possèdent un horizon et un intervalle d'incertitude.

### 11.8 Facteurs humains

| ID | Exigence UX | Mesure |
|---|---|---|
| SCC-UX-001 | Une anomalie critique DOIT être détectable en moins de 5 secondes dans le scénario nominal | Test opérateur chronométré |
| SCC-UX-002 | Le groupe affecté DOIT être identifiable en moins de 10 secondes | Test avec réseau dégradé |
| SCC-UX-003 | Les plateformes affectées DOIVENT être identifiables en moins de 20 secondes | Test sans connaissance préalable |
| SCC-UX-004 | La cause et la fraîcheur DOIVENT être accessibles sans quitter l'incident | Parcours de deux interactions maximum |
| SCC-UX-005 | Une commande critique DOIT résister au clic accidentel | Confirmation explicite et résumé de portée |
| SCC-UX-006 | Le système DOIT rester exploitable au clavier et avec mouvement réduit | Audit WCAG et test manuel |

## 12 Architecture frontend

Le frontend cible React, TypeScript, React Three Fiber, Three.js, WebGPU avec repli WebGL2, Tailwind CSS, Zustand et Web Workers. Les données fréquentes transitent par `Transport Adapter → Worker → Decoder → Hot State Store → rendu et analytics`. L'état d'interface transite par Zustand et React.

Le Hot State contient position, orientation, vitesse, qualité de lien, énergie, voisinage et trajectoire. L'UI State contient sélection, couches, caméra, panneau, mode et position temporelle. Un flux fréquent ne doit pas déclencher un rerender React complet. Les calculs de trajectoires, partitions réseau et décodages volumineux doivent être déportés dans des Workers.

### 12.1 Adaptateurs de transport

La V1 utilise WebSocket. L'interface `TransportAdapter` permet un futur WebTransport. Le domaine ne dépend jamais d'une API WebSocket. Chaque adaptateur expose connexion, authentification, snapshot, deltas, backpressure, reprise et fermeture.

### 12.2 Reconnexion

Le cycle obligatoire est `DISCONNECTED → RECONNECTING → AUTHENTICATING → REQUESTING_SNAPSHOT → RESYNCING → STREAMING`. Les deltas reçus avant le snapshot sont tamponnés ou rejetés selon séquence. L'état local ne redevient fiable qu'après validation de la version du snapshot et du dernier numéro de séquence.

## 13 Mission Gateway et services backend

Le Mission Gateway assure authentification, autorisation, API mission, commandes, accès au jumeau, événements, historique, architecture, filtrage, limitation de débit et WebSocket. Il ne contient aucune boucle de vol critique.

### 13.1 API minimale

| Méthode | Ressource | Usage |
|---|---|---|
| GET | `/v1/missions` | Liste des missions autorisées |
| GET | `/v1/missions/{id}` | Détail et version active |
| GET | `/v1/swarm` | Snapshot agrégé |
| GET | `/v1/groups/{id}` | Détail groupe |
| GET | `/v1/platforms/{id}` | État plateforme et provenance |
| GET | `/v1/tasks/{id}` | État tâche et historique d'assignation |
| GET | `/v1/events` | Recherche paginée et filtrée |
| GET | `/v1/algorithms` | Catalogue et versions |
| GET | `/v1/architecture` | Manifest canonique |
| GET | `/v1/architecture/deltas` | Changements depuis une version |
| POST | `/v1/commands` | Création idempotente de commande |
| POST | `/v1/simulations` | Création d'un run reproductible |

Les écritures exigent une clé d'idempotence. Les réponses exposent `correlationId`, `schemaVersion` et `architectureVersion`.

### 13.2 WebSocket

L'enveloppe contient version, type, timestamp, entité, séquence, corrélation, provenance et payload. Le client détecte duplicata, ordre incorrect, perte et péremption. Le serveur peut transmettre un watermark de séquence et demander une resynchronisation. La compression et le batching sont négociés.

## 14 Événements, décisions et audit

### 14.1 Event Store

Les événements métier sont immuables et portent `eventId`, temps source, temps d'ingestion, type, source, entité, mission, sévérité, corrélation, causation, schéma et payload. Les événements incluent démarrage et fin de mission, connexion et perte de plateforme, création et réallocation de tâche, changement de formation, dégradation réseau, alerte collision, exécution algorithmique, cycle de commande et dérive d'architecture.

### 14.2 Decision Trace

Toute décision algorithmique significative doit enregistrer algorithme, version, déclencheur, snapshot d'entrée référencé, contraintes, alternatives évaluées, résultat sélectionné, métriques, entités affectées et statut de validation. Les entrées volumineuses sont référencées par empreinte et URI immuable. Une trace ne doit pas prétendre expliquer un algorithme au-delà des informations réellement émises par celui-ci.

### 14.3 Audit Event

Les connexions, créations et modifications de mission, commandes, changements d'algorithme, configurations et rôles sont audités. L'événement contient utilisateur, rôle, action, cible, paramètres filtrés, mission, résultat, raison, adresse de contexte si autorisée et corrélation. Les secrets et tokens ne sont jamais journalisés.

## 15 Architecture des algorithmes

Le modèle public contient 15 algorithmes hébergés par 11 composants. Les chaînes de sûreté restent Onboard. La coordination dépend du lien et peut se dégrader. Le catalogue comprend notamment allocation CBBA, consensus, contrôle de formation, élection de leader, fusion EKF ou UKF, navigation GNSS dégradée, évitement de collision, règles de sûreté, planification de chemin, diagnostic et gestion énergétique.

Chaque fiche algorithmique doit contenir identité, version, catégorie, objectif, composant hôte, runtime, entrées, sorties, paramètres, invariants, modes dégradés, complexité, budget, données de validation, spécification et publications. Un changement de paramètres en mission est interdit par défaut et nécessite une politique explicite.

### 15.1 Connaissance scientifique

Les sources scientifiques soutiennent ou contredisent une affirmation précise. Le produit ne doit pas transformer une publication en certification du système. Il doit exposer population expérimentale, environnement, métrique, résultat, limite et niveau de transférabilité. Le modèle public signale notamment que la preuve d'échelle à 30 plateformes réelles n'est pas établie et qu'une affirmation sur les Control Barrier Functions est classée non supportée. Ces limites doivent rester visibles dans les vues concernées.

## 16 Runtime mapping et dérive d'architecture

Le mapping associe un `likec4Id` à une ou plusieurs instances observées avec version, configuration, hôte, runtime, état, dernière observation et confiance de découverte. Le moteur de dérive compare attendu et observé et produit : composant manquant, composant inattendu, version incorrecte, relation absente, topic ou service manquant, algorithme différent, configuration divergente ou mapping ambigu.

Une dérive n'est pas automatiquement une panne. Elle doit être classée comme autorisée, temporaire, non conforme ou inconnue. L'opérateur peut l'acquitter, mais seule une mise à jour de LikeC4 ou du runtime la résout.

## 17 World Model et représentation

Le World Model publié contient des entités simulables, des zones sémantiques et des catégories de scène. Il matérialise 18 UAV-R, 8 UAV-F, 4 USV, une maquette de référence distincte, un nœud Edge, un poste C2 et un jumeau numérique. Les zones Air, Surface, Allocation, Communications, Commandement et Cloud sont abstraites et sans coordonnées.

Le consommateur 2D ou 3D fournit spatialisation, cinématique, horloge, interpolation, bruit capteur, turbulence et modèle de canal. Il ne peut pas attribuer une portée radio, une capacité batterie ou une fidélité capteur lorsque le contrat ne la fournit pas. Toute valeur substituée doit être un paramètre de scénario identifié.

## 18 Sécurité et gouvernance

### 18.1 Identité et rôles

L'authentification utilise OIDC ou OAuth2. Les rôles initiaux sont Viewer, Operator, Mission Admin et System Admin. Les permissions sont complétées par attributs de mission, zone, plateforme et type de commande. Le principe du moindre privilège s'applique.

### 18.2 Contrôle des actions

| Action | Viewer | Operator | Mission Admin | System Admin |
|---|---:|---:|---:|---:|
| Visualiser et rejouer | Oui | Oui | Oui | Oui |
| Acquitter une alerte | Non | Oui | Oui | Oui |
| Émettre une commande mission | Non | Oui | Oui | Selon délégation |
| Modifier une mission | Non | Non | Oui | Selon délégation |
| Modifier un paramètre algorithmique | Non | Non | Politique dédiée | Politique dédiée |
| Gérer rôles et intégrations | Non | Non | Non | Oui |

### 18.3 Réseau et secrets

Le navigateur utilise TLS vers le Gateway. Les services utilisent mTLS lorsque le déploiement l'exige. Les ordres critiques sont signés. Les clés sont rotatives, révocables et distinctes par environnement. Les technologies ROS 2, DDS, Zenoh, MQTT et MAVLink doivent être configurées selon un modèle de menace approuvé. La décision ouverte DE-03 reste bloquante pour l'homologation.

### 18.4 Licences

Le produit doit maintenir un inventaire SBOM et le statut des licences. PX4 BSD-3 et ArduPilot GPLv3 imposent un arbitrage de socle. MinIO et Grafana portent des obligations AGPL à faire valider. Aucun choix d'implémentation ne doit être considéré approuvé par sa seule présence dans LikeC4.

## 19 Observabilité et exploitation

Chaque service expose santé, readiness, version et métriques. Les logs sont structurés, corrélés et horodatés. Les métriques incluent taux, perte et latence des messages, backlog, latence et échecs de commande, plateformes connectées ou stale, sessions WebSocket, retard du Digital Twin, temps de resynchronisation, dérives d'architecture et files de priorité.

Les tableaux d'exploitation distinguent santé du produit, santé du runtime robotique et état de mission. Une indisponibilité du dashboard de métriques ne doit pas altérer le traitement opérationnel.

## 20 Performance et capacité

| ID | Indicateur | Cible nominale |
|---|---|---:|
| SCC-NFR-001 | Plateformes nominales | 30 |
| SCC-NFR-002 | Cible d'architecture | 100 ou plus |
| SCC-NFR-003 | Stress | 200 |
| SCC-NFR-004 | Rendu cible | 60 FPS |
| SCC-NFR-005 | FPS p95 minimum | 50 |
| SCC-NFR-006 | Réponse interaction locale p95 | moins de 100 ms |
| SCC-NFR-007 | Ouverture panneau p95 | moins de 150 ms |
| SCC-NFR-008 | Télémétrie LAN bout en bout p95 | moins de 250 ms |
| SCC-NFR-009 | Freeze supérieur à 100 ms en nominal | 0 |
| SCC-NFR-010 | Resynchronisation après coupure de 30 s | moins de 5 s sur réseau de référence |

Ces valeurs sont des objectifs jusqu'à mesure sur une configuration de référence documentée. Les tests doivent enregistrer matériel, navigateur, résolution, versions, débit, latence, perte, dataset et options de rendu.

Le rendu utilise instancing, LOD, pooling, frustum culling, picking GPU et batching lorsque nécessaire. La lisibilité à 30 plateformes prévaut sur une optimisation qui dégrade l'information.

## 21 Disponibilité et résilience

Le système doit tolérer perte, latence, jitter, duplicata, désordre, redémarrage backend, reconnexion navigateur, télémétrie partielle, état stale, déconnexion et partition. Un redémarrage reconstruit le Digital Twin depuis snapshot et événements sans réémettre les commandes. Les consommateurs doivent être idempotents.

Les objectifs RPO et RTO doivent être fixés par environnement. En l'absence de décision, la baseline proposée est RPO nul pour commandes et audit, RPO inférieur à 5 secondes pour événements opérationnels, et RTO inférieur à 5 minutes pour le C2, sans incidence sur la sûreté Onboard.

## 22 Accessibilité et design system

Le design system comprend MetricCard, StatusBadge, PlatformCard, GroupCard, AlertCard, TimelineEvent, CommandStatus, LayerControl, MissionTree, HealthIndicator, ConnectivityIndicator, AlgorithmBadge et DecisionTracePanel. Chaque composant traite normal, sélectionné, warning, critical, disabled, stale et unknown.

Les surfaces critiques sont opaques et fortement contrastées. Le glassmorphism est réservé au décoratif. L'information critique n'est jamais transmise par la couleur seule. La navigation clavier, les focus visibles, le mouvement réduit, le contraste et les tailles lisibles sont obligatoires. L'objectif est WCAG 2.2 niveau AA pour toutes les fonctions non dépendantes de la scène spatiale, avec alternatives tabulaires pour les informations de la scène.

## 23 Validation et scénarios d'acceptation

### 23.1 Scénario principal

Une mission avec 30 plateformes et trois groupes est active. Les liens de UAV-17, UAV-19 et UAV-23 se dégradent. Le produit doit mettre à jour la santé réseau, créer un incident, identifier le groupe affecté, afficher les plateformes et le graphe, conserver les événements, exposer une Decision Trace si un algorithme réagit, permettre une action autorisée, suivre l'accusé et le résultat, puis rejouer la séquence complète.

### 23.2 Catalogue minimal

| ID | Scénario | Preuve attendue |
|---|---|---|
| SCC-VAL-001 | Nominal 30 plateformes | Continuité, lisibilité et performance |
| SCC-VAL-002 | Perte réseau 5, 10 et 20 pour cent | Fraîcheur, priorisation et incident |
| SCC-VAL-003 | Latence 50 ms, 200 ms, 500 ms et 2 s | États aging et stale sans fausse panne |
| SCC-VAL-004 | Déconnexion d'une puis plusieurs plateformes | Différenciation silence, perte et défaillance |
| SCC-VAL-005 | Partition en deux clusters | Consensus éventuel, vues de partition et absence de cohérence fictive |
| SCC-VAL-006 | Réallocation après perte d'un agent | Trace de décision et historique d'assignation |
| SCC-VAL-007 | Redémarrage backend | Reconstruction sans duplicata de commande |
| SCC-VAL-008 | Rejeu | Déterminisme logique et synchronisation UI |
| SCC-VAL-009 | Mise à jour LikeC4 | Delta et conservation des identifiants |
| SCC-VAL-010 | Perte du leader à N égal 30 | Résultat simulé clairement séparé d'une preuve réelle |
| SCC-VAL-011 | C2 et Cloud indisponibles | Mission Onboard non bloquée |
| SCC-VAL-012 | GNSS dégradé et lien C2 perdu | Incertitude position, coordination locale et éventuel RTL local |
| SCC-VAL-013 | Tempête d'alertes | Corrélation et charge cognitive maîtrisée |
| SCC-VAL-014 | Saturation radio | Priorité sûreté respectée |
| SCC-VAL-015 | Commande critique non autorisée | Rejet, audit et absence d'émission |

### 23.3 Paliers de preuve

La validation suit les paliers 2, 5, 12 et 30 plateformes. Le réel, le SIL, le HIL et le jumeau numérique sont enregistrés séparément. Aucun résultat SIL ou HIL ne peut être résumé comme validation en vol réel. La décision DE-02 doit fixer le plafond réaliste de preuve physique.

## 24 Risques et angles morts prioritaires

### 24.1 Risques critiques du modèle

| Risque | Effet sur le Command Center | Traitement produit |
|---|---|---|
| R-1 Contention radio à 30 agents | États périmés et perte de messages | Backpressure, priorité, métriques et budget radio |
| R-2 Prise de contrôle via lien non authentifié | Commande hostile | Signature, mTLS ou profil approuvé, autorisation et audit |
| R-3 Validation réelle à 30 irréalisable | Faux niveau de confiance | Séparation stricte des paliers de preuve |
| R-4 Perte de cohérence sous partition | Vue globale trompeuse | Affichage des partitions, versions et convergence éventuelle |
| R-5 Dérive en GNSS dégradé | Position faussement précise | Source, covariance, confiance et âge |

### 24.2 Angles morts à intégrer au backlog

Les angles morts majeurs concernent licences, exposition AGPL, budget radio, perception d'essaim, validation d'échelle, synchronisation d'horloge, cybersécurité, réglementation BVLOS et COLREGs, énergie hétérogène, handover UAV-USV, ontologie commune, agent défaillant, observabilité et rejeu, bruit et fausses détections, fidélité du jumeau et charge humaine C2. Chacun doit posséder un propriétaire, une décision attendue, une échéance et un critère de fermeture.

## 25 Décisions ouvertes

| ID | Décision | Options ou sortie attendue | Échéance recommandée |
|---|---|---|---|
| DE-SCC-01 | Normaliser UAV-R et UAV-F | Dictionnaire de types et migration LikeC4 | Avant maquette UI |
| DE-01 | Socle autopilote et licence | PX4, ArduPilot ou stratégie par domaine | Avant HIL |
| DE-02 | Plafond de validation réelle | Répartition réel, SIL, HIL et simulation | Avant plan de V&V |
| DE-03 | Modèle de menace et clés | PKI, sécurité graduée, séparation des canaux | Avant commande réelle |
| DE-04 | Architecture radio à 30 | Mesh, étoile Edge, hybride ou multi-radio | Après budget radio |
| DE-SCC-02 | Objectifs RPO et RTO | Valeurs par environnement | Avant production |
| DE-SCC-03 | Stockage opérationnel | PostgreSQL, TimescaleDB, objet et bus | Avant phase temporelle |
| DE-SCC-04 | Politique de données sensibles | Classification, rétention et export | Avant collecte terrain |
| DE-SCC-05 | Cadre réglementaire | BVLOS, maritime, données et export | Avant essai opérationnel |

## 26 Roadmap de mise en œuvre

### Phase 0 Décisions et contrats

Fermer la taxonomie de flotte, figer les identifiants LikeC4, décider le modèle de menace, produire le budget radio, définir les seuils de fraîcheur et publier les schémas de messages.

### Phase 1 Fondations

Implémenter le Canonical Architecture Graph, les schémas du Digital Twin, l'enveloppe événementielle, le transport abstrait, l'identité, l'audit et le registre de missions.

### Phase 2 MVP opérationnel

Livrer shell React, vue 2D, scène 3D initiale, 30 plateformes, hiérarchie Swarm Group Platform, mission, tâches, fraîcheur, alertes, graphe réseau et commandes non critiques.

### Phase 3 Jumeau temporel

Livrer snapshots, Event Store, timeline, rejeu déterministe, reconstruction après redémarrage, signets et export d'incident.

### Phase 4 Intelligence explicable

Livrer Decision Trace, catalogue algorithmique, vues d'allocation, réseau, prédiction et comparaison baseline variante.

### Phase 5 Architecture vivante

Livrer import LikeC4, explorer, runtime mapping, drift detection, delta engine, documents et navigation vers les preuves scientifiques.

### Phase 6 Validation avancée

Livrer campagnes SIL et HIL, stress 100 et 200, scénarios de partition, facteurs humains, protocole H-Zip observable et qualification des objectifs EXTRAP.

## 27 Gouvernance de livraison

Une fonctionnalité est terminée lorsque son schéma est versionné, les erreurs et états stale sont gérés, les événements sont émis, l'audit est présent si nécessaire, les tests passent, les performances sont mesurées, la sécurité est revue, LikeC4 est mis à jour, les documents sont liés et les critères d'accessibilité sont satisfaits.

Chaque évolution suit la boucle `Spécifier → Implémenter → Observer → Mesurer → Challenger → Corriger → Tester la régression → Documenter`. Les revues obligatoires couvrent fonction, UX, performance, réseau dégradé, explicabilité, temporalité, sécurité et alignement architectural.

## 28 Matrice de traçabilité de synthèse

| Besoin | Source LikeC4 | Exigences principales | Validation |
|---|---|---|---|
| Mission sans C2 ni Cloud | INV-1, ADR-1, H1, H2 | SCC-FR-010, 034, 050 | SCC-VAL-011 |
| Sûreté locale | INV-2, ADR-3, `onboard.safety` | SCC-FR-031, 034 | SCC-VAL-012, 015 |
| Fraîcheur explicite | ADR-2, `onboard.telemetry` | SCC-FR-005, SCC-DATA temporel | SCC-VAL-002, 003 |
| Coordination sous partition | ADR-5, `edge.coordinator` | SCC-FR-003, 022 | SCC-VAL-005 |
| Cloud non bloquant | ADR-4, `cloud.*` | SCC-FR-040, 050 | SCC-VAL-007, 011 |
| Architecture vivante | LikeC4, runtime mapping | SCC-FR-060 à 064 | SCC-VAL-009 |
| Preuve scientifique honnête | INV-5, findings | SCC-FR-063, 064 | Revue de contenu |
| Autorité humaine | H5, `c2.auth`, `c2.gcs` | SCC-FR-030 à 033 | SCC-VAL-015 |

## 29 Architecture cible consolidée

```text
Opérateur
   |
   v
Swarm Command Center
   |-- Situation 2D et 3D
   |-- Mission et tâches
   |-- Alertes et commandes
   |-- Timeline et rejeu
   |-- Architecture et science
   |
   v
Mission Gateway et services C2
   |-- Identité et autorisation
   |-- Planner et Mission Store
   |-- Digital Twin et Event Store
   |-- Architecture Graph et Drift Engine
   |
   v
Edge 1..N
   |-- Gateway de confiance
   |-- Relay et Cache
   |-- Fusion et Coordinator
   |
   v
Onboard x30
   |-- Mission, Task Auction et Telemetry
   |-- Perception, Health et Energy
   |-- Safety et Autopilot

Cloud différé
   |-- Replay Store
   |-- Twin et Analytics
   |-- Model Registry

LikeC4
   |-- Architecture attendue
   |-- Décisions, hypothèses et invariants
   |-- Algorithmes, messages et preuves
   `-- Export vers le graphe canonique
```

## 30 Conclusion de conception

Le Swarm Command Center doit être jugé sur sa capacité à préserver les distinctions qui rendent l'essaim compréhensible et sûr. L'attendu architectural ne doit pas être confondu avec l'observé. Une perte de lien ne doit pas devenir automatiquement une panne. Une prédiction ne doit pas devenir un fait. Une simulation ne doit pas devenir une preuve réelle. Une publication ne doit pas devenir une certification. Une interface ne doit pas devenir une boucle de contrôle.

La réussite du produit se mesure lorsque l'opérateur peut passer d'un symptôme global à un groupe, une plateforme, une tâche, une décision, un composant, une spécification et une source, tout en conservant le temps, la provenance, l'incertitude et l'autorité nécessaires à l'action.

## Annexe A Index des exigences

- `SCC-FR-001 à 005` Situation opérationnelle
- `SCC-FR-010 à 014` Mission et tâches
- `SCC-FR-020 à 024` Alertes
- `SCC-FR-030 à 034` Commandes
- `SCC-FR-040 à 043` Historique et rejeu
- `SCC-FR-050 à 054` Simulation
- `SCC-FR-060 à 064` Architecture et science
- `SCC-UX-001 à 006` Facteurs humains
- `SCC-NFR-001 à 010` Performance et capacité
- `SCC-VAL-001 à 015` Validation

## Annexe B Sources

- Spécification détaillée consolidée V2.0 fournie avec la demande, 22 septembre 2026.
- Modèle SwarmDrones LikeC4 public, `https://likec4.breizh.ai`, consultation du 22 septembre 2026.
- LikeC4, Introduction du DSL, `https://likec4.dev/dsl/intro/`.
- LikeC4, Modèle logique, `https://likec4.dev/dsl/model/`.
- LikeC4, Modèle de déploiement, `https://likec4.dev/dsl/deployment/model/`.

