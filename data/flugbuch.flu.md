```text
4
```

- The first line of the file contains the data format version

```text
[GenSettings]Tobias Bieniek;Baumallee 14;12345 Hamburg;Bieniek, Tobias;
```

- `Name`
- `Road` 
- `Location`
- `Pilot Name`

```text
:Segelflug;Segelflug
```

- `:` character starts a new flight log
- Only the first field seems to be read by the original software.

```text
[LicSettings]0;00000:00;  .  .    ;15.11.2004;D-;0;00000:00;0;km;
```

- 1: `BFStarts`
- 2: `BFTime`
- 4: `LicenseSince`
- 5: `IDPrefix`
- 9: `DistUnit`

Everything below until `TableCols` can be in random order.

Additional lists that are not part of the example file:

- `LicenseCat` (LicenseCategories)
- `LicenseTimeCat`
- `LicenseDates`
- `AccLicenses`
- `OptConditions`


```text
[AId]D-4449;D-9041;D-8784;D-0030;D-KCSS;D-4865;D-5074;D-1896;D-KAIM;D-0816;D-2914;D-KESH;D-KARM;D-4708;D-4462;
```

- AutoComplete IDs

```text
[AType]Hornet-Libelle;Duo Discus;ASK 21;DG 1000;ASG 29E;ASK 21;ASK 21;LS 8;ASH 25E;LS 6;Twin Astir II;ASK 21 Mi;DG 1000;ASK 21;Hornet;
```

- AutoComplete aircraft types

```text
[CoPilot]Mustermann, Max;Mustermann, Martin;Hampelmann, Heiner;
```

- AutoComplete copilots

```text
[Loc]Meiersberg;Hilden;Langenfeld Wiesch;Außenlandung;EDRS;St. Auban;Stölln Rhinow;EDKA;Puimoisson;EDKL;Düren-Hürtgenwald;Sainte Croix;
```

- AutoComplete airfields

```text
[ColWidth]30;62;62;44;82;92;21;35;34;35;13;85;88;251;46;88;9;6;4;96;
```

- Column widths

```text
[CatTime]
```

- Time categories

```text
[Category]Schulflug;Einweisung;Gastflug;Wettbewerb;Gebirge;Kunstflug;Checkflug;FI;Auffrischungsschulung;
```

- Categories

```text
[Contest]OLC;Hangelar;DMSt;
```

- Content categories

```text
[TableCols]Num;Dat;Mus;Ken;Pil;Beg;Art;StZ;LaZ;FlZ;AFl;StO;LaO;Bem;Str;Kat;Via;CTi;Fil;Con;
```

Conversions:

- `Mus` -> `ATy`
- `Ken` -> `AId`
- `Pil` -> `Pi1`
- `Beg` -> `Pi2`
- `Art` -> `ToS`
- `StZ` -> `StT`
- `LaZ` -> `LaT`
- `FlZ` -> `FlT`
- `BlZ` -> `BlT`
- `AFl` -> `NoL`
- `StO` -> `StL`
- `LaO` -> `LaL`
- `Bem` -> `Rem`
- `Str` -> `Dst`
- `Kat` -> `Cat`

```text
1;20.04.2002;ASK 13;D-2326;Bieniek, Tobias;Mustermann, Max;W;11:40;11:43;0:03;1;Meiersberg;Meiersberg;;;Schulflug/;;;;;
2;20.04.2002;ASK 13;D-2326;Bieniek, Tobias;Mustermann, Max;W;12:15;12:18;0:03;1;Meiersberg;Meiersberg;;;Schulflug/;;;;;
472;08.06.2007;Discus 2t;D-KSFN;Bieniek, Tobias;;F;10:25;15:11;4:46;1;Meiersberg;Meiersberg;;326;;mengeringhausen/Siegen Eisernhard/;;;;
1021;07.08.2017;Hornet-Libelle;D-4449;Bieniek, Tobias;;F;10:40;16:03;5:23;1;Puimoisson;Puimoisson;;;Gebirge/;;;;;
1237;15.09.2019;ASK 21 Mi;D-KESH;Bieniek, Tobias;+1;F;13:00;13:21;0:21;1;EDKA;EDKA;;;Gastflug/;;;;;
1336;20.09.2020;ASK 21;D-4708;Bieniek, Tobias;Hampelmann, Heiner;W;14:14;14:18;0:04;1;Düren-Hürtgenwald;Düren-Hürtgenwald;;;FI/;;;;;
:Motorsegler;Motorsegler
[LicSettings]0;00000:00;  .  .    ;19.07.2007;D-K;0;00000:00;0;km;
[AId]D-KGTI;D-KGTC;D-KABH;D-KOEA;
[AType]Super Dimona;Super Dimona;RF 4;Tandem Falke;
[CoPilot]Mustermann, Martin;Hampelmann, Heiner;
[Loc]Meiersberg;
[ColWidth]30;62;44;50;60;55;42;42;30;30;85;88;80;46;88;68;119;53;96;
[CatTime]Motorlaufzeit;
[Category]Schulflug;Einweisung;Übungsflug m Fl;
[Contest]
[TableCols]Num;Dat;Mus;Ken;Pil;Beg;StZ;LaZ;FlZ;AFl;StO;LaO;Bem;Str;Kat;Via;CTi;Fil;Con;
1;18.09.2004;Super Dimona;D-KGTI;Bieniek, Tobias;Mustermann, Martin;15:52;16:17;0:25;1;Meiersberg;Meiersberg;;;Schulflug/;;;;;
2;04.10.2004;Super Dimona;D-KGTI;Bieniek, Tobias;Mustermann, Martin;16:50;17:07;0:17;3;Meiersberg;Meiersberg;;;Schulflug/;;;;;
32;18.07.2007;Super Dimona;D-KGTI;Bieniek, Tobias;Hampelmann, Heiner;12:56;13:19;0:23;1;Meiersberg;Meiersberg;Prüfung;;Schulflug/Übungsflug m Fl/;;;;;
57;07.03.2010;Tandem Falke;D-KOEA;Bieniek, Tobias;;15:26;15:46;0:20;1;Meiersberg;Meiersberg;;;;;;;;
```