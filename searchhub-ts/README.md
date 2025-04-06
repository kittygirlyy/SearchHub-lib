# searchhub-client

Client non-officiel pour l'API SearchHub.

## Installation

```bash
npm install searchhub-client
```

## Utilisation

```ts
import { searchQuery } from "searchhub-client";

const result = await searchQuery("ton ID", "ta_clé_api");
console.log(result.answer);
```
