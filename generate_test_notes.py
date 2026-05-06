#!/usr/bin/env python3
"""
Génère 10 notes markdown de test dans <repo>/notes.
Chaque note contient du lorem ipsum + quelques mots-clés plantés
pour valider la recherche (littérale, casse, caractères spéciaux).
"""

from pathlib import Path
import random
import textwrap

OUTPUT_DIR = Path(__file__).resolve().parent / "notes"
NOTE_COUNT = 10

LOREM = (
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod "
    "tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, "
    "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo "
    "consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse "
    "cillum dolore eu fugiat nulla pariatur."
).split()

# Mots-clés plantés volontairement pour tester la recherche.
# Chacun teste un cas précis :
PLANTED = [
    "Tauri",                  # mot simple, casse normale
    "TAURI",                  # même mot, casse différente -> teste case-sensitive
    "rust-lang",              # tiret
    "C++",                    # caractères spéciaux
    "café",                   # accent
    "hello world",            # espace dans la requête
    "TODO:",                  # ponctuation
    "v2.0.1",                 # version avec points
    "naïve",                  # tréma
    "fuzzy_match",            # underscore
]


def make_note(idx: int) -> tuple[str, str]:
    """Retourne (filename, content) pour la note idx."""
    title = f"Note {idx:02d}"
    filename = f"note-{idx:02d}.md"

    # Corps : 3-5 paragraphes de lorem
    paragraphs = []
    for _ in range(random.randint(3, 5)):
        words = random.choices(LOREM, k=random.randint(30, 60))
        paragraphs.append(" ".join(words).capitalize() + ".")

    # Plante 1 à 3 mots-clés à des positions aléatoires
    keywords = random.sample(PLANTED, k=random.randint(1, 3))
    for kw in keywords:
        target = random.randint(0, len(paragraphs) - 1)
        # On insère le mot-clé au milieu d'un paragraphe
        words = paragraphs[target].split()
        pos = random.randint(0, len(words))
        words.insert(pos, kw)
        paragraphs[target] = " ".join(words)

    content = f"# {title}\n\n" + "\n\n".join(paragraphs) + "\n"
    return filename, content


def main() -> None:
    random.seed(42)  # déterministe, utile pour debug
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    for i in range(1, NOTE_COUNT + 1):
        filename, content = make_note(i)
        path = OUTPUT_DIR / filename
        path.write_text(content, encoding="utf-8")
        print(f"  wrote {path}")

    # Récap des mots-clés pour que tu saches quoi chercher
    print(f"\n✓ {NOTE_COUNT} notes générées dans {OUTPUT_DIR}")
    print("\nMots-clés plantés (à tester dans la recherche) :")
    for kw in PLANTED:
        print(f"  - {kw!r}")


if __name__ == "__main__":
    main()
