# Berichtsheft Generator

## Projektüberblick

Der Berichtsheft Generator ist ein professionelles, in Rust entwickeltes CLI-Tool, das darauf abzielt, die mühsame manuelle Erstellung von Berichtshefteinträgen zu automatisieren. Durch die Integration mit Git, der GitHub CLI (gh-cli) und lokalen Ollama-LLMs analysiert das Tool automatisch die täglichen, wöchentlichen oder monatlichen Aktivitäten eines Benutzers auf GitHub und in lokalen Markdown-Dateien.

---

## Funktionsweise

Das Tool extrahiert relevante Informationen aus Commits, Pull-Requests und anderen GitHub-Aktivitäten sowie aus strukturierten Notizen in Markdown-Dateien. Mithilfe eines lokalen Large Language Models (LLM), das über Ollama ausgeführt wird, werden diese gesammelten Daten intelligent zusammengefasst und in einen kohärenten, gut formatierten Berichtshefteintrag umgewandelt. Der Benutzer kann den gewünschten Analysezeitraum (Tag, Woche, Monat) oder ein spezifisches Datum für die Generierung festlegen.

---

## Kern-Features

_Automatisierte Datenerfassung_: Scannt GitHub-Aktivitäten und lokale Markdown-Dateien.

_Lokale KI-Integration_: Nutzt ein lokales Ollama-LLM für die Zusammenfassung, um Datenschutz und Unabhängigkeit von externen Diensten zu gewährleisten.

_Flexible Zeiträume_: Anpassbare Analyse für tägliche, wöchentliche oder monatliche Berichte.

_Einfache Handhabung_: Das CLI-Format ermöglicht eine schnelle und effiziente Nutzung direkt über das Terminal.

---

## Initialisierung & Bereitstellung

Um eine reibungslose Inbetriebnahme zu gewährleisten, beinhaltet das Projekt ein Initialisierungsskript. Dieses Skript erkennt das Betriebssystem des Benutzers und installiert automatisch alle notwendigen Abhängigkeiten wie git, gh-cli und, falls unter Windows erforderlich, das Windows Subsystem for Linux (WSL). Nach der Installation der Tools wird das Hauptprogramm kompiliert und bereitgestellt, sodass es sofort einsatzbereit ist. Diese Vorgehensweise stellt sicher, dass wir das Tool ohne komplizierte manuelle Konfigurationen nutzen können.

---

## Projektplan für den Berichtsheft Generator

### 1. Projektziele

Das primäre Ziel ist die Entwicklung eines zuverlässigen, automatisierten und plattformübergreifenden CLI-Tools in Rust, das wir zur effizienten Erstellung von Berichtshefteinträgen nutzen können. Das Tool soll manuelle Arbeitsschritte minimieren und die Genauigkeit der Berichte durch die Analyse von Git-Aktivitäten und Markdown-Notizen maximieren.

### 2. Technische Anforderungen & Tools

_Programmiersprache_: Rust wird für die Entwicklung verwendet, da es eine hohe Performance, Sicherheit und Plattformunabhängigkeit bietet.

_Versionskontrolle_: Git wird zur Quellcode-Verwaltung genutzt.

_Abhängigkeiten_:

- git2-rs oder gitoxide: Für die Interaktion mit lokalen Git-Repositories, um Commits und Metadaten zu analysieren.

- reqwest: Zum Senden von HTTP-Anfragen an die GitHub API, um Pull Requests, Issues und andere Aktivitäten zu erfassen.

- serde & serde_json: Für die Deserialisierung der JSON-Antworten von der GitHub API.

- tokio: Für asynchrone Operationen, um API-Aufrufe und Dateisystem-Operationen effizient zu handhaben.

- ollama-rs (oder vergleichbare Crate): Für die Kommunikation mit der lokalen Ollama-Instanz und dem LLM.

- clap: Für die Erstellung des CLI, zur Definition von Befehlen und Argumenten (z. B. --period <tag|woche|monat>).

- dirs: Zum Auffinden von plattformspezifischen Verzeichnissen.

- walkdir: Zur effizienten Rekursion durch das Dateisystem.

_Shell-Skripte (Bash/PowerShell)_: Für das Initialisierungsskript, das die Installation von Abhängigkeiten basierend auf dem Betriebssystem (Linux, macOS, Windows) übernimmt.

### 3. Projektstruktur

Die Projektstruktur soll klar und modular sein, um die Wartbarkeit zu erleichtern.

```
/berichsheft-generator
├── /src
│ ├── /cli # Modul für die CLI-Logik (`main.rs`)
│ ├── /git_analyzer # Modul zur Git-Analyse
│ ├── /github_api # Modul für die GitHub API-Interaktion
│ ├── /markdown_parser # Modul zur Analyse von Markdown-Dateien
│ ├── /ollama_api # Modul zur Kommunikation mit Ollama
│ ├── /report_generator # Modul zur Generierung des fertigen Berichts
│ └── main.rs
├── /scripts
│ ├── init.sh # Initialisierungsskript für Linux/macOS
│ └── init.ps1 # Initialisierungsskript für Windows
├── Cargo.toml # Rust-Projektdaten
└── README.md
```

### 4. Umsetzungsphasen

Die Entwicklung erfolgt in mehreren Phasen, um den Fortschritt kontinuierlich zu überprüfen.

**Phase 1**: Konzeption & Prototyping (Woche 1-2)

- Auswahl der endgültigen Crates.

- Entwurf der API-Modelle für GitHub und Ollama.

- Erstellung eines einfachen CLI-Prototyps, der grundlegende Befehle verarbeitet.

- Erstellung eines Prototyps für die Git-Analyse.

**Phase 2**: Kernfunktionalität (Woche 3-6)

- Implementierung der Git-Analyse (Commits, Branches).

- Implementierung der GitHub API-Interaktion (Pull Requests, Issues).

- Implementierung der Markdown-Dateien-Analyse.

- Integration der Ollama-API zur Verarbeitung der gesammelten Daten.

- Entwicklung der Logik zur Generierung des Berichtseintrags.

**Phase 3**: Initialisierungsskript & Verpackung (Woche 7-8)

- Erstellung des plattformübergreifenden Initialisierungsskripts (.sh und .ps1).

- Das Skript muss das Betriebssystem erkennen, Abhängigkeiten (Git, gh-cli, WSL) überprüfen und installieren.

- Das Skript führt cargo build --release aus und legt das fertige Binary an einem zugänglichen Ort ab.

- Entwurf eines Beispiel-Arbeitsablaufs und Integration in die Dokumentation.

**Phase 4**: Tests & Veröffentlichung (Woche 9-10)

- Gründliche Tests der Funktionalität und des Initialisierungsskripts auf Linux, macOS und Windows.

- Testen verschiedener LLM-Modelle.

- Schreiben einer detaillierten README-Datei mit Installationsanleitung und Nutzungsbeispielen.

- Erstellung von Git-Tags und Versions-Releases.
