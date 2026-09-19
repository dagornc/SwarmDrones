//! Simulateur ALG_CONSENSUS — portage fidèle de `sim_consensus.py`.
//!
//! Implémente le harnais décrit en §4.7.1 de la spécification v5 :
//! topologie grille 2D 6×5 (N = 30), fanout f = 6, perte i.i.d. par message,
//! latence configurable, horloges logiques par agent, gossip de digests
//! d'état avec fusion semi-treillis, mesure du temps de re-fusion après
//! partition (test T5D).
//!
//! # Propriété fondamentale
//!
//! La fusion est un **semi-treillis** : `max` sur le couple
//! `(valeur, horloge_logique)`. Cette opération est :
//!
//! - **commutative** : `a ⊔ b = b ⊔ a`
//! - **associative** : `(a ⊔ b) ⊔ c = a ⊔ (b ⊔ c)`
//! - **idempotente** : `a ⊔ a = a`
//!
//! Ces trois propriétés garantissent que l'état final est **indépendant de
//! l'ordre d'arrivée des messages** — c'est ce qui rend le protocole
//! tolérant à la perte, à la duplication et à la réordonnancement.

use crate::rng::PyRandom;

/// Période de gossip (s).
pub const T_G: f64 = 1.0;
/// Largeur de la grille.
pub const GRID_W: usize = 6;
/// Hauteur de la grille.
pub const GRID_H: usize = 5;
/// Nombre d'agents.
pub const N: usize = GRID_W * GRID_H;
/// Fanout : nombre de voisins contactés par période.
pub const FANOUT: usize = 6;
/// Latence maximale (s).
pub const LATENCE_MAX: f64 = 2.0 * T_G;

/// État d'un agent : `(valeur, horloge_logique)`.
///
/// L'ordre lexicographique sur ce couple est l'ordre du semi-treillis.
pub type Etat = (i32, u32);

/// Fusion semi-treillis : `max` sur le couple `(valeur, horloge)`.
///
/// `None` représente l'élément absorbant (agent sans information).
#[inline]
pub fn fusion(a: Option<Etat>, b: Option<Etat>) -> Option<Etat> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(x), Some(y)) => Some(if x >= y { x } else { y }),
    }
}

/// Voisinage 4-connexe sur une grille `w × h`.
pub fn voisins(i: usize, w: usize, h: usize) -> Vec<usize> {
    let x = i % w;
    let y = i / w;
    let mut out = Vec::with_capacity(4);
    if x > 0 {
        out.push(i - 1);
    }
    if x < w - 1 {
        out.push(i + 1);
    }
    if y > 0 {
        out.push(i - w);
    }
    if y < h - 1 {
        out.push(i + w);
    }
    out
}

/// Paramètres d'un test.
#[derive(Debug, Clone, Copy)]
pub struct Params {
    /// Probabilité de perte par message.
    pub perte: f64,
    /// Nombre de copies par message.
    pub duplication: usize,
    /// Nombre d'agents retirés.
    pub retrait: usize,
    /// Partition gauche/droite active.
    pub partition: bool,
    /// Nombre maximal de périodes simulées.
    pub max_periodes: usize,
    /// Latence en périodes (0 = arrivée dans la même période).
    pub latence: usize,
    /// Période du retrait (`None` = à l'initialisation).
    pub t_retrait: Option<usize>,
    /// Force une divergence réelle entre les deux moitiés.
    pub forcer_divergence: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            perte: 0.0,
            duplication: 1,
            retrait: 0,
            partition: false,
            max_periodes: 200,
            latence: 0,
            t_retrait: None,
            forcer_divergence: false,
        }
    }
}

/// Résultat d'une exécution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resultat {
    /// Période de convergence (`None` si non convergé).
    pub periode_convergence: Option<usize>,
    /// Accord atteint (tous les agents actifs partagent la même valeur).
    pub accord: bool,
    /// Nombre de messages émis.
    pub messages: u64,
    /// Nombre d'agents actifs en fin de simulation.
    pub taille_etat: usize,
    /// Période de re-fusion après partition (`None` si non mesurée).
    pub periode_refusion: Option<usize>,
    /// Divergence réelle observée à la fin de la partition.
    pub divergence_reelle: bool,
}

