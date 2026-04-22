# Test Plan

# Test Plan

Questo documento definisce un piano di test operativo per verificare i mode di
Canon da Copilot Chat su due contesti reali:

- un repository Java esistente
- un repository per un nuovo sistema per il lavoro "Bird"

L'obiettivo non e' testare ogni combinazione teorica, ma validare che i mode
attualmente supportati end-to-end instradino correttamente il lavoro, impongano
il `system_context` quando richiesto, producano gli artifact attesi e falliscano
nei casi sbagliati con guidance corretta.

Questo piano e' chat-first: i casi vanno eseguiti da Copilot Chat usando le
repo-local skills di Canon. La CLI serve solo per inizializzare o aggiornare la
superficie locale, non come formato operativo dei test.

## Scope

### Mode in scope

- `discovery`
- `requirements`
- `system-shaping`
- `architecture`
- `change`
- `review`
- `verification`
- `pr-review`

### Fuori scope

Questi mode restano fuori dal piano perche' non sono ancora chiusi end-to-end
nella slice attuale:

- `implementation`
- `refactor`
- `incident`
- `migration`

## Modello Da Tenere Fisso

### Due assi espliciti

Canon va verificato come modello a due assi:

- `mode`: che tipo di lavoro governato sta avvenendo
- `system_context`: se il sistema target e' nuovo o esistente

Un mode non implica da solo il contesto del sistema. Quando Canon ha bisogno
di binding sul target, il test deve verificare che `system_context` sia
esplicito invece di essere implicito nel nome del mode.

### Regola decisionale rapida

- usare `system-shaping` quando la struttura di una capability non e' ancora
  definita
- usare `change` quando la struttura e' nota e il task e' bounded
  modification con comportamento preservato
- usare `system-shaping` con `system_context = existing` quando si lavora dentro
  un sistema esistente ma il bisogno immediato e' ancora strutturale, non di
  modifica

## Regole Da Verificare

### Regole su `system_context`

- `system-shaping` richiede `new|existing`
- `architecture` richiede `new|existing`
- `change` richiede `existing`
- `discovery`, `requirements`, `review`, `verification`, `pr-review` non devono
  richiedere `system_context`

### Regole su input e namespace

- i mode file-backed devono usare i canonical input sotto `canon-input/`
- le skills non devono inferire input dall'editor attivo, dai tab aperti o da
  contenuti sotto `.canon/`
- `change` deve emettere artifact sotto `.canon/artifacts/<RUN_ID>/change/`
- `review` e' packet-based, non diff-based
- `pr-review` e' diff-based, non packet-based

### Regole di confine tra mode

- `system-shaping` con `system_context = existing` e' il caso ponte per un
  sistema gia' esistente quando la struttura della prossima capability non e'
  ancora fissata
- `change` ha senso solo quando esistono gia' slice, invarianti e change
  surface da preservare
- un nuovo sistema parte da `discovery`, `requirements`, `system-shaping`,
  `architecture`; non da `change`

## Prerequisiti

Una sola volta, fuori da Copilot Chat, inizializzare o aggiornare la superficie
repo-local di Canon per Copilot:

```bash
canon init --ai copilot
canon skills update --ai copilot
```

Verificare poi che siano presenti:

- `.canon/`
- `.agents/skills/`
- il binario `canon` nel `PATH`

Preparare i canonical input path necessari:

- `canon-input/discovery.md`
- `canon-input/requirements.md`
- `canon-input/system-shaping.md`
- `canon-input/architecture.md`
- `canon-input/change.md`
- `canon-input/review.md`
- `canon-input/verification.md`

Per `pr-review`, preparare una branch o un worktree reale con un diff da
ispezionare.

## Formato Prompt In Copilot Chat

Ogni caso di test deve usare questo formato:

1. prima riga: nome della skill
2. seconda riga: testo aggiuntivo con owner, risk, zone, input e, quando serve,
   `system_context`

Esempio minimo:

```text
$canon-requirements
Start a requirements run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/requirements.md.
```

Se Copilot chiede un chiarimento prima di creare il run, rispondere solo al
campo mancante e registrare quella richiesta come parte dell'evidence.

## Evidence Da Raccogliere Per Ogni Run

Per ogni test eseguito, raccogliere almeno:

