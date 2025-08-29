# Berichtsheft-Generator

## Beschreibung

---

Ein Berichtsheft-Generator über die Github-CLI und Markdown.

Das Ziel des Projektes ist es, Berichtshefte automatisiert generieren zu lassen. Hierbei zieht das Projekt
verschiedene CLI (*C*ommand *L*ine *I*nterface) Programme wie bspw. **gh-cli**, **git** und **batcat** hinzu,
um die Arbeit im Git sowie die Schulnotizen zu tracken und die notwendigen Daten zu ziehen.

Beim ersten Ausführen des Skripts werden essentielle Programme installiert, dies beinhaltet u.a. die oben genannten
CLI-Programme sowie ein lokales Ollama sowie auf Windows ein WSL mit Ubuntu falls noch keines vorhanden ist.

Dieses Projekt entsteht als Schulprojekt im Fachbereich Anwendungsentwicklung 3. Lehrjahr.

Beginn des Projekts ist am 05.09.25. Abgabe des Projekts ist am 12.12.2025.

Anforderungen an das Projekt:

- Anwendung mit Benutzerschnittstelle
- Projektentwurf mit Präsentation
- Projektplanung (Zeitplanung, Ziele, Kosten, etc)
- Abschlussbericht
  - Soll-Ist Vergleich
  - Zeitliche Auswertung
  - Produktvorstellung (zum dezeitigen Stand)
  - Fazit / Erweiterungsmöglichkeiten
  - (Amortisierungsrechnung)

---

## Anforderungen

---

### Erste Installation

Die Installation läuft über ein eigenes Skript (install.sh).

Während der ersten Installation müssen verschiedene Einstellungen gesetzt werden:

- Git-Anmeldung
- Speicherort der zu trackenden Repositories sowie der Markdown-Dateien des Schulstoffes
- Grundeinstellung der Zeitspanne der Berichte (täglich? wöchentlich? monatlich?)

Darüber hinaus muss ebenfalls eine Initialisierung des Hauptskriptes ausgeführt werden.
Hierfür erfasst das Installationsskript folgende Werte:

- Betriebssystem (Windows, Linux)
  - Wenn OS=Windows: Ist WSL installiert?
- Existenz der dependencies
  - gh-cli
  - git
  - ollama
  - batcat

Nachdem das Installationsskript die Notwendigen Daten erfasst hat, beginnt es mit der
Installation des Hauptskriptes:

- (Bei Windows): Installation und Einrichtung von WSL
- Installation der notwendigen Programme
- Erstellung des individuellen Programmskriptes anhand der erfassten Daten

---

## Programmanforderungen

Geschrieben wird das Projekt in Rust / Shell

Das Programm