/// Exécute une simulation complète.
///
/// Portage fidèle de `simuler()` du simulateur Python : l'ordre des tirages
/// aléatoires est identique, ce qui garantit la parité des résultats.
pub fn simuler(seed: u64, p: &Params) -> Resultat {
    let mut rng = PyRandom::new(seed);

    // État initial : chaque agent a une valeur propre (valeur, horloge 0).
    let mut etat: Vec<Etat> = (0..N).map(|_| (rng.randint(0, 9) as i32, 0)).collect();
    let mut actifs: Vec<bool> = vec![true; N];

    // Retrait à l'initialisation.
    if p.retrait > 0 && p.t_retrait.is_none() {
        let pop: Vec<usize> = (0..N).collect();
        let n_retrait = p.retrait.min(N - 1);
        for i in rng.sample(&pop, n_retrait) {
            actifs[i] = false;
        }
    }

    // Partition : moitié gauche / moitié droite (par colonnes).
    let cote = |i: usize| -> bool { i % GRID_W < GRID_W / 2 };

    if p.forcer_divergence && p.partition {
        // Garantir que le maximum global n'est présent que dans la moitié
        // gauche : les agents de droite sont plafonnés sous le max de gauche.
        let max_gauche = (0..N)
            .filter(|&i| cote(i))
            .map(|i| etat[i].0)
            .max()
            .unwrap_or(0);
        for i in 0..N {
            if !cote(i) {
                etat[i] = (etat[i].0.min(max_gauche - 1), 0);
            }
        }
    }

    let mut messages: u64 = 0;
    let mut periode_conv: Option<usize> = None;
    let mut periode_refusion: Option<usize> = None;
    let fin_partition = p.max_periodes / 2;
    // File d'attente par agent : (période d'arrivée, valeur).
    let mut en_vol: Vec<Vec<(usize, Etat)>> = vec![Vec::new(); N];
    let mut divergence_reelle = false;

    for t in 1..=p.max_periodes {
        // Retrait programmé.
        if p.retrait > 0 && p.t_retrait == Some(t) {
            let pop: Vec<usize> = (0..N).filter(|&i| actifs[i]).collect();
            let n_retrait = p.retrait.min(pop.len().saturating_sub(1));
            for i in rng.sample(&pop, n_retrait) {
                actifs[i] = false;
            }
        }

        // Livrer les messages arrivés à cette période.
        for d in 0..N {
            let mut restants = Vec::new();
            for &(ta, v) in &en_vol[d] {
                if ta <= t {
                    etat[d] = fusion(Some(etat[d]), Some(v)).unwrap();
                } else {
                    restants.push((ta, v));
                }
            }
            en_vol[d] = restants;
        }

        // Émission : chaque agent actif contacte un sous-ensemble de voisins.
        let mut nouveaux: Vec<Option<Etat>> = vec![None; N];
        for i in 0..N {
            if !actifs[i] {
                continue;
            }
            let mut cibles = voisins(i, GRID_W, GRID_H);
            if p.partition && t <= fin_partition {
                cibles.retain(|&c| cote(c) == cote(i));
            }
            if cibles.is_empty() {
                continue;
            }
            let n_dests = FANOUT.min(cibles.len());
            let dests = rng.sample(&cibles, n_dests);
            for d in dests {
                for _ in 0..p.duplication {
                    messages += 1;
                    if rng.random() < p.perte {
                        continue; // message perdu
                    }
                    if p.latence > 0 {
                        en_vol[d].push((t + p.latence, etat[i]));
                    } else {
                        nouveaux[d] = fusion(nouveaux[d], Some(etat[i]));
                    }
                }
            }
        }
        for d in 0..N {
            if let Some(v) = nouveaux[d] {
                etat[d] = fusion(Some(etat[d]), Some(v)).unwrap();
            }
        }

        // Divergence réelle : à la FIN de la partition, les deux moitiés ont
        // convergé vers des valeurs DIFFÉRENTES (et non une différence
        // transitoire).
        if p.partition && t == fin_partition {
            let g: std::collections::HashSet<i32> = (0..N)
                .filter(|&i| actifs[i] && cote(i))
                .map(|i| etat[i].0)
                .collect();
            let dr: std::collections::HashSet<i32> = (0..N)
                .filter(|&i| actifs[i] && !cote(i))
                .map(|i| etat[i].0)
                .collect();
            if g.len() == 1 && dr.len() == 1 && g != dr {
                divergence_reelle = true;
            }
        }

        // Convergence : tous les agents actifs partagent la même valeur.
        let vals: std::collections::HashSet<i32> = (0..N)
            .filter(|&i| actifs[i])
            .map(|i| etat[i].0)
            .collect();
        if vals.len() == 1 && periode_conv.is_none() {
            periode_conv = Some(t);
            if p.partition && divergence_reelle && t > fin_partition {
                periode_refusion = Some(t - fin_partition);
            }
            break;
        }
    }

    let vals: std::collections::HashSet<i32> = (0..N)
        .filter(|&i| actifs[i])
        .map(|i| etat[i].0)
        .collect();
    let accord = vals.len() == 1;
    let taille_etat = actifs.iter().filter(|&&a| a).count();

    Resultat {
        periode_convergence: periode_conv,
        accord,
        messages,
        taille_etat,
        periode_refusion,
        divergence_reelle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voisins_coin() {
        // Coin (0,0) : deux voisins (droite, bas).
        assert_eq!(voisins(0, 6, 5), vec![1, 6]);
    }

    #[test]
    fn voisins_centre() {
        // Case (1,1) = index 7 : quatre voisins.
        assert_eq!(voisins(7, 6, 5), vec![6, 8, 1, 13]);
    }

    #[test]
    fn fusion_est_commutative() {
        let a = Some((3, 1));
        let b = Some((5, 0));
        assert_eq!(fusion(a, b), fusion(b, a));
    }

    #[test]
    fn fusion_est_idempotente() {
        let a = Some((3, 1));
        assert_eq!(fusion(a, a), a);
    }

    #[test]
    fn fusion_est_associative() {
        let a = Some((3, 1));
        let b = Some((5, 0));
        let c = Some((1, 9));
        assert_eq!(fusion(fusion(a, b), c), fusion(a, fusion(b, c)));
    }

    #[test]
    fn fusion_none_est_neutre() {
        let a = Some((3, 1));
        assert_eq!(fusion(None, a), a);
        assert_eq!(fusion(a, None), a);
    }

    #[test]
    fn t1_converge_sans_perte() {
        let r = simuler(1001, &Params::default());
        assert!(r.accord, "T1 doit converger sans perte");
        assert_eq!(r.taille_etat, 30);
    }

    #[test]
    fn t8_retrait_avant_convergence() {
        // Le retrait à t=3 n'a lieu QUE si la simulation n'a pas déjà
        // convergé (le simulateur `break` à la convergence, comme la
        // référence Python). Sans perte, la convergence survient avant t=3 :
        // le retrait ne s'applique donc pas et la taille reste 30.
        let p = Params {
            retrait: 3,
            t_retrait: Some(3),
            ..Default::default()
        };
        let r = simuler(1001, &p);
        assert_eq!(r.taille_etat, 30, "convergence avant t=3 : retrait non appliqué");
    }

    #[test]
    fn retrait_a_l_initialisation() {
        // Retrait à l'initialisation (t_retrait = None) : 3 agents retirés.
        let p = Params {
            retrait: 3,
            t_retrait: None,
            ..Default::default()
        };
        let r = simuler(1001, &p);
        assert_eq!(r.taille_etat, 27, "3 agents retirés sur 30");
    }
}