- il prompt usato in Copilot Chat
- eventuale richiesta di chiarimento fatta da Copilot
- `RUN_ID`
- output di `$canon-status`
- output di `$canon-inspect-artifacts`
- output di `$canon-inspect-evidence` quando applicabile
- contenuto di `.canon/runs/<RUN_ID>/context.toml`
- eventuale messaggio di errore nei test negativi

Prompt di ispezione consigliati:

```text
$canon-status
Show status for run <RUN_ID>.
```

```text
$canon-inspect-artifacts
Inspect artifacts for run <RUN_ID>.
```

```text
$canon-inspect-evidence
Inspect evidence for run <RUN_ID>.
```

## Piano A: Repository Java Esistente

### Obiettivo

Verificare il percorso "existing system", con attenzione particolare a
`system-shaping` con `system_context = existing` come caso ponte, poi
`architecture` con `system_context = existing` e `change` con
`system_context = existing`.

### A1. Discovery su problema poco chiaro

Scopo: verificare che Discovery lavori senza `system_context` e produca un
packet esplorativo.

In Copilot Chat:

```text
$canon-discovery
Start a discovery run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/discovery.md.
```

Verifiche:

- il run completa senza richiedere `system_context`
- `context.toml` non serializza un valore implicito
- sono presenti gli artifact discovery attesi

### A2. Requirements su richiesta Java gia' abbastanza chiara

Scopo: verificare il packet di framing prima del lavoro strutturale o di change.

In Copilot Chat:

```text
$canon-requirements
Start a requirements run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/requirements.md.
```

Verifiche:

- il run completa senza `system_context`
- gli artifact requirements sono presenti
- il packet e' riusabile come upstream per `system-shaping`, `architecture` o
  `change`

### A3. System-shaping su sistema esistente quando la struttura non e' ancora fissata

Scopo: verificare il caso ponte in cui il repository esiste ma il prossimo
bisogno e' ancora strutturale, non di bounded modification.

In Copilot Chat:

```text
$canon-system-shaping
Start a system-shaping run with owner staff-engineer, system context existing, risk bounded-impact, zone yellow, input canon-input/system-shaping.md.
```

Verifiche:

- il mode accetta `system_context = existing`
- `context.toml` contiene `system_context = "existing"`
- gli artifact restano quelli di `system-shaping`, non un packet `change`
- il risultato esplicita capability map, delivery options e risk hotspots
  senza inventare `change-surface.md` o `legacy-invariants.md`

### A4. Architecture su sistema esistente

Scopo: verificare il mode decisionale su repository live.

In Copilot Chat:

```text
$canon-architecture
Start an architecture run with owner staff-engineer, system context existing, risk bounded-impact, zone yellow, input canon-input/architecture.md.
```

Verifiche:

- il mode parte solo con `system_context`
- `context.toml` contiene `system_context = "existing"`
- `status` e inspect espongono il context correttamente
- gli artifact architecture sono presenti

### A5. Change su sistema esistente

Scopo: verificare il bounded change flow sul repo Java quando slice,
invarianti e change surface sono ormai espliciti.

Input richiesto nel brief:

- `System Slice:`
- `Intended Change:`
- `Legacy Invariants:`
- `Change Surface:`
- `Implementation Plan:`
- `Validation Strategy:`
- `Decision Record:`

In Copilot Chat:

```text
$canon-change
Start a change run with owner staff-engineer, system context existing, risk bounded-impact, zone yellow, input canon-input/change.md.
```

Verifiche:

- gli artifact vivono sotto `.canon/artifacts/<RUN_ID>/change/`
- `context.toml` contiene `system_context = "existing"`
- il packet contiene almeno `legacy-invariants.md` e `change-surface.md`
- il run non inventa un change surface se il brief e' debole

### A6. Review del packet system-shaping, change o architecture

Scopo: verificare la review di un packet non-PR.

In Copilot Chat:

```text
$canon-review
Start a review run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/review.md.
```

Verifiche:

- review accetta un packet authored e non un path arbitrario al codice
- `review-disposition.md` e' il risultato principale
- il run puo' essere usato come go/no-go prima dell'implementazione reale

### A7. Verification sugli invariants del caso Java

Scopo: verificare il challenge mode su claim, contract e invarianti.

In Copilot Chat:

```text
$canon-verification
Start a verification run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/verification.md.
```

Verifiche:

- `verification-report.md` e `unresolved-findings.md` sono presenti
- le finding riflettono davvero le prove fornite
- il mode non si comporta come review generica del packet

