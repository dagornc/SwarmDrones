#!/usr/bin/env python3
"""Simulateur de reference pour ALG_CONSENSUS (Specification_ALG_CONSENSUS_v5).

Implemente le harnais decrit en section 4.7.1 :
  - topologie grille 2D 6x5 (N=30), fanout f=6
  - perte i.i.d. par message ; latence configurable (parametre `latence`,
    defaut 0 = arrivee dans la meme periode)
  - horloges logiques par agent
  - pseudo-code du Bloc 5 (gossip de digests d'etat, fusion semi-treillis)
  - mesure du temps de re-fusion apres partition (test T5D, divergence forcee)

Usage :
    python3 sim_consensus.py --test T1 --seeds 1001-1100 --out resultats.csv
    python3 sim_consensus.py --test T5D --seeds 1001-1100 --out refusion.csv
    python3 sim_consensus.py --all --out resultats.csv

Sortie : CSV (test, seed, periode_convergence, accord, messages, taille_etat).
"""
import argparse
import csv
import random
import statistics
import sys

T_G = 1.0          # periode de gossip (s)
GRID_W, GRID_H = 6, 5
N = GRID_W * GRID_H
FANOUT = 6
LATENCE_MAX = 2 * T_G


def voisins(i, w=GRID_W, h=GRID_H):
    """Voisinage 4-connexe sur grille w x h."""
    x, y = i % w, i // w
    out = []
    if x > 0:
        out.append(i - 1)
    if x < w - 1:
        out.append(i + 1)
    if y > 0:
        out.append(i - w)
    if y < h - 1:
        out.append(i + w)
    return out


def fusion(a, b):
    """Fusion semi-treillis : max sur (valeur, horloge logique)."""
    if a is None:
        return b
    if b is None:
        return a
    return max(a, b)


