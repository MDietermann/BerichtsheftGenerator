# 📋 Berichtsheft Generator

**Ein intelligentes CLI-Tool zur automatisierten Generierung von Berichtshefteinträgen**

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)
![Version](https://img.shields.io/badge/Version-0.1.0-green?style=for-the-badge)

---

## 🎯 Projektüberblick

Der **Berichtsheft Generator** ist ein professionelles, in Rust entwickeltes CLI-Tool, das die mühsame manuelle Erstellung von Berichtshefteinträgen automatisiert. Durch die intelligente Integration mit Git, GitHub API und lokalen Ollama-LLMs analysiert das Tool automatisch Ihre täglichen, wöchentlichen oder monatlichen Entwicklungsaktivitäten und generiert strukturierte, professionelle Berichte.

### 🚀 Kernfeatures

- **🔍 Automatisierte Datenerfassung**: Scannt Git-Commits, GitHub-Aktivitäten und lokale Markdown-Dateien
- **🤖 Lokale KI-Integration**: Nutzt Ollama-LLM für intelligente Zusammenfassungen (100% datenschutzkonform)
- **📅 Flexible Zeiträume**: Anpassbare Analyse für tägliche, wöchentliche oder monatliche Berichte
- **🖥️ Plattformübergreifend**: Unterstützt Windows (mit WSL), Linux und macOS
- **⚡ Performant**: Geschrieben in Rust für maximale Geschwindigkeit und Sicherheit
- **🎨 Benutzerfreundlich**: Intuitive CLI mit interaktiven Eingabeaufforderungen

---

## 📦 Installation

### Voraussetzungen

Das Tool installiert automatisch alle erforderlichen Abhängigkeiten, einschließlich:
- Git
- GitHub CLI (gh)
- Ollama (für lokale LLM-Verarbeitung)
- WSL (nur Windows)

### Schnellstart

1. **Repository klonen**:
   ```bash
   git clone https://github.com/yourusername/BerichtsheftGenerator.git
   cd BerichtsheftGenerator
   ```

2. **Installation ausführen**:
   ```bash
   # Linux/macOS
   ./scripts/init.sh
   
   # Windows (PowerShell als Administrator)
   .\scripts\init.ps1
   ```

3. **Tool verwenden**:
   ```bash
   ./target/release/b-gen install
   ./target/release/b-gen generate-commits
   ```

---

## 🛠️ Verwendung

### Erste Konfiguration

Beim ersten Start führt Sie das Tool durch die Einrichtung:

```bash
./b-gen install
```

Dabei werden folgende Daten erfasst:
- Git-Benutzername und GitHub-Token
- Pfad zu Ihren Projekten/Repositories
- Bevorzugte Berichtszeiträume

### Berichte generieren

```bash
# Commits aller Repositories scannen
./b-gen generate-commits

# Interaktive Auswahl einzelner Repositories
./b-gen generate-commits --interactive
```

### Konfiguration über Umgebungsvariablen

Erstellen Sie eine `.env`-Datei:

```env
GITHUB_TOKEN=your_github_token_here
PROJECT_PATH=/path/to/your/projects
REPORT_PERIOD=weekly
```

---

## 🏗️ Projektstruktur

```
BerichtsheftGenerator/
├── src/
│   ├── cli/                 # CLI-Interface und Argument-Parsing
│   ├── helper/              # Hilfsfunktionen und Utilities
│   │   ├── custom_cli.rs    # Benutzerdefinierte CLI-Interaktionen
│   │   └── user_data.rs     # Benutzerdatenverwaltung
│   ├── report_generator/    # Berichtsgenerierung und Ausgabe
│   └── main.rs              # Haupteinstiegspunkt
├── scripts/
│   ├── init.sh             # Linux/macOS Installationsskript
│   └── init.ps1            # Windows PowerShell Installationsskript
├── Cargo.toml              # Rust Projektkonfiguration
├── .env                    # Umgebungsvariablen (nicht im Repository)
└── README.md
```

---

## 🔧 Technische Details

### Verwendete Dependencies

| Crate | Version | Zweck |
|-------|---------|-------|
| `clap` | 4.5.46 | CLI-Argument-Parsing mit Derive-Features |
| `tokio` | 1.47.1 | Asynchrone Runtime für API-Calls |
| `serde` | 1.0.219 | JSON-Serialisierung/-Deserialisierung |
| `octocrab` | 0.44.1 | GitHub API-Integration |
| `chrono` | 0.4.41 | Datum- und Zeitverarbeitung |
| `walkdir` | 2.5.0 | Rekursive Dateisystem-Navigation |
| `dotenv` | 0.15.0 | Umgebungsvariablen-Management |

### Architektur

Das Tool folgt einer modularen Architektur:

1. **CLI-Interface**: Verarbeitet Benutzereingaben und Parameter
2. **Datensammlung**: Scannt Git-Repositories und GitHub-Aktivitäten
3. **Datenverarbeitung**: Aggregiert und strukturiert gesammelte Informationen
4. **KI-Integration**: Nutzt lokale Ollama-LLMs für intelligente Zusammenfassungen
5. **Berichtsgenerierung**: Erstellt formatierte Berichte in verschiedenen Formaten

---

## 🎓 Schulprojekt-Kontext

**Entwicklungszeit**: September 2025 - Dezember 2025  
**Fachbereich**: Anwendungsentwicklung, 3. Lehrjahr  
**Projekttyp**: Abschlussprojekt mit umfassender Dokumentation

### Projektanforderungen

- ✅ Anwendung mit Benutzerschnittstelle
- ✅ Detaillierter Projektentwurf mit Präsentation
- ✅ Umfassende Projektplanung (Zeit, Ziele, Kosten)
- ✅ Ausführlicher Abschlussbericht mit Soll-Ist-Vergleich

### Entwicklungsphasen

| Phase | Zeitraum | Schwerpunkt |
|-------|----------|-------------|
| **Phase 1** | Woche 1-2 | Konzeption & Prototyping |
| **Phase 2** | Woche 3-8 | Kernfunktionalität & Integration |
| **Phase 3** | Woche 9-10 | Testing & Optimierung |
| **Phase 4** | Woche 11-12 | Dokumentation & Präsentation |

---

## 🚀 Funktionsweise

Das Tool arbeitet in mehreren Schritten:

1. **Initialisierung**: Automatische Installation aller Dependencies
2. **Konfiguration**: Erfassung von Benutzereinstellungen und Zugangsdaten
3. **Datensammlung**: 
   - Scanning lokaler Git-Repositories
   - Abrufen von GitHub-Aktivitäten über die API
   - Analyse von Markdown-Notizen und Dokumentation
4. **Datenverarbeitung**: Intelligente Filterung und Strukturierung
5. **KI-Zusammenfassung**: Lokale Verarbeitung durch Ollama-LLM
6. **Berichtsgenerierung**: Erstellung formatierter Berichtshefteinträge

---

## 🔒 Datenschutz & Sicherheit

- **Lokale Verarbeitung**: Alle sensiblen Daten bleiben auf Ihrem System
- **Keine Cloud-Dependencies**: Ollama läuft komplett lokal
- **Sichere Token-Handhabung**: GitHub-Tokens werden verschlüsselt gespeichert
- **Konfigurierbare Datenerfassung**: Vollständige Kontrolle über gesammelte Daten

---

## 🤝 Mitwirken

Dieses Projekt ist Teil eines Schulprojekts, aber Feedback und Verbesserungsvorschläge sind willkommen!

### Entwicklung

```bash
# Projekt für Entwicklung einrichten
git clone https://github.com/yourusername/BerichtsheftGenerator.git
cd BerichtsheftGenerator

# Dependencies installieren und Build
cargo build

# Tests ausführen
cargo test

# Entwicklungsversion ausführen
cargo run -- --help
```

### Code-Stil

Das Projekt folgt den Standard-Rust-Konventionen:
- `cargo fmt` für Formatierung
- `cargo clippy` für Linting
- Ausführliche Dokumentation mit `cargo doc`

---

## 🐛 Bekannte Probleme & Roadmap

### Aktueller Status
- ✅ Grundlegende CLI-Struktur
- ✅ Git-Repository-Scanning
- ✅ Installationsskripte für Linux/Windows
- 🔄 GitHub API-Integration (in Entwicklung)
- 🔄 Ollama-LLM-Integration (geplant)
- 🔄 Berichtsgenerierung (geplant)

### Geplante Features
- [ ] Unterstützung für GitLab und andere Git-Hosting-Services
- [ ] Export in verschiedene Formate (PDF, Word, HTML)
- [ ] Konfigurierbare Berichts-Templates
- [ ] Web-Interface für erweiterte Konfiguration
- [ ] Automatische Zeiterfassung mit Kalendar-Integration

---

## 📄 Lizenz

Dieses Projekt steht unter der MIT-Lizenz. Siehe [LICENSE](LICENSE) für Details.

---

## 📞 Kontakt

**Entwickler**: [Ihr Name]  
**E-Mail**: [ihre.email@example.com]  
**Projekt-Repository**: [GitHub-Link]

---

<p align="center">
  <i>🎯 Automatisieren Sie Ihre Berichtshefterstellung mit der Power von Rust und KI!</i>
</p>