### A8. PR review su diff reale Java

Scopo: verificare il flusso diff-based.

In Copilot Chat:

```text
$canon-pr-review
Start a pr-review run with owner staff-engineer, risk bounded-impact, zone yellow, base ref refs/heads/main, head ref HEAD.
```

Verifiche:

- il mode lavora su un diff reale
- sono presenti `review-summary.md`, `missing-tests.md`, `contract-drift.md`
- il run non richiede un brief packet-based sotto `canon-input/`

## Piano B: Nuovo Sistema Bird (Da Zero)

### Obiettivo

Verificare il percorso di un nuovo sistema e dimostrare che il mode giusto per
partire non e' `change`, ma `discovery`, `requirements`, `system-shaping` e
poi `architecture`.

Il punto da verificare non e' solo che `change` con `system_context = new`
fallisca, ma che il caso Bird sia proprio fuori modello per `change`: senza un
sistema live e invarianti esistenti da preservare, il lavoro resta nel flusso
di shaping e architecture.

### B1. Discovery su Bird

In Copilot Chat:

```text
$canon-discovery
Start a discovery run with owner staff-engineer, risk low-impact, zone green, input canon-input/discovery.md.
```

Verifiche:

- il run completa senza `system_context`
- il packet discovery esplicita unknowns, boundary e pressure points

### B2. Requirements su Bird

In Copilot Chat:

```text
$canon-requirements
Start a requirements run with owner staff-engineer, risk low-impact, zone green, input canon-input/requirements.md.
```

Verifiche:

- il run completa senza `system_context`
- il packet requirements e' abbastanza chiaro da alimentare `system-shaping`

### B3. System-shaping su nuovo sistema

In Copilot Chat:

```text
$canon-system-shaping
Start a system-shaping run with owner staff-engineer, system context new, risk bounded-impact, zone yellow, input canon-input/system-shaping.md.
```

Verifiche:

- il mode richiede `system_context`
- `context.toml` contiene `system_context = "new"`
- sono presenti `system-shape.md`, `architecture-outline.md`, `capability-map.md`,
  `delivery-options.md`, `risk-hotspots.md`

### B4. Architecture su Bird come nuovo sistema

In Copilot Chat:

```text
$canon-architecture
Start an architecture run with owner staff-engineer, system context new, risk bounded-impact, zone yellow, input canon-input/architecture.md.
```

Verifiche:

- il run completa solo con `system_context`
- inspect e status mostrano `new`
- il packet architecture e' coerente con un sistema ancora da costruire

### B5. Review del packet system-shaping o architecture

In Copilot Chat:

```text
$canon-review
Start a review run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/review.md.
```

Verifiche:

- il packet e' reviewable senza essere un diff
- `review-disposition.md` e' leggibile come output principale

### B6. Verification su claim Bird

In Copilot Chat:

```text
$canon-verification
Start a verification run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/verification.md.
```

Verifiche:

- il mode challenge-a claims e invariants del sistema nuovo
- le unresolved findings sono coerenti con l'evidence basis fornita

### B7. PR review su primo diff Bird

In Copilot Chat:

```text
$canon-pr-review
Start a pr-review run with owner staff-engineer, risk bounded-impact, zone yellow, base ref refs/heads/main, head ref HEAD.
```

Verifiche:

- review del diff reale
- findings, missing tests e decision impact sono presenti

## Test Negativi Obbligatori

Nei test negativi, il comportamento corretto in chat puo' essere uno dei due:

- rifiuto prima della creazione del run, con guidance esplicita
- richiesta mirata del solo campo mancante o invalido

Se Copilot crea un run nonostante il vincolo sia violato, il test e' fallito.

### N1. Architecture senza `system_context`

In Copilot Chat:

```text
$canon-architecture
Start an architecture run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/architecture.md.
```

Atteso:

- nessun run viene creato finche' non si specifica `new|existing`
- Copilot chiede solo il `system_context` mancante oppure rifiuta con guidance
  corretta

### N2. System-shaping senza `system_context`

In Copilot Chat:

```text
$canon-system-shaping
Start a system-shaping run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/system-shaping.md.
```

Atteso:

- nessun run viene creato finche' non si specifica `new|existing`
- Copilot chiede solo il `system_context` mancante oppure rifiuta prima del run

