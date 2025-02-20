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

## Mise en place
1. Compiler le programme
2. `nano /etc/dyndns/records.csv` remplir avec vos enregistrements DNS (voir [Cloudflare API](https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/list/))
3. crontab -e
4. `0 2 * * * cd /etc/dyndns && CF_EMAIL=xxxx CF_API_KEY=xxxx cf-dyndns` avec cf-dyndns étant le programme compilé localisé à `/usr/local/bin`

Ainsi, le programme sera éxecuté chaque jour à 2 AM assurant que l'ip vers laquelle pointe les enregistrements soit bien à jour.

*Ce programme peut devenir obselète si l'api [ipify](https://www.ipify.org) devient indisponible*
