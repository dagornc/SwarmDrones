#!/usr/bin/env bash
# =============================================================================
# demo.sh — Démonstration reproductible de la chaîne SWARM-3D
# =============================================================================
# Clone la solution complète, compile et teste les algorithmes Rust, vérifie
# la parité bit-à-bit, et affiche les points d'entrée publics.
#
# Usage :
#   ./demo.sh              # démo complète (clone + build + tests + parité)
#   ./demo.sh --quick      # un seul algorithme (formation-control)
#   ./demo.sh --no-clone   # suppose SwarmDrones déjà cloné dans ./SwarmDrones
#
# Prérequis : git, cargo (Rust), python3.
# =============================================================================
set -euo pipefail

QUICK=0
NO_CLONE=0
for arg in "$@"; do
  case "$arg" in
    --quick) QUICK=1 ;;
    --no-clone) NO_CLONE=1 ;;
    -h|--help) sed -n '2,14p' "$0"; exit 0 ;;
  esac
done

ROOT="$(pwd)"
REPO_DIR="$ROOT/SwarmDrones"
SITE="https://likec4.breizh.ai"

say()  { printf '\n\033[1;36m==> %s\033[0m\n' "$*"; }
ok()   { printf '    \033[1;32m✓\033[0m %s\n' "$*"; }
warn() { printf '    \033[1;33m!\033[0m %s\n' "$*"; }

# --- Prérequis ---------------------------------------------------------------
say "Vérification des prérequis"
for tool in git python3; do
  command -v "$tool" >/dev/null 2>&1 && ok "$tool présent" || { warn "$tool manquant"; exit 1; }
done
if command -v cargo >/dev/null 2>&1; then
  ok "cargo présent ($(cargo --version))"
  HAVE_CARGO=1
else
  warn "cargo absent — les étapes Rust seront ignorées"
  HAVE_CARGO=0
fi

# --- Clone -------------------------------------------------------------------
if [ "$NO_CLONE" -eq 0 ]; then
  say "Clonage de la solution complète (submodules épinglés)"
  if [ -d "$REPO_DIR" ]; then
    warn "$REPO_DIR existe déjà — réutilisation"
  else
    git clone --recurse-submodules https://github.com/dagornc/SwarmDrones.git "$REPO_DIR"
  fi
  ok "solution clonée dans $REPO_DIR"
else
  [ -d "$REPO_DIR" ] || { warn "$REPO_DIR introuvable (--no-clone)"; exit 1; }
fi

# --- Inventaire --------------------------------------------------------------
say "Inventaire des composants"
ALGOS=$(find "$REPO_DIR/algorithms" -maxdepth 1 -mindepth 1 -type d | sort)
N_ALGOS=$(printf '%s\n' "$ALGOS" | grep -c . || true)
ok "$N_ALGOS algorithmes présents"
printf '%s\n' "$ALGOS" | sed 's#.*/#      - #'

# --- Build + tests + parité --------------------------------------------------
if [ "$HAVE_CARGO" -eq 1 ]; then
  if [ "$QUICK" -eq 1 ]; then
    TARGETS="$REPO_DIR/algorithms/formation-control"
    say "Mode rapide : un seul algorithme"
  else
    TARGETS="$ALGOS"
    say "Compilation, tests et parité sur tous les algorithmes"
  fi

  PASS=0; FAIL=0
  for d in $TARGETS; do
    name=$(basename "$d")
    printf '\n  --- %s ---\n' "$name"
    if [ ! -f "$d/Cargo.toml" ]; then
      warn "$name : pas de Cargo.toml (branche épinglée sans crate ?) — ignoré"
      continue
    fi
    if (cd "$d" && cargo build --release >/tmp/demo_build.log 2>&1); then
      ok "$name : build OK"
    else
      warn "$name : build ÉCHEC"; FAIL=$((FAIL+1)); continue
    fi
    if (cd "$d" && cargo test >/tmp/demo_test.log 2>&1); then
      tp=$(grep -oE '[0-9]+ passed' /tmp/demo_test.log | awk '{s+=$1} END{print s+0}')
      ok "$name : $tp tests passés"
    else
      warn "$name : tests ÉCHEC"; FAIL=$((FAIL+1)); continue
    fi
    if [ -f "$d/verify_parite_rust.py" ]; then
      if (cd "$d" && python3 verify_parite_rust.py >/tmp/demo_parite.log 2>&1); then
        line=$(grep -oE '[0-9]+/[0-9]+ identiques' /tmp/demo_parite.log | tail -1)
        ok "$name : parité ${line:-OK}"
      else
        warn "$name : parité ÉCHEC"; FAIL=$((FAIL+1)); continue
      fi
    fi
    PASS=$((PASS+1))
  done
  say "Bilan : $PASS composants validés, $FAIL en échec"
fi

# --- Points d'entrée publics -------------------------------------------------
say "Points d'entrée publics"
printf '    Modèle d architecture : %s\n' "$SITE"
printf '    Orchestrateur         : https://github.com/dagornc/SwarmDrones\n'
printf '    Un algorithme         : https://github.com/dagornc/alg-formation-control\n'
printf '    Modèle LikeC4         : https://github.com/dagornc/swarmdrones-likec4\n'

say "Démonstration terminée"