### N3. Change senza `system_context`

In Copilot Chat:

```text
$canon-change
Start a change run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/change.md.
```

Atteso:

- nessun run viene creato finche' non si specifica `existing`
- la guidance chiarisce che `change` richiede `system_context = existing`

### N4. Change con `system_context = new`

In Copilot Chat:

```text
$canon-change
Start a change run with owner staff-engineer, system context new, risk bounded-impact, zone yellow, input canon-input/change.md.
```

Atteso:

- nessun run valido viene creato con `new`
- Copilot spiega che `change` supporta solo `existing`
- la guidance chiarisce che `change` presuppone un sistema live e invarianti
  gia' esistenti da preservare
- e' accettabile anche un redirect esplicito verso `system-shaping` o
  `architecture`, ma non un cambio silenzioso di mode

### N5. Review puntato al codice invece che a un packet

In Copilot Chat:

```text
$canon-review
Start a review run with owner staff-engineer, risk bounded-impact, zone yellow, input src/.
```

Atteso:

- fallimento o guidance che indica input packet-based corretto
- Copilot non deve reinterpretare automaticamente `src/` come diff review o
  review del codice libera

### N6. PR review senza diff reale

In Copilot Chat:

```text
$canon-pr-review
Start a pr-review run with owner staff-engineer, risk bounded-impact, zone yellow.
```

Atteso:

- fallimento o guidance che richiede ref validi oppure `WORKTREE`
- Copilot non deve trasformare `pr-review` in una review packet-based

## Checklist Trasversale Di Verifica

Per ogni run riuscito, verificare:

1. la skill usata e il mode in `status` corrispondono al run richiesto
2. il prompt ha reso espliciti owner, risk, zone, input e, quando richiesto,
   `system_context`
3. il namespace artifact e' corretto
4. `context.toml` contiene `system_context` solo quando fornito
5. `inspect artifacts` e `inspect evidence` sono coerenti col run
6. non ci sono default impliciti per i mode optional-context
7. non ci sono binding impliciti da editor attivo, tab aperti o file sotto
   `.canon/`
8. gli artifact riflettono il tipo di lavoro: `system-shaping` resta
   strutturale, `change` esplicita invarianti e change surface

Prompt di supporto consigliati:

```text
$canon-status
Show status for run <RUN_ID>.
```

```text
$canon-inspect-artifacts
Inspect artifacts for run <RUN_ID>.
```

```text
$canon-inspect-evidence
Inspect evidence for run <RUN_ID>.
```

## Ordine Consigliato Di Esecuzione

### Repository Java

1. `discovery`
2. `requirements`
3. `system-shaping` con `system_context = existing`
4. `architecture` con `system_context = existing`
5. `change` con `system_context = existing`
6. `review`
7. `verification`
8. `pr-review`

### Repository Bird

1. `discovery`
2. `requirements`
3. `system-shaping` con `system_context = new`
4. `architecture` con `system_context = new`
5. `review`
6. `verification`
7. `pr-review`

### Negative tests finali

1. `architecture` senza context
2. `system-shaping` senza context
3. `change` senza context
4. `change` con `system_context = new`
5. `review` con input sbagliato
6. `pr-review` senza target reale

## Criteri Di Uscita

Il piano e' soddisfatto se:

1. tutti i mode in scope sono stati eseguiti almeno una volta nel contesto piu'
   corretto
2. tutti i failure path obbligatori sono stati verificati in Copilot Chat
3. `system-shaping` passa con `existing` sul repo Java quando il bisogno e'
   ancora strutturale e passa con `new` sul percorso Bird
4. `change` passa sul repo Java solo con `existing` e con slice, invarianti e
   change surface espliciti; sul percorso Bird con `new` viene rifiutato oppure
   reindirizzato esplicitamente, mai eseguito in silenzio
5. `system-shaping` e `architecture` richiedono sempre `system_context`
6. `review` lavora su packet e `pr-review` lavora su diff reali
7. summary, inspect surface e messaggi di chiarimento riflettono correttamente
   mode, artifact e context

## Estensioni Future

Quando `implementation`, `refactor`, `incident` e `migration` saranno promossi a
end-to-end depth, questo piano dovra' essere esteso con:

- test su controlled mutation
- test su rollback e completion evidence
- test su containment, compatibility e sequencing
- test su approval gating dei mode execution-heavy e high-risk
