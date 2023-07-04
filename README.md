# Energipris

Kommandolinjeverktøy for å sjekke strømprisene til Tibber visuelt.

> Når du allerede er i terminalen, så er det for mye stress å finne fram en app på mobilen 

### Eksempel:

```
I dag (Pris nå 0.3678)

0.97 ┤                          ╭─────────╮                                         ╭────────────╮
0.91 ┼─╮                     ╭──╯         ╰─╮                                     ╭─╯            ╰───
0.85 ┤ ╰─────────────────────╯              ╰─╮                               ╭───╯
0.79 ┤                                        ╰╮                       ╭──────╯
0.73 ┤                                         ╰╮                  ╭───╯
0.67 ┤                                          │                  │
0.61 ┤                                          ╰╮                ╭╯
0.55 ┤                                           │               ╭╯
0.49 ┤                                           ╰╮             ╭╯
0.43 ┤                                            ╰╮            │
0.37 ┤                                             ╰────────────╯
‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾|
00:00                   06:00                   12:00                   18:00               24:00
```

## Features
- Viser dagens priser
- Viser morgendagens priser
 
## Bruk
For å bruke Tibber APIet, så må du [opprette et API token](https://developer.tibber.com/docs/guides/calling-api).
Dette tokenet legger du så inn i en miljøvariabel på følgende måte:
```
export TIBBER_API_TOKEN=YOUR_TOKEN_FROM_THE_TIBBER_API_PAGE
```

For å vise dagens priser:
```
energipriser --idag
```

For å vise morgendagens priser:
```
energipriser --imorgen
```

