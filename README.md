## Mets à jours des enregistrements DNS Cloudflare pour un réseau avec des addresses dynamiques.

Le fichier record.csv doit suivre ce schéma

```csv
zone_id, record_id
..., ...
```

Ensuite, vous devez avoir les variables d'environnements suivantes:
CF_EMAIL et CF_API_KEY
*Elles correspondent à l'authentification pour [ces endpoints](https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/edit/)*

Enfin, vous pouvez éxecuter le programme à un intervalle de temps fixe.