def simuler(seed, perte=0.0, duplication=1, retrait=0, partition=False,
            max_periodes=200, latence=0, t_retrait=None, forcer_divergence=False):
    """Une execution. Retourne (periode_convergence, accord, messages, taille_etat,
    periode_refusion, divergence_reelle).

    latence : nombre de periodes de retard (0 = arrivee dans la meme periode).
    t_retrait : periode du retrait des agents (None = a l'initialisation).
    forcer_divergence : si True, la partition est construite pour que le maximum
        global ne soit present que dans une seule moitie (la re-fusion est alors
        reellement exercee).
    """
    rng = random.Random(seed)
    # etat initial : chaque agent a une valeur propre (valeur, horloge)
    etat = [(rng.randint(0, 9), 0) for _ in range(N)]
    actifs = set(range(N))
    if retrait and t_retrait is None:
        for i in rng.sample(range(N), min(retrait, N - 1)):
            actifs.discard(i)

    # partition : moitie gauche / moitie droite (colonnes)
    def cote(i):
        return i % GRID_W < GRID_W // 2

    if forcer_divergence and partition:
        # garantir que le maximum global n'est present que dans la moitie gauche :
        # les agents de droite sont plafonnes sous le max de gauche.
        max_gauche = max(etat[i][0] for i in range(N) if cote(i))
        for i in range(N):
            if not cote(i):
                etat[i] = (min(etat[i][0], max_gauche - 1), 0)

    messages = 0
    periode_conv = None
    periode_refusion = None
    fin_partition = max_periodes // 2
    # file d'attente par agent : liste de (periode_arrivee, valeur)
    en_vol = [[] for _ in range(N)]
    divergence_reelle = False
    for t in range(1, max_periodes + 1):
        if retrait and t_retrait is not None and t == t_retrait:
            for i in rng.sample(sorted(actifs), min(retrait, len(actifs) - 1)):
                actifs.discard(i)
        # livrer les messages arrives a cette periode
        for d in range(N):
            for (ta, v) in en_vol[d]:
                if ta <= t:
                    etat[d] = fusion(etat[d], v)
            en_vol[d] = [(ta, v) for (ta, v) in en_vol[d] if ta > t]
        nouveaux = {}
        for i in sorted(actifs):
            cibles = voisins(i)
            if partition and t <= fin_partition:
                # coupure : moitie gauche / moitie droite
                cibles = [c for c in cibles if cote(c) == cote(i)]
            if not cibles:
                continue
            dests = rng.sample(cibles, min(FANOUT, len(cibles)))
            for d in dests:
                for _ in range(duplication):
                    messages += 1
                    if rng.random() < perte:
                        continue  # message perdu
                    if latence:
                        en_vol[d].append((t + latence, etat[i]))
                    else:
                        nouveaux[d] = fusion(nouveaux.get(d), etat[i])
        for d, v in nouveaux.items():
            etat[d] = fusion(etat[d], v)
        # divergence reelle : a la FIN de la partition, les deux moities ont
        # converge vers des valeurs DIFFERENTES (et non une difference transitoire)
        if partition and t == fin_partition:
            g = {etat[i][0] for i in actifs if cote(i)}
            dr = {etat[i][0] for i in actifs if not cote(i)}
            if g and dr and len(g) == 1 and len(dr) == 1 and g != dr:
                divergence_reelle = True
        # convergence : tous les agents actifs partagent la meme valeur
        vals = {etat[i][0] for i in actifs}
        if len(vals) == 1 and periode_conv is None:
            periode_conv = t
            if partition and divergence_reelle and t > fin_partition:
                periode_refusion = t - fin_partition
            break

    vals = {etat[i][0] for i in actifs}
    accord = len(vals) == 1
    taille_etat = len(actifs)
    return periode_conv, accord, messages, taille_etat, periode_refusion, divergence_reelle


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--test', default='T1')
    ap.add_argument('--seeds', default='1001-1100')
    ap.add_argument('--all', action='store_true')
    ap.add_argument('--out', default='resultats.csv')
    args = ap.parse_args()

    lo, hi = (int(x) for x in args.seeds.split('-'))
    seeds = list(range(lo, hi + 1))

    # parametres par test (cf. tableau 4.7.1)
    params = {
        'T1': dict(perte=0.0),
        'T2': dict(perte=0.0),
        'T3': dict(perte=0.25),
        'T4': dict(perte=0.0, duplication=2),
        'T5': dict(perte=0.0, partition=True),
        'T5D': dict(perte=0.0, partition=True, forcer_divergence=True),
        'T6': dict(perte=0.0),
        'T8': dict(perte=0.0, retrait=3, t_retrait=3),
        'T9': dict(perte=0.0, partition=True),
    }
    tests = list(params) if args.all else [args.test]

    lignes = []
    for test in tests:
        p = params.get(test, dict(perte=0.0))
        for s in seeds:
            conv, accord, msg, taille, refus, div = simuler(s, **p)
            lignes.append([test, s, conv if conv else '', int(accord), msg,
                           taille, refus if refus else '', int(div)])

    with open(args.out, 'w', newline='') as f:
        w = csv.writer(f)
        w.writerow(['test', 'seed', 'periode_convergence', 'accord',
                    'messages', 'taille_etat', 'periode_refusion',
                    'divergence_reelle'])
        w.writerows(lignes)

    # resume par test
    for test in tests:
        sub = [l for l in lignes if l[0] == test]
        convs = [l[2] for l in sub if l[2] != '']
        accords = sum(l[3] for l in sub)
        n = len(sub)
        med = statistics.median(convs) if convs else float('nan')
        refus = [l[6] for l in sub if l[6] != '']
        ndiv = sum(l[7] for l in sub)
        extra = ''
        if refus:
            extra = (f' refusion_med={statistics.median(refus)} '
                     f'refusion_max={max(refus)} graines_divergentes={ndiv}/{n}')
        print(f'{test}: n={n} accord={accords}/{n} '
              f'mediane_conv={med} messages_moy={sum(l[4] for l in sub)/n:.1f}{extra}')
    print(f'-> {args.out}')


if __name__ == '__main__':
    sys.exit(main())